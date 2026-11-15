use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::contact::ContactSubmissionInput;

pub use crate::contact_fields::ContactResponse;
pub use crate::todos::{normalize_todo_title, TodoItem, TodoUpdateOutcome, TodosResponse};

#[server(name = SearchTodos, endpoint = "search_todos")]
pub async fn search_todos(query: String, status: String) -> Result<TodosResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::search_todos(query, status)).await
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(name = UpdateTodo, endpoint = "update_todo")]
pub async fn update_todo(
    id: i64,
    title: String,
    completed: bool,
    expected_title: String,
    expected_completed: bool,
) -> Result<TodoUpdateOutcome, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::update_todo(
            id,
            title,
            completed,
            expected_title,
            expected_completed,
        ))
        .await
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(ListTodos)]
pub async fn list_todos() -> Result<TodosResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::list_todos()).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(CreateTodo)]
pub async fn create_todo(title: String) -> Result<TodoItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::create_todo(title)).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(ToggleTodo)]
pub async fn toggle_todo(id: i64) -> Result<TodoItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::toggle_todo(id)).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(DeleteTodo)]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::delete_todo(id)).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(GetTodo)]
pub async fn get_todo(id: i64) -> Result<TodoItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::todos::get_todo(id)).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}

#[server(SubmitContact)]
pub async fn submit_contact(
    name: String,
    email: String,
    topic: String,
    message: String,
    website: String,
) -> Result<ContactResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        crate::server::run(crate::server::contact::submit_contact(
            ContactSubmissionInput {
                name,
                email,
                topic,
                message,
                website,
            },
        ))
        .await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}
