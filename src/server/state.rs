use std::sync::Arc;

use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub env: Arc<worker::Env>,
}

impl AppState {
    pub fn new(leptos_options: LeptosOptions, env: worker::Env) -> Self {
        Self {
            leptos_options,
            env: Arc::new(env),
        }
    }

    pub fn db(&self) -> worker::Result<worker::D1Database> {
        self.env.d1("DB")
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(input: &AppState) -> Self {
        input.leptos_options.clone()
    }
}
