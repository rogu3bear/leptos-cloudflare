use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::contact::ContactSubmissionInput;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoStats {
    pub total: usize,
    pub open: usize,
    pub completed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodosResponse {
    pub items: Vec<TodoItem>,
    pub stats: TodoStats,
    pub visible_limit: usize,
    pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactResponse {
    pub accepted: bool,
    pub message: String,
}

#[server(ListTodos)]
pub async fn list_todos() -> Result<TodosResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        send_wrapper::SendWrapper::new(async move {
            crate::server::todos::list_todos()
                .await
                .map_err(crate::server::server_error)
        })
        .await
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
        send_wrapper::SendWrapper::new(async move {
            crate::server::todos::create_todo(title)
                .await
                .map_err(crate::server::server_error)
        })
        .await
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
        send_wrapper::SendWrapper::new(async move {
            crate::server::todos::toggle_todo(id)
                .await
                .map_err(crate::server::server_error)
        })
        .await
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
        send_wrapper::SendWrapper::new(async move {
            crate::server::todos::delete_todo(id)
                .await
                .map_err(crate::server::server_error)
        })
        .await
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
        send_wrapper::SendWrapper::new(async move {
            crate::server::todos::get_todo(id)
                .await
                .map_err(crate::server::server_error)
        })
        .await
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
        send_wrapper::SendWrapper::new(async move {
            crate::server::contact::submit_contact(ContactSubmissionInput {
                name,
                email,
                topic,
                message,
                website,
            })
            .await
            .map_err(crate::server::server_error)
        })
        .await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}
