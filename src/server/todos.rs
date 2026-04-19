use leptos::prelude::use_context;
use serde::Deserialize;
use worker::D1Type;

use crate::api::{TodoItem, TodoStats, TodosResponse};

use super::{AppError, AppResult, AppState};

const MAX_VISIBLE_TODOS: usize = 100;
const MAX_TODOS_PER_SESSION: usize = 200;
const MAX_TODOS_GLOBAL: usize = 5_000;

#[derive(Debug, Deserialize)]
struct TodoRow {
    id: i64,
    title: String,
    completed: i64,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct TodoStatsRow {
    total: i64,
    open: i64,
    completed: i64,
}

#[derive(Debug, Deserialize)]
struct CountRow {
    total: i64,
}

pub async fn list_todos() -> AppResult<TodosResponse> {
    let db = database()?;
    let session_id = session_id()?;
    let session_arg = D1Type::Text(session_id.as_str());
    let visible_limit_arg =
        d1_limit_arg(MAX_VISIBLE_TODOS, "Visible todo limit exceeded D1 range.")?;
    let result = db
        .prepare(
            "SELECT
                id,
                title,
                completed,
                strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at
             FROM todos
             WHERE session_id = ?1
             ORDER BY completed ASC, id DESC
             LIMIT ?2",
        )
        .bind_refs([&session_arg, &visible_limit_arg])
        .map_err(|error| d1_error("Failed to bind todo list query.", error))?
        .all()
        .await
        .map_err(|error| d1_error("Failed to list todos from D1.", error))?;

    let items = result
        .results::<TodoRow>()
        .map_err(|error| d1_error("Failed to deserialize todo rows from D1.", error))?
        .into_iter()
        .map(map_todo)
        .collect::<Vec<_>>();

    let stats = load_todo_stats(&db, session_id.as_str()).await?;

    Ok(TodosResponse {
        is_truncated: stats.total > items.len(),
        items,
        stats,
        visible_limit: MAX_VISIBLE_TODOS,
    })
}

pub async fn create_todo(title: String) -> AppResult<TodoItem> {
    let db = database()?;
    let session_id = session_id()?;
    let title = normalize_title(title)?;

    if count_session_todos(&db, session_id.as_str()).await? >= MAX_TODOS_PER_SESSION {
        return Err(AppError::client(format!(
            "This demo stores up to {MAX_TODOS_PER_SESSION} todos per browser session. Delete a few to keep going."
        )));
    }

    trim_global_todos(&db).await?;

    let session_arg = D1Type::Text(session_id.as_str());
    let title_arg = D1Type::Text(title.as_str());
    let args = [&session_arg, &title_arg];

    let result = db
        .prepare("INSERT INTO todos (session_id, title) VALUES (?1, ?2)")
        .bind_refs(args)
        .map_err(|error| d1_error("Failed to bind todo insert query.", error))?
        .run()
        .await
        .map_err(|error| d1_error("Failed to insert todo into D1.", error))?;

    let inserted_id = result
        .meta()
        .map_err(|error| d1_error("Failed to inspect D1 insert metadata.", error))?
        .and_then(|meta| meta.last_row_id)
        .ok_or_else(|| {
            AppError::internal(
                "D1 insert completed without returning last_row_id.",
                "missing last_row_id metadata",
            )
        })?;

    get_todo_by_id(&db, inserted_id, session_id.as_str()).await
}

pub async fn toggle_todo(id: i64) -> AppResult<TodoItem> {
    let db = database()?;
    let session_id = session_id()?;
    let id_arg = todo_id_arg(id)?;
    let session_arg = D1Type::Text(session_id.as_str());
    let args = [&id_arg, &session_arg];

    let result = db
        .prepare(
            "UPDATE todos
             SET completed = CASE completed WHEN 0 THEN 1 ELSE 0 END
             WHERE id = ?1 AND session_id = ?2",
        )
        .bind_refs(args)
        .map_err(|error| d1_error("Failed to bind todo toggle query.", error))?
        .run()
        .await
        .map_err(|error| d1_error("Failed to toggle todo in D1.", error))?;

    ensure_row_changed(result, "toggle")?;
    get_todo_by_id(&db, id, session_id.as_str()).await
}

pub async fn delete_todo(id: i64) -> AppResult<()> {
    let db = database()?;
    let session_id = session_id()?;
    let id_arg = todo_id_arg(id)?;
    let session_arg = D1Type::Text(session_id.as_str());
    let args = [&id_arg, &session_arg];

    let result = db
        .prepare("DELETE FROM todos WHERE id = ?1 AND session_id = ?2")
        .bind_refs(args)
        .map_err(|error| d1_error("Failed to bind todo delete query.", error))?
        .run()
        .await
        .map_err(|error| d1_error("Failed to delete todo from D1.", error))?;

    ensure_row_changed(result, "delete")
}

fn database() -> AppResult<worker::D1Database> {
    app_state()?
        .db()
        .map_err(|error| AppError::internal("Failed to access D1 binding from app state.", error))
}

fn app_state() -> AppResult<AppState> {
    use_context::<AppState>().ok_or_else(|| {
        AppError::internal(
            "Missing app state in Leptos server function context.",
            "state was not provided to the request",
        )
    })
}

fn session_id() -> AppResult<String> {
    Ok(app_state()?.session_id().to_string())
}

fn normalize_title(title: String) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::client("Todo titles cannot be empty."));
    }

    if trimmed.len() > 120 {
        return Err(AppError::client(
            "Todo titles are capped at 120 characters.",
        ));
    }

    Ok(trimmed.to_string())
}

