use serde::Deserialize;
use worker::D1Type;

use crate::{
    api::ContactResponse,
    contact::{validate_contact_submission, ContactSubmissionInput, ValidatedContactSubmission},
};

use super::{
    d1::{count as db_count_to_usize, limit_arg as d1_limit_arg, request_database},
    AppError, AppResult,
};

const MAX_CONTACT_SUBMISSIONS_PER_HOUR: usize = 3;
const MAX_CONTACT_SUBMISSIONS_PER_DAY: usize = 8;
const MAX_CONTACT_SUBMISSIONS_GLOBAL: usize = 2_000;

#[derive(Debug, Deserialize)]
struct CountRow {
    total: i64,
}

pub async fn submit_contact(input: ContactSubmissionInput) -> AppResult<ContactResponse> {
    let submission =
        validate_contact_submission(input).map_err(|error| AppError::client(error.message()))?;
    let (db, session_id) = request_database()?;

    ensure_contact_budget(&db, session_id.as_str()).await?;
    trim_global_contact_submissions(&db).await?;
    insert_contact_submission(&db, session_id.as_str(), &submission).await?;

    Ok(ContactResponse {
        accepted: true,
        message: "Thanks. Your message was accepted.".to_string(),
    })
}

async fn insert_contact_submission(
    db: &worker::D1Database,
    session_id: &str,
    submission: &ValidatedContactSubmission,
) -> AppResult<()> {
    let session_arg = D1Type::Text(session_id);
    let name_arg = D1Type::Text(submission.name.as_str());
    let email_arg = D1Type::Text(submission.email.as_str());
    let topic_arg = D1Type::Text(submission.topic.as_str());
    let message_arg = D1Type::Text(submission.message.as_str());
    let args = [
        &session_arg,
        &name_arg,
        &email_arg,
        &topic_arg,
        &message_arg,
    ];

    db.prepare(
        "INSERT INTO contact_submissions (session_id, name, email, topic, message)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind_refs(args)
    .map_err(|_| AppError::Internal)?
    .run()
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}

async fn ensure_contact_budget(db: &worker::D1Database, session_id: &str) -> AppResult<()> {
    let hourly = count_contact_submissions_since(db, session_id, "-1 hour").await?;
    if hourly >= MAX_CONTACT_SUBMISSIONS_PER_HOUR {
        return Err(AppError::client(
            "Too many contact submissions. Try again later.",
        ));
    }

    let daily = count_contact_submissions_since(db, session_id, "-1 day").await?;
    if daily >= MAX_CONTACT_SUBMISSIONS_PER_DAY {
        return Err(AppError::client(
            "Too many contact submissions. Try again tomorrow.",
        ));
    }

    Ok(())
}

async fn count_contact_submissions_since(
    db: &worker::D1Database,
    session_id: &str,
    window: &'static str,
) -> AppResult<usize> {
    let session_arg = D1Type::Text(session_id);
    let window_arg = D1Type::Text(window);
    let args = [&session_arg, &window_arg];
    let row = db
        .prepare(
            "SELECT COUNT(*) AS total
             FROM contact_submissions
             WHERE session_id = ?1
               AND created_at >= datetime('now', ?2)",
        )
        .bind_refs(args)
        .map_err(|_| AppError::Internal)?
        .first::<CountRow>(None)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::Internal)?;

    db_count_to_usize(row.total)
}

async fn trim_global_contact_submissions(db: &worker::D1Database) -> AppResult<()> {
    let total = count_all_contact_submissions(db).await?;
    if total < MAX_CONTACT_SUBMISSIONS_GLOBAL {
        return Ok(());
    }

    let overflow = total + 1 - MAX_CONTACT_SUBMISSIONS_GLOBAL;
    let overflow_arg = d1_limit_arg(overflow)?;
    db.prepare(
        "DELETE FROM contact_submissions
         WHERE id IN (
            SELECT id
            FROM contact_submissions
            ORDER BY id ASC
            LIMIT ?1
         )",
    )
    .bind_refs(&overflow_arg)
    .map_err(|_| AppError::Internal)?
    .run()
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}

async fn count_all_contact_submissions(db: &worker::D1Database) -> AppResult<usize> {
    let row = db
        .prepare("SELECT COUNT(*) AS total FROM contact_submissions")
        .first::<CountRow>(None)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::Internal)?;

    db_count_to_usize(row.total)
}
