#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactSubmissionInput {
    pub name: String,
    pub email: String,
    pub topic: String,
    pub message: String,
    pub website: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedContactSubmission {
    pub name: String,
    pub email: String,
    pub topic: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactValidationError {
    message: &'static str,
}

impl ContactValidationError {
    pub fn message(&self) -> &'static str {
        self.message
    }
}

const MAX_NAME_CHARS: usize = 80;
const MAX_EMAIL_CHARS: usize = 254;
const MAX_TOPIC_CHARS: usize = 120;
const MIN_MESSAGE_CHARS: usize = 20;
const MAX_MESSAGE_CHARS: usize = 1_800;
const MAX_MESSAGE_LINES: usize = 40;
const MAX_LINK_MARKERS: usize = 3;

pub fn validate_contact_submission(
    input: ContactSubmissionInput,
) -> Result<ValidatedContactSubmission, ContactValidationError> {
    if !input.website.trim().is_empty() {
        return Err(ContactValidationError {
            message: "Submission could not be accepted.",
        });
    }

    let name = normalize_single_line(
        &input.name,
        MAX_NAME_CHARS,
        "Name is required.",
        "Name is too long.",
    )?;
    let email = normalize_email(&input.email)?;
    let topic = normalize_single_line(
        &input.topic,
        MAX_TOPIC_CHARS,
        "Topic is required.",
        "Topic is too long.",
    )?;
    let message = normalize_message(&input.message)?;

    Ok(ValidatedContactSubmission {
        name,
        email,
        topic,
        message,
    })
}

fn normalize_single_line(
    value: &str,
    max_chars: usize,
    empty_message: &'static str,
    length_message: &'static str,
) -> Result<String, ContactValidationError> {
    if value.contains(['\r', '\n']) || contains_disallowed_control(value, false) {
        return Err(ContactValidationError {
            message: "Use plain text without control characters.",
        });
    }

    let normalized = collapse_horizontal_whitespace(value);
    if normalized.is_empty() {
        return Err(ContactValidationError {
            message: empty_message,
        });
    }
    if normalized.chars().count() > max_chars {
        return Err(ContactValidationError {
            message: length_message,
        });
    }

    Ok(normalized)
}

fn normalize_email(value: &str) -> Result<String, ContactValidationError> {
    let normalized = normalize_single_line(
        value,
        MAX_EMAIL_CHARS,
        "Email is required.",
        "Email is too long.",
    )?
    .to_ascii_lowercase();

    if !normalized.is_ascii()
        || normalized.contains(char::is_whitespace)
        || normalized.matches('@').count() != 1
    {
        return Err(ContactValidationError {
            message: "Use a valid email address.",
        });
    }

    let (local, domain) = normalized.split_once('@').ok_or(ContactValidationError {
        message: "Use a valid email address.",
    })?;
    if local.is_empty()
        || domain.is_empty()
        || local.len() > 64
        || domain.len() > 253
        || !domain.contains('.')
        || domain.starts_with('.')
        || domain.ends_with('.')
        || domain.contains("..")
    {
        return Err(ContactValidationError {
            message: "Use a valid email address.",
        });
    }

    Ok(normalized)
}

fn normalize_message(value: &str) -> Result<String, ContactValidationError> {
    if contains_disallowed_control(value, true) {
        return Err(ContactValidationError {
            message: "Use plain text without control characters.",
        });
    }

    let normalized = value
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    let char_count = normalized.chars().count();
    if char_count < MIN_MESSAGE_CHARS {
        return Err(ContactValidationError {
            message: "Message is too short.",
        });
    }
    if char_count > MAX_MESSAGE_CHARS {
        return Err(ContactValidationError {
            message: "Message is too long.",
        });
    }
    if normalized.lines().count() > MAX_MESSAGE_LINES {
        return Err(ContactValidationError {
            message: "Message has too many lines.",
        });
    }
    if link_marker_count(&normalized) > MAX_LINK_MARKERS {
        return Err(ContactValidationError {
            message: "Message has too many links.",
        });
    }

    Ok(normalized)
}

fn collapse_horizontal_whitespace(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn contains_disallowed_control(value: &str, allow_newlines: bool) -> bool {
    value.chars().any(|character| {
        if allow_newlines && matches!(character, '\n' | '\r' | '\t') {
            return false;
        }

        character.is_control()
    })
}

fn link_marker_count(value: &str) -> usize {
    let lower = value.to_ascii_lowercase();
    ["https://", "http://", "www."]
        .iter()
        .map(|marker| lower.matches(marker).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> ContactSubmissionInput {
        ContactSubmissionInput {
            name: "Ada Lovelace".to_string(),
            email: "ADA@Example.COM".to_string(),
            topic: "Launch question".to_string(),
            message: "I would like to understand the launch timeline for this project.".to_string(),
            website: String::new(),
        }
    }

    #[test]
    fn accepts_and_normalizes_valid_contact_submission() {
        let submission = validate_contact_submission(valid_input()).unwrap();

        assert_eq!(submission.name, "Ada Lovelace");
        assert_eq!(submission.email, "ada@example.com");
        assert_eq!(submission.topic, "Launch question");
    }

    #[test]
    fn rejects_honeypot_submissions() {
        let mut input = valid_input();
        input.website = "https://spam.example".to_string();

        let error = validate_contact_submission(input).unwrap_err();

        assert_eq!(error.message(), "Submission could not be accepted.");
    }

    #[test]
    fn rejects_header_injection_in_single_line_fields() {
        let mut input = valid_input();
        input.email = "sender@example.com\r\nBcc: attacker@example.com".to_string();

        let error = validate_contact_submission(input).unwrap_err();

        assert_eq!(
            error.message(),
            "Use plain text without control characters."
        );
    }

    #[test]
    fn rejects_messages_that_are_too_short_or_link_heavy() {
        let mut short = valid_input();
        short.message = "too short".to_string();
        assert_eq!(
            validate_contact_submission(short).unwrap_err().message(),
            "Message is too short."
        );

        let mut spam = valid_input();
        spam.message =
            "Links https://a.test https://b.test http://c.test www.d.test are excessive."
                .to_string();
        assert_eq!(
            validate_contact_submission(spam).unwrap_err().message(),
            "Message has too many links."
        );
    }
}
