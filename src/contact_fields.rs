use serde::{Deserialize, Serialize};

pub const MAX_NAME_CHARS: usize = 80;
pub const MAX_EMAIL_CHARS: usize = 254;
pub const MAX_TOPIC_CHARS: usize = 120;
pub const MAX_MESSAGE_CHARS: usize = 1_800;

#[cfg(any(feature = "ssr", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactSubmissionInput {
    pub name: String,
    pub email: String,
    pub topic: String,
    pub message: String,
    pub website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactResponse {
    pub accepted: bool,
    pub message: String,
}
