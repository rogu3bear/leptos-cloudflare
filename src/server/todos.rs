use serde::Deserialize;
use worker::D1Type;

use crate::todos::{normalize_todo_title, TodoItem, TodoStats, TodoUpdateOutcome, TodosResponse};

use super::{
    d1::{count as db_count_to_usize, limit_arg as d1_limit_arg, request_database, write_context},
    AppError, AppResult,
};

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

pub async fn list_todos() -> AppResult<TodosResponse> {
    search_todos(String::new(), "all".into()).await
}

/// Filters run before the visible limit, including in the matching counts.
pub async fn search_todos(query: String, status: String) -> AppResult<TodosResponse> {
    let query = query.trim();
    if query.chars().count() > crate::todos::MAX_SEARCH_CHARS {
        return Err(AppError::client("Search is limited to 120 characters."));
    }
    let completed = match crate::todos::TodoStatus::parse(&status)
        .ok_or_else(|| AppError::client("Choose all, open, or completed records."))?
    {
        crate::todos::TodoStatus::All => -1,
        crate::todos::TodoStatus::Open => 0,
        crate::todos::TodoStatus::Completed => 1,
    };
    let (db, session_id) = request_database()?;
    let session_arg = D1Type::Text(session_id.as_str());
    let visible_limit_arg = d1_limit_arg(MAX_VISIBLE_TODOS)?;
    let query_arg = D1Type::Text(query);
    let status_arg = D1Type::Integer(completed);
    let result = db
        .prepare(
            "SELECT
                id,
                title,
                completed,
                strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at
             FROM todos
             WHERE session_id = ?1
               AND instr(lower(title), lower(?3)) > 0
               AND (?4 = -1 OR completed = ?4)
             ORDER BY completed ASC, id DESC
             LIMIT ?2",
        )
        .bind_refs([&session_arg, &visible_limit_arg, &query_arg, &status_arg])
        .map_err(|_| AppError::Internal)?
        .all()
        .await
        .map_err(|_| AppError::Internal)?;

    let items = result
        .results::<TodoRow>()
        .map_err(|_| AppError::Internal)?
        .into_iter()
        .map(map_todo)
        .collect::<Vec<_>>();

    let stats = load_todo_stats(&db, session_id.as_str(), query, completed).await?;

    Ok(TodosResponse {
        is_truncated: stats.total > items.len(),
        items,
        stats,
        visible_limit: MAX_VISIBLE_TODOS,
    })
}

pub async fn create_todo(title: String) -> AppResult<TodoItem> {
    let title = normalize_todo_title(&title).map_err(AppError::client)?;
    let (db, session_id, admission_key) = write_context().await?;
    let session_arg = D1Type::Text(&session_id);
    let title_arg = D1Type::Text(&title);
    let admission_arg = D1Type::Text(&admission_key);
    let session_limit = d1_limit_arg(MAX_TODOS_PER_SESSION)?;
    let global_limit = d1_limit_arg(MAX_TODOS_GLOBAL)?;
    let row = db
        .prepare(
            "INSERT INTO todos (session_id, title, admission_key)
         SELECT ?1, ?2, ?3
         WHERE (SELECT COUNT(*) FROM todos WHERE session_id = ?1) < ?4
           AND (SELECT COUNT(*) FROM todos WHERE admission_key = ?3) < ?4
           AND (SELECT COUNT(*) FROM todos) < ?5
         RETURNING id, title, completed, strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at",
        )
        .bind_refs([
            &session_arg,
            &title_arg,
            &admission_arg,
            &session_limit,
            &global_limit,
        ])
        .map_err(|_| AppError::Internal)?
        .first::<TodoRow>(None)
        .await
        .map_err(|_| AppError::Internal)?;
    row.map(map_todo).ok_or_else(|| AppError::client(
        "This demo's session, network or storage limit is full. Delete your own records or ask the operator."))
}

pub async fn toggle_todo(id: i64) -> AppResult<TodoItem> {
    let (db, session_id) = request_database()?;
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
        .map_err(|_| AppError::Internal)?
        .run()
        .await
        .map_err(|_| AppError::Internal)?;

    ensure_row_changed(result, "toggle")?;
    get_todo_by_id(&db, id, session_id.as_str()).await
}

