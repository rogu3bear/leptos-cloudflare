use std::sync::Arc;

use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub env: Arc<worker::Env>,
    pub session_id: Arc<str>,
}

impl AppState {
    pub fn new(leptos_options: LeptosOptions, env: worker::Env, session_id: String) -> Self {
        Self {
            leptos_options,
            env: Arc::new(env),
            session_id: Arc::<str>::from(session_id),
        }
    }

    pub fn db(&self) -> worker::Result<worker::D1Database> {
        self.env.d1("DB")
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(input: &AppState) -> Self {
        input.leptos_options.clone()
    }
}
