use super::{
    d1::{limit_arg, write_context},
    AppError, AppResult,
};
use crate::{
    api::ContactResponse,
    contact::{validate_contact_submission, ContactSubmissionInput},
};
use worker::D1Type;

const MAX_CONTACT_SUBMISSIONS_PER_HOUR: usize = 3;
const MAX_CONTACT_SUBMISSIONS_PER_DAY: usize = 8;
const MAX_CONTACT_SUBMISSIONS_GLOBAL: usize = 2_000;

pub async fn submit_contact(input: ContactSubmissionInput) -> AppResult<ContactResponse> {
    let submission =
        validate_contact_submission(input).map_err(|error| AppError::client(error.message()))?;
    let (db, session_id, admission_key) = write_context().await?;
    let session = D1Type::Text(&session_id);
    let admission = D1Type::Text(&admission_key);
    let name = D1Type::Text(&submission.name);
    let email = D1Type::Text(&submission.email);
    let topic = D1Type::Text(&submission.topic);
    let message = D1Type::Text(&submission.message);
    let hourly = limit_arg(MAX_CONTACT_SUBMISSIONS_PER_HOUR)?;
    let daily = limit_arg(MAX_CONTACT_SUBMISSIONS_PER_DAY)?;
    let global = limit_arg(MAX_CONTACT_SUBMISSIONS_GLOBAL)?;
    // One write statement serializes budget decisions with insertion on D1.
    // At capacity, preserve every existing row rather than evicting another user.
    let result = db.prepare(
        "INSERT INTO contact_submissions (session_id, admission_key, name, email, topic, message)
         SELECT ?1, ?2, ?3, ?4, ?5, ?6
         WHERE (SELECT COUNT(*) FROM contact_submissions WHERE admission_key = ?2 AND created_at >= datetime('now', '-1 hour')) < ?7
           AND (SELECT COUNT(*) FROM contact_submissions WHERE admission_key = ?2 AND created_at >= datetime('now', '-1 day')) < ?8
           AND (SELECT COUNT(*) FROM contact_submissions WHERE session_id = ?1 AND created_at >= datetime('now', '-1 hour')) < ?7
           AND (SELECT COUNT(*) FROM contact_submissions WHERE session_id = ?1 AND created_at >= datetime('now', '-1 day')) < ?8
           AND (SELECT COUNT(*) FROM contact_submissions) < ?9")
        .bind_refs([&session, &admission, &name, &email, &topic, &message, &hourly, &daily, &global])
        .map_err(|_| AppError::Internal)?.run().await.map_err(|_| AppError::Internal)?;
    if result
        .meta()
        .map_err(|_| AppError::Internal)?
        .and_then(|meta| meta.changes)
        != Some(1)
    {
        return Err(AppError::client("Contact submission limit reached. Try again later or ask the operator if storage is full."));
    }
    Ok(ContactResponse {
        accepted: true,
        message: "Thanks. Your message was accepted.".into(),
    })
}
