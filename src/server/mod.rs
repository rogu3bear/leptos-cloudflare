pub mod state;
pub mod todos;

pub use state::AppState;

use leptos::prelude::ServerFnError;

pub fn server_error(error: impl std::fmt::Display) -> ServerFnError {
    ServerFnError::ServerError(error.to_string())
}