/// Compare the mutable saved fields atomically. Retrying after a lost response
/// succeeds if the requested values are already present; conflicting values
/// require another explicit review. Session identity always comes from context.
pub async fn update_todo(
    id: i64,
    title: String,
    completed: bool,
    expected_title: String,
    expected_completed: bool,
) -> AppResult<TodoUpdateOutcome> {
    let (db, session_id) = request_database()?;
    let title = normalize_todo_title(&title).map_err(AppError::client)?;
    let id_arg = todo_id_arg(id)?;
    let session_arg = D1Type::Text(&session_id);
    let title_arg = D1Type::Text(&title);
    let completed_arg = D1Type::Integer(i32::from(completed));
    let expected_title_arg = D1Type::Text(&expected_title);
    let expected_completed_arg = D1Type::Integer(i32::from(expected_completed));
    let updated = db
        .prepare(
            "UPDATE todos SET title = ?3, completed = ?4
         WHERE id = ?1 AND session_id = ?2 AND title = ?5 AND completed = ?6
         RETURNING id, title, completed, strftime('%Y-%m-%d %H:%M UTC', created_at) AS created_at",
        )
        .bind_refs([
            &id_arg,
            &session_arg,
            &title_arg,
            &completed_arg,
            &expected_title_arg,
            &expected_completed_arg,
        ])
        .map_err(|_| AppError::Internal)?
        .first::<TodoRow>(None)
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(row) = updated {
        return Ok(TodoUpdateOutcome::Saved(map_todo(row)));
    }
    match get_todo_by_id(&db, id, &session_id).await {
        Ok(current) if current.title == title && current.completed == completed => {
            Ok(TodoUpdateOutcome::Saved(current))
        }
        Ok(current) => Ok(TodoUpdateOutcome::Conflict(current)),
        Err(AppError::Client(_)) => Ok(TodoUpdateOutcome::Unavailable),
        Err(error) => Err(error),
    }
}

pub async fn delete_todo(id: i64) -> AppResult<()> {
    let (db, session_id) = request_database()?;
    let id_arg = todo_id_arg(id)?;
    let session_arg = D1Type::Text(session_id.as_str());
    let args = [&id_arg, &session_arg];

    let result = db
        .prepare("DELETE FROM todos WHERE id = ?1 AND session_id = ?2")
        .bind_refs(args)
        .map_err(|_| AppError::Internal)?
        .run()
        .await
        .map_err(|_| AppError::Internal)?;

    ensure_row_changed(result, "delete")
}

pub async fn get_todo(id: i64) -> AppResult<TodoItem> {
    let (db, session_id) = request_database()?;
    get_todo_by_id(&db, id, session_id.as_str()).await
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

async fn load_todo_stats(
    db: &worker::D1Database,
    session_id: &str,
    query: &str,
    completed: i32,
) -> AppResult<TodoStats> {
    let session_arg = D1Type::Text(session_id);
    let query_arg = D1Type::Text(query);
    let status_arg = D1Type::Integer(completed);
    let row = db
        .prepare(
            "SELECT
                COUNT(*) AS total,
                COALESCE(SUM(CASE WHEN completed = 0 THEN 1 ELSE 0 END), 0) AS open,
                COALESCE(SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END), 0) AS completed
             FROM todos
             WHERE session_id = ?1
               AND instr(lower(title), lower(?2)) > 0
               AND (?3 = -1 OR completed = ?3)",
        )
        .bind_refs([&session_arg, &query_arg, &status_arg])
        .map_err(|_| AppError::Internal)?
        .first::<TodoStatsRow>(None)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::Internal)?;

    Ok(TodoStats {
        total: db_count_to_usize(row.total)?,
        open: db_count_to_usize(row.open)?,
        completed: db_count_to_usize(row.completed)?,
    })
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
        .map_err(|_| AppError::Internal)?
        .first::<TodoRow>(None)
        .await
        .map_err(|_| AppError::Internal)?;

    row.map(map_todo).ok_or_else(|| {
        AppError::client(format!("Todo {id} was not found in this browser session."))
    })
}

fn ensure_row_changed(result: worker::D1Result, action: &str) -> AppResult<()> {
    let changed = result
        .meta()
        .map_err(|_| AppError::Internal)?
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