fn todo_id_arg(id: i64) -> AppResult<D1Type<'static>> {
    let id = i32::try_from(id).map_err(|_| AppError::client("Todo id is out of range."))?;
    Ok(D1Type::Integer(id))
}

fn d1_limit_arg(limit: usize, context: &'static str) -> AppResult<D1Type<'static>> {
    let limit =
        i32::try_from(limit).map_err(|_| AppError::internal(context, "value overflowed"))?;
    Ok(D1Type::Integer(limit))
}

fn map_todo(row: TodoRow) -> TodoItem {
    TodoItem {
        id: row.id,
        title: row.title,
        completed: row.completed != 0,
        created_at: row.created_at,
    }
}

async fn load_todo_stats(db: &worker::D1Database, session_id: &str) -> AppResult<TodoStats> {
    let session_arg = D1Type::Text(session_id);
    let row = db
        .prepare(
            "SELECT
                COUNT(*) AS total,
                COALESCE(SUM(CASE WHEN completed = 0 THEN 1 ELSE 0 END), 0) AS open,
                COALESCE(SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END), 0) AS completed
             FROM todos
             WHERE session_id = ?1",
        )
        .bind_refs(&session_arg)
        .map_err(|error| d1_error("Failed to bind todo stats query.", error))?
        .first::<TodoStatsRow>(None)
        .await
        .map_err(|error| d1_error("Failed to fetch todo stats from D1.", error))?
        .ok_or_else(|| {
            AppError::internal(
                "Todo stats query returned no rows.",
                "missing aggregate row",
            )
        })?;

    Ok(TodoStats {
        total: db_count_to_usize(row.total, "Todo total exceeded usize range.")?,
        open: db_count_to_usize(row.open, "Open todo count exceeded usize range.")?,
        completed: db_count_to_usize(row.completed, "Completed todo count exceeded usize range.")?,
    })
}

async fn count_session_todos(db: &worker::D1Database, session_id: &str) -> AppResult<usize> {
    let session_arg = D1Type::Text(session_id);
    let row = db
        .prepare("SELECT COUNT(*) AS total FROM todos WHERE session_id = ?1")
        .bind_refs(&session_arg)
        .map_err(|error| d1_error("Failed to bind session todo count query.", error))?
        .first::<CountRow>(None)
        .await
        .map_err(|error| d1_error("Failed to count session todos from D1.", error))?
        .ok_or_else(|| {
            AppError::internal(
                "Session todo count query returned no rows.",
                "missing count row",
            )
        })?;

    db_count_to_usize(row.total, "Session todo count exceeded usize range.")
}

async fn count_all_todos(db: &worker::D1Database) -> AppResult<usize> {
    let row = db
        .prepare("SELECT COUNT(*) AS total FROM todos")
        .first::<CountRow>(None)
        .await
        .map_err(|error| d1_error("Failed to count total todos from D1.", error))?
        .ok_or_else(|| {
            AppError::internal(
                "Total todo count query returned no rows.",
                "missing count row",
            )
        })?;

    db_count_to_usize(row.total, "Total todo count exceeded usize range.")
}

async fn trim_global_todos(db: &worker::D1Database) -> AppResult<()> {
    let total = count_all_todos(db).await?;
    if total < MAX_TODOS_GLOBAL {
        return Ok(());
    }

    let overflow = total + 1 - MAX_TODOS_GLOBAL;
    let overflow_arg = d1_limit_arg(overflow, "Todo trim count exceeded D1 range.")?;
    let result = db
        .prepare(
            "DELETE FROM todos
             WHERE id IN (
                SELECT id
                FROM todos
                ORDER BY id ASC
                LIMIT ?1
             )",
        )
        .bind_refs(&overflow_arg)
        .map_err(|error| d1_error("Failed to bind todo trim query.", error))?
        .run()
        .await
        .map_err(|error| d1_error("Failed to trim old todos from D1.", error))?;

    result
        .meta()
        .map_err(|error| d1_error("Failed to inspect D1 trim metadata.", error))?;

    Ok(())
}

async fn get_todo_by_id(db: &worker::D1Database, id: i64, session_id: &str) -> AppResult<TodoItem> {
    let id_arg = todo_id_arg(id)?;
    let session_arg = D1Type::Text(session_id);
    let args = [&id_arg, &session_arg];
    let row = db
        .prepare(
            "SELECT
                id,
                title,
                completed,
                strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at
             FROM todos
             WHERE id = ?1 AND session_id = ?2",
        )
        .bind_refs(args)
        .map_err(|error| d1_error("Failed to bind todo lookup query.", error))?
        .first::<TodoRow>(None)
        .await
        .map_err(|error| d1_error("Failed to fetch todo from D1.", error))?;

    row.map(map_todo).ok_or_else(|| {
        AppError::client(format!("Todo {id} was not found in this browser session."))
    })
}

fn ensure_row_changed(result: worker::D1Result, action: &str) -> AppResult<()> {
    let changed = result
        .meta()
        .map_err(|error| d1_error("Failed to inspect D1 mutation metadata.", error))?
        .and_then(|meta| meta.changes)
        .unwrap_or_default();

    if changed == 0 {
        Err(AppError::client(format!(
            "Todo {action} target was not found in this browser session."
        )))
    } else {
        Ok(())
    }
}

fn db_count_to_usize(value: i64, context: &'static str) -> AppResult<usize> {
    usize::try_from(value)
        .map_err(|_| AppError::internal(context, "value was negative or too large"))
}

fn d1_error(context: &'static str, error: impl std::fmt::Display) -> AppError {
    AppError::internal(context, error)
}
