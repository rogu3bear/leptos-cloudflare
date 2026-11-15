use super::session::{session_cookie_header_value, RequestIdentity};

const CONTENT_SECURITY_POLICY_HEADER: &str = "content-security-policy";
const X_FRAME_OPTIONS_HEADER: &str = "x-frame-options";

pub(super) fn apply_response_headers(
    response: &mut axum::http::Response<axum::body::Body>,
    content_security_policy: &axum::http::header::HeaderValue,
    request_identity: &RequestIdentity,
) -> worker::Result<()> {
    use axum::http::header::{
        HeaderValue, CACHE_CONTROL, REFERRER_POLICY, SET_COOKIE, X_CONTENT_TYPE_OPTIONS,
    };
    use axum::http::HeaderName;

    let headers = response.headers_mut();
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(
        REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        HeaderName::from_static(CONTENT_SECURITY_POLICY_HEADER),
        content_security_policy.clone(),
    );
    headers.insert(
        HeaderName::from_static(X_FRAME_OPTIONS_HEADER),
        HeaderValue::from_static("DENY"),
    );

    if request_identity.set_cookie {
        headers.append(SET_COOKIE, session_cookie_header_value(request_identity)?);
    }

    Ok(())
}

pub(super) fn content_security_policy(
    nonce: &leptos::nonce::Nonce,
) -> worker::Result<axum::http::header::HeaderValue> {
    let script_sources = format!("'self' 'nonce-{nonce}' 'wasm-unsafe-eval'");
    let value = format!(
        "default-src 'self'; base-uri 'none'; object-src 'none'; frame-ancestors 'none'; form-action 'self'; img-src 'self' data:; connect-src 'self' ws: wss:; style-src 'self' 'unsafe-inline'; script-src {script_sources};"
    );
    axum::http::header::HeaderValue::from_str(&value)
        .map_err(|error| worker::Error::RustError(error.to_string()))
}
