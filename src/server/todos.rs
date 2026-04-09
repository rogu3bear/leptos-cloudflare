use leptos::prelude::use_context;
use serde::Deserialize;
use worker::D1Type;

use crate::api::{TodoItem, TodoStats, TodosResponse};

use super::{AppError, AppResult, AppState};

#[derive(Debug, Deserialize)]
struct TodoRow {
    id: i64,
    title: String,
    completed: i64,
    created_at: String,
}

pub async fn list_todos() -> AppResult<TodosResponse> {
    let db = database()?;
    let result = db
        .prepare(
            "SELECT
                id,
                title,
                completed,
                strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at
             FROM todos
             ORDER BY completed ASC, id DESC",
        )
        .all()
        .await
        .map_err(|error| d1_error("Failed to list todos from D1.", error))?;

    let items = result
        .results::<TodoRow>()
        .map_err(|error| d1_error("Failed to deserialize todo rows from D1.", error))?
        .into_iter()
        .map(map_todo)
        .collect::<Vec<_>>();

    let stats = TodoStats {
        total: items.len(),
        open: items.iter().filter(|todo| !todo.completed).count(),
        completed: items.iter().filter(|todo| todo.completed).count(),
    };

    Ok(TodosResponse { items, stats })
}

pub async fn create_todo(title: String) -> AppResult<TodoItem> {
    let db = database()?;
    let title = normalize_title(title)?;
    let title_arg = D1Type::Text(title.as_str());

    let result = db
        .prepare("INSERT INTO todos (title) VALUES (?1)")
        .bind_refs(&title_arg)
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

    get_todo_by_id(&db, inserted_id).await
}

pub async fn toggle_todo(id: i64) -> AppResult<TodoItem> {
    let db = database()?;
    let id_arg = todo_id_arg(id)?;

    let result = db
        .prepare(
            "UPDATE todos
             SET completed = CASE completed WHEN 0 THEN 1 ELSE 0 END
             WHERE id = ?1",
        )
        .bind_refs(&id_arg)
        .map_err(|error| d1_error("Failed to bind todo toggle query.", error))?
        .run()
        .await
        .map_err(|error| d1_error("Failed to toggle todo in D1.", error))?;

    ensure_row_changed(result, "toggle")?;
    get_todo_by_id(&db, id).await
}

pub async fn delete_todo(id: i64) -> AppResult<()> {
    let db = database()?;
    let id_arg = todo_id_arg(id)?;

    let result = db
        .prepare("DELETE FROM todos WHERE id = ?1")
        .bind_refs(&id_arg)
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

fn map_todo(row: TodoRow) -> TodoItem {
    TodoItem {
        id: row.id,
        title: row.title,
        completed: row.completed != 0,
        created_at: row.created_at,
    }
}

async fn get_todo_by_id(db: &worker::D1Database, id: i64) -> AppResult<TodoItem> {
    let id_arg = todo_id_arg(id)?;
    let row = db
        .prepare(
            "SELECT
                id,
                title,
                completed,
                strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at
             FROM todos
             WHERE id = ?1",
        )
        .bind_refs(&id_arg)
        .map_err(|error| d1_error("Failed to bind todo lookup query.", error))?
        .first::<TodoRow>(None)
        .await
        .map_err(|error| d1_error("Failed to fetch todo from D1.", error))?;

    row.map(map_todo)
        .ok_or_else(|| AppError::client(format!("Todo {id} was not found.")))
}

fn ensure_row_changed(result: worker::D1Result, action: &str) -> AppResult<()> {
    let changed = result
        .meta()
        .map_err(|error| d1_error("Failed to inspect D1 mutation metadata.", error))?
        .and_then(|meta| meta.changes)
        .unwrap_or_default();

    if changed == 0 {
        Err(AppError::client(format!(
            "Todo {action} target was not found."
        )))
    } else {
        Ok(())
    }
}

fn d1_error(context: &'static str, error: impl std::fmt::Display) -> AppError {
    AppError::internal(context, error)
}
