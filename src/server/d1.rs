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

/// Network admission is separate from the anonymous row-ownership cookie.
/// The database-local key keeps raw addresses out of persisted application data.
pub async fn write_context() -> AppResult<(worker::D1Database, String, String)> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    #[derive(serde::Deserialize)]
    struct Key {
        secret: String,
    }
    let state = leptos::prelude::use_context::<AppState>().ok_or(AppError::Internal)?;
    let ip = state.client_ip.as_deref().ok_or_else(|| {
        AppError::client(
            "Client network identity is unavailable. Try again through the configured edge.",
        )
    })?;
    let db = state.db().map_err(|_| AppError::Internal)?;
    let load_key = || db.prepare("SELECT secret FROM anonymous_admission_key WHERE id = 1");
    let key = match load_key()
        .first::<Key>(None)
        .await
        .map_err(|_| AppError::Internal)?
    {
        Some(key) => key,
        None => {
            let mut bytes = [0u8; 32];
            getrandom::fill(&mut bytes).map_err(|_| AppError::Internal)?;
            let secret: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            let secret_arg = D1Type::Text(&secret);
            db.prepare("INSERT OR IGNORE INTO anonymous_admission_key (id, secret) VALUES (1, ?1)")
                .bind_refs([&secret_arg])
                .map_err(|_| AppError::Internal)?
                .run()
                .await
                .map_err(|_| AppError::Internal)?;
            // Concurrent initializers must all use the database winner's key.
            load_key()
                .first::<Key>(None)
                .await
                .map_err(|_| AppError::Internal)?
                .ok_or(AppError::Internal)?
        }
    };
    if key.secret.len() != 64 {
        return Err(AppError::Internal);
    }
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.secret.as_bytes()).map_err(|_| AppError::Internal)?;
    mac.update(b"leptos-cf-admission-v1\0");
    mac.update(ip.as_bytes());
    let admission_key = mac
        .finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok((db, state.session_id().to_owned(), admission_key))
}
