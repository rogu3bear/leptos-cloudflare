pub mod contact;
pub mod state;
pub mod todos;

pub use state::AppState;

use leptos::prelude::ServerFnError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Client(String),
    Internal {
        context: &'static str,
        source: String,
    },
}

impl AppError {
    pub fn client(message: impl Into<String>) -> Self {
        Self::Client(message.into())
    }

    pub fn internal(context: &'static str, error: impl std::fmt::Display) -> Self {
        Self::Internal {
            context,
            source: error.to_string(),
        }
    }
}

pub fn server_error(error: AppError) -> ServerFnError {
    match error {
        AppError::Client(message) => ServerFnError::ServerError(message),
        AppError::Internal { context, source } => {
            // The request-level telemetry contract records the closed server
            // function name and failure outcome. Provider, SQL, and submitted
            // values stay out of logs by design.
            let _ = (context, source);
            ServerFnError::ServerError("Request failed. Try again later.".to_string())
        }
    }
}
