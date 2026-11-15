use super::{AppError, AppResult, AppState};
use worker::D1Type;

pub fn request_database() -> AppResult<(worker::D1Database, String)> {
    let state = leptos::prelude::use_context::<AppState>().ok_or(AppError::Internal)?;
    let db = state.db().map_err(|_| AppError::Internal)?;
    Ok((db, state.session_id().to_owned()))
}

pub fn limit_arg(limit: usize) -> AppResult<D1Type<'static>> {
    i32::try_from(limit)
        .map(D1Type::Integer)
        .map_err(|_| AppError::Internal)
}

pub fn count(value: i64) -> AppResult<usize> {
    usize::try_from(value).map_err(|_| AppError::Internal)
}
