use serde::{Deserialize, Serialize};

pub const MAX_TITLE_BYTES: usize = 120;
pub const MAX_SEARCH_CHARS: usize = 120;

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

/// A saved result acknowledges the exact submitted values. Conflict includes
/// only the caller's own current record; unavailable does not disclose others.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoUpdateOutcome {
    Saved(TodoItem),
    Conflict(TodoItem),
    Unavailable,
}

pub fn normalize_todo_title(title: &str) -> Result<String, &'static str> {
    let title = title.trim();
    if title.is_empty() {
        return Err("Todo titles cannot be empty.");
    }
    if title.len() > MAX_TITLE_BYTES {
        return Err("That title is too long. Use a shorter title.");
    }
    Ok(title.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodoStatus {
    All,
    Open,
    Completed,
}
impl TodoStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "all" => Some(Self::All),
            "open" => Some(Self::Open),
            "completed" => Some(Self::Completed),
            _ => None,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Open => "open",
            Self::Completed => "completed",
        }
    }
}
