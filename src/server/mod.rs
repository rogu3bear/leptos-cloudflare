pub mod contact;
mod d1;
pub mod state;
pub mod todos;

pub use state::AppState;

use leptos::prelude::ServerFnError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Client(String),
    Internal,
}

impl AppError {
    pub fn client(message: impl Into<String>) -> Self {
        Self::Client(message.into())
    }
}

pub fn server_error(error: AppError) -> ServerFnError {
    match error {
        AppError::Client(message) => ServerFnError::ServerError(message),
        // Provider and submitted values stay outside the public error and logs.
        AppError::Internal => {
            ServerFnError::ServerError("Request failed. Try again later.".to_string())
        }
    }
}

// workers-rs futures are not Send; Leptos server functions require Send.
pub fn run<T>(
    future: impl std::future::Future<Output = AppResult<T>>,
) -> impl std::future::Future<Output = Result<T, ServerFnError>> + Send {
    send_wrapper::SendWrapper::new(async move { future.await.map_err(server_error) })
}
