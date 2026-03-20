use leptos::prelude::use_context;
use serde::Deserialize;
use worker::D1Type;

use crate::api::{TodoItem, TodoStats, TodosResponse};

use super::AppState;

#[derive(Debug, Deserialize)]
struct TodoRow {
    id: i64,
    title: String,
    completed: i64,
    created_at: String,
}

pub async fn list_todos() -> Result<TodosResponse, String> {
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
        .map_err(d1_error)?;

    let items = result
        .results::<TodoRow>()
        .map_err(d1_error)?
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

pub async fn create_todo(title: String) -> Result<TodoItem, String> {
    let db = database()?;
    let title = normalize_title(title)?;
    let title_arg = D1Type::Text(title.as_str());

    let result = db
        .prepare("INSERT INTO todos (title) VALUES (?1)")
        .bind_refs(&title_arg)
        .map_err(d1_error)?
        .run()
        .await
        .map_err(d1_error)?;

    let inserted_id = result
        .meta()
        .map_err(d1_error)?
        .and_then(|meta| meta.last_row_id)
        .ok_or_else(|| "D1 insert completed without returning last_row_id.".to_string())?;

    get_todo_by_id(&db, inserted_id).await
}

pub async fn toggle_todo(id: i64) -> Result<TodoItem, String> {
    let db = database()?;
    let id_arg = todo_id_arg(id)?;

    let result = db
        .prepare(
            "UPDATE todos
             SET completed = CASE completed WHEN 0 THEN 1 ELSE 0 END
             WHERE id = ?1",
        )
        .bind_refs(&id_arg)
        .map_err(d1_error)?
        .run()
        .await
        .map_err(d1_error)?;

    ensure_row_changed(result, "toggle")?;
    get_todo_by_id(&db, id).await
}

pub async fn delete_todo(id: i64) -> Result<(), String> {
    let db = database()?;
    let id_arg = todo_id_arg(id)?;

    let result = db
        .prepare("DELETE FROM todos WHERE id = ?1")
        .bind_refs(&id_arg)
        .map_err(d1_error)?
        .run()
        .await
        .map_err(d1_error)?;

    ensure_row_changed(result, "delete")
}

fn database() -> Result<worker::D1Database, String> {
    app_state()?.db().map_err(d1_error)
}

fn app_state() -> Result<AppState, String> {
    use_context::<AppState>()
        .ok_or_else(|| "Missing app state in Leptos server function context.".to_string())
}

fn normalize_title(title: String) -> Result<String, String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err("Todo titles cannot be empty.".to_string());
    }

    if trimmed.len() > 120 {
        return Err("Todo titles are capped at 120 characters.".to_string());
    }

    Ok(trimmed.to_string())
}

fn todo_id_arg(id: i64) -> Result<D1Type<'static>, String> {
    let id = i32::try_from(id).map_err(|_| "Todo id is out of range.".to_string())?;
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

async fn get_todo_by_id(db: &worker::D1Database, id: i64) -> Result<TodoItem, String> {
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
        .map_err(d1_error)?
        .first::<TodoRow>(None)
        .await
        .map_err(d1_error)?;

    row.map(map_todo)
        .ok_or_else(|| format!("Todo {id} was not found after the database write."))
}

fn ensure_row_changed(result: worker::D1Result, action: &str) -> Result<(), String> {
    let changed = result
        .meta()
        .map_err(d1_error)?
        .and_then(|meta| meta.changes)
        .unwrap_or_default();

    if changed == 0 {
        Err(format!("D1 reported no rows changed during {action}."))
    } else {
        Ok(())
    }
}

fn d1_error(error: impl std::fmt::Display) -> String {
    error.to_string()
}
