mod api;
mod app;
mod asset_hashes;
mod components;
#[cfg(any(feature = "ssr", test))]
mod contact;
#[cfg(feature = "ssr")]
mod server;
#[cfg(any(feature = "ssr", test))]
mod telemetry;

#[cfg(feature = "ssr")]
const CONTENT_SECURITY_POLICY_HEADER: &str = "content-security-policy";
#[cfg(feature = "ssr")]
const SESSION_COOKIE_NAME: &str = "leptos_cf_session";
#[cfg(feature = "ssr")]
const SESSION_COOKIE_MAX_AGE_SECONDS: u32 = 60 * 60 * 24 * 30;
#[cfg(feature = "ssr")]
const SESSION_ID_BYTES: usize = 32;
#[cfg(feature = "ssr")]
const X_FRAME_OPTIONS_HEADER: &str = "x-frame-options";
#[cfg(feature = "ssr")]
const MAX_SERVER_FUNCTION_BODY_BYTES: usize = 4 * 1024;

#[cfg(feature = "ssr")]
#[derive(Clone)]
struct RequestIdentity {
    secure_cookie: bool,
    session_id: String,
    set_cookie: bool,
}

#[cfg(feature = "ssr")]
#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let observation = telemetry::RequestObservation::start(req.uri().path(), req.method().as_str());
    let result = fetch_inner(req, env, ctx).await;
    observation.finish(
        result
            .as_ref()
            .ok()
            .map(|response| response.status().as_u16()),
        result.is_err(),
    );
    result
}

#[cfg(feature = "ssr")]
async fn fetch_inner(
    req: worker::HttpRequest,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    use axum::body::Body;
    use axum::extract::DefaultBodyLimit;
    use axum::http::{Response, StatusCode};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_service::Service;

    let conf =
        get_configuration(None).map_err(|error| worker::Error::RustError(error.to_string()))?;
    let leptos_options = conf.leptos_options;
    let request_path = req.uri().path().to_string();
    let request_identity = request_identity(&req)?;
    let request_nonce = leptos::nonce::Nonce::new();
    let content_security_policy = content_security_policy(&request_nonce)?;

    if let Some(rejection) = reject_api_request(&req) {
        let mut response = Response::builder()
            .status(rejection.status)
            .body(Body::from(rejection.message))
            .map_err(|error| worker::Error::RustError(error.to_string()))?;
        apply_response_headers(&mut response, &content_security_policy, &request_identity)?;
        return Ok(response);
    }

    if server_fn_body_too_large(&req) {
        let mut response = Response::builder()
            .status(StatusCode::PAYLOAD_TOO_LARGE)
            .body(Body::from("Request payload exceeds the demo limit."))
            .map_err(|error| worker::Error::RustError(error.to_string()))?;
        apply_response_headers(&mut response, &content_security_policy, &request_identity)?;
        return Ok(response);
    }

    let routes = generate_route_list(app::App);
    let state = server::AppState::new(
        leptos_options.clone(),
        env,
        request_identity.session_id.clone(),
    );

    let mut router = Router::new()
        .layer(DefaultBodyLimit::max(MAX_SERVER_FUNCTION_BODY_BYTES))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let request_nonce = request_nonce.clone();
                move || provide_context(request_nonce.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || app::shell(leptos_options.clone())
            },
        )
        .with_state(state);

    let mut response = router.call(req).await?;
    if !request_path.starts_with("/api/")
        && !is_public_document_path(&request_path)
        && response.status().is_success()
    {
        *response.status_mut() = StatusCode::NOT_FOUND;
    }
    apply_response_headers(&mut response, &content_security_policy, &request_identity)?;

    Ok(response)
}

#[cfg(any(feature = "ssr", test))]
fn is_public_document_path(path: &str) -> bool {
    let normalized = path
        .strip_suffix('/')
        .filter(|without_trailing_slash| !without_trailing_slash.is_empty())
        .unwrap_or(path);

    matches!(
        normalized,
        "/" | "/start" | "/architecture" | "/patterns" | "/lab" | "/about" | "/contact"
    ) || has_single_path_parameter(normalized, "/lab/")
        || has_single_path_parameter(normalized, "/todo/")
}

#[cfg(any(feature = "ssr", test))]
fn has_single_path_parameter(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|parameter| !parameter.is_empty() && !parameter.contains('/'))
}

#[cfg(feature = "ssr")]
struct ApiRejection {
    status: axum::http::StatusCode,
    message: &'static str,
}

#[cfg(feature = "ssr")]
fn reject_api_request(req: &worker::HttpRequest) -> Option<ApiRejection> {
    use axum::http::{Method, StatusCode};

    if !req.uri().path().starts_with("/api/") {
        return None;
    }

    let method = req.method();
    if !matches!(method, &Method::GET | &Method::HEAD | &Method::POST) {
        return Some(ApiRejection {
            status: StatusCode::METHOD_NOT_ALLOWED,
            message: "Method not allowed for server functions.",
        });
    }

    if method == Method::POST {
        if !same_origin_api_request(req) {
            return Some(ApiRejection {
                status: StatusCode::FORBIDDEN,
                message: "Cross-origin server function requests are not allowed.",
            });
        }

        if !supported_api_content_type(
            req.headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
        ) {
            return Some(ApiRejection {
                status: StatusCode::UNSUPPORTED_MEDIA_TYPE,
                message: "Unsupported server function content type.",
            });
        }
    }

    None
}

#[cfg(feature = "ssr")]
fn same_origin_api_request(req: &worker::HttpRequest) -> bool {
    let headers = req.headers();
    let sec_fetch_site = headers
        .get("sec-fetch-site")
        .and_then(|value| value.to_str().ok())
        .map(str::trim);
    let origin = headers.get("origin").and_then(|value| value.to_str().ok());
    let request_origin = request_origin(req);

    same_origin_api_policy_allows(sec_fetch_site, origin, request_origin.as_deref())
}

#[cfg(feature = "ssr")]
fn request_origin(req: &worker::HttpRequest) -> Option<String> {
    let scheme = req
        .uri()
        .scheme_str()
        .or_else(|| {
            req.headers()
                .get("x-forwarded-proto")
                .and_then(|value| value.to_str().ok())
        })
        .unwrap_or("https");
    let authority = req
        .uri()
        .authority()
        .map(|authority| authority.as_str())
        .or_else(|| {
            req.headers()
                .get(axum::http::header::HOST)
                .and_then(|value| value.to_str().ok())
        })?;

    normalize_origin_parts(scheme, authority)
}

#[cfg(feature = "ssr")]
fn apply_response_headers(
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

#[cfg(feature = "ssr")]
fn content_security_policy(
    nonce: &leptos::nonce::Nonce,
) -> worker::Result<axum::http::header::HeaderValue> {
    let script_sources = format!("'self' 'nonce-{nonce}' 'wasm-unsafe-eval'");
    let value = format!(
        "default-src 'self'; base-uri 'none'; object-src 'none'; frame-ancestors 'none'; form-action 'self'; img-src 'self' data:; connect-src 'self' ws: wss:; style-src 'self' 'unsafe-inline'; script-src {script_sources};"
    );
    axum::http::header::HeaderValue::from_str(&value)
        .map_err(|error| worker::Error::RustError(error.to_string()))
}

#[cfg(feature = "ssr")]
fn request_identity(req: &worker::HttpRequest) -> worker::Result<RequestIdentity> {
    let session_id = request_session_id(req);
    Ok(match session_id {
        Some(session_id) => RequestIdentity {
            secure_cookie: is_secure_request(req),
            session_id,
            set_cookie: false,
        },
        None => RequestIdentity {
            secure_cookie: is_secure_request(req),
            session_id: random_session_id()?,
            set_cookie: true,
        },
    })
}

#[cfg(feature = "ssr")]
fn request_session_id(req: &worker::HttpRequest) -> Option<String> {
    req.headers()
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .and_then(parse_session_cookie)
}

#[cfg(feature = "ssr")]
fn parse_session_cookie(cookie_header: &str) -> Option<String> {
    cookie_header.split(';').map(str::trim).find_map(|cookie| {
        let (name, value) = cookie.split_once('=')?;
        if name == SESSION_COOKIE_NAME && valid_session_id(value) {
            Some(value.to_string())
        } else {
            None
        }
    })
}

#[cfg(feature = "ssr")]
fn valid_session_id(value: &str) -> bool {
    value.len() == SESSION_ID_BYTES * 2
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(feature = "ssr")]
fn random_session_id() -> worker::Result<String> {
    let mut bytes = [0u8; SESSION_ID_BYTES];
    getrandom::fill(&mut bytes).map_err(|error| worker::Error::RustError(error.to_string()))?;
    Ok(hex_encode(&bytes))
}

#[cfg(feature = "ssr")]
fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(feature = "ssr")]
fn session_cookie_header_value(
    request_identity: &RequestIdentity,
) -> worker::Result<axum::http::header::HeaderValue> {
    let secure = if request_identity.secure_cookie {
        "; Secure"
    } else {
        ""
    };
    let value = format!(
        "{SESSION_COOKIE_NAME}={}; HttpOnly; Path=/; SameSite=Lax; Max-Age={SESSION_COOKIE_MAX_AGE_SECONDS}{secure}",
        request_identity.session_id
    );
    axum::http::header::HeaderValue::from_str(&value)
        .map_err(|error| worker::Error::RustError(error.to_string()))
}

#[cfg(feature = "ssr")]
fn is_secure_request(req: &worker::HttpRequest) -> bool {
    req.uri().scheme_str() == Some("https")
        || req
            .headers()
            .get("x-forwarded-proto")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.eq_ignore_ascii_case("https"))
}

#[cfg(feature = "ssr")]
fn server_fn_body_too_large(req: &worker::HttpRequest) -> bool {
    use axum::http::header::CONTENT_LENGTH;

    if !req.uri().path().starts_with("/api/") {
        return false;
    }

    req.headers()
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|value| value > MAX_SERVER_FUNCTION_BODY_BYTES)
}

#[cfg(any(feature = "ssr", test))]
fn supported_api_content_type(content_type: Option<&str>) -> bool {
    let Some(content_type) = content_type else {
        return true;
    };
    let media_type = content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    matches!(
        media_type.as_str(),
        "application/json"
            | "application/x-www-form-urlencoded"
            | "application/cbor"
            | "multipart/form-data"
    )
}

#[cfg(any(feature = "ssr", test))]
fn same_origin_api_policy_allows(
    sec_fetch_site: Option<&str>,
    origin: Option<&str>,
    request_origin: Option<&str>,
) -> bool {
    let Some(request_origin) = request_origin else {
        return false;
    };
    let sec_fetch_site = sec_fetch_site.map(str::trim);

    if sec_fetch_site.is_some_and(|value| {
        value.eq_ignore_ascii_case("cross-site") || value.eq_ignore_ascii_case("none")
    }) {
        return false;
    }

    if let Some(origin) = origin {
        return origins_match(origin, request_origin);
    }

    sec_fetch_site.is_some_and(|value| value.eq_ignore_ascii_case("same-origin"))
}

#[cfg(any(feature = "ssr", test))]
fn origins_match(left: &str, right: &str) -> bool {
    normalize_origin(left).is_some_and(|left| {
        normalize_origin(right).is_some_and(|right| left.eq_ignore_ascii_case(&right))
    })
}

#[cfg(any(feature = "ssr", test))]
fn normalize_origin(value: &str) -> Option<String> {
    let value = value.trim();
    let (scheme, rest) = value.split_once("://")?;
    let authority = rest.split('/').next().unwrap_or_default();

    normalize_origin_parts(scheme, authority)
}

#[cfg(any(feature = "ssr", test))]
fn normalize_origin_parts(scheme: &str, authority: &str) -> Option<String> {
    let scheme = scheme.trim().to_ascii_lowercase();
    if !matches!(scheme.as_str(), "http" | "https") {
        return None;
    }

    let authority = authority.trim().to_ascii_lowercase();
    if authority.is_empty()
        || authority.contains('@')
        || authority.contains('/')
        || authority.contains('\\')
        || authority.chars().any(char::is_whitespace)
        || authority.chars().any(char::is_control)
    {
        return None;
    }

    let authority = match (scheme.as_str(), authority.strip_suffix(":443")) {
        ("https", Some(host)) => host,
        _ => match (scheme.as_str(), authority.strip_suffix(":80")) {
            ("http", Some(host)) => host,
            _ => authority.as_str(),
        },
    };
    if authority.is_empty() {
        return None;
    }

    Some(format!("{scheme}://{authority}"))
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_matching_normalizes_case_and_default_ports() {
        assert!(origins_match(
            "https://Example.COM:443/contact",
            "https://example.com"
        ));
        assert!(origins_match("http://example.com:80", "http://EXAMPLE.com"));
        assert!(!origins_match(
            "https://evil.example",
            "https://example.com"
        ));
    }

    #[test]
    fn origin_matching_rejects_malformed_authorities() {
        assert!(!origins_match(
            "https://example.com@evil.example",
            "https://example.com"
        ));
        assert!(!origins_match(
            "javascript://example.com",
            "https://example.com"
        ));
    }

    #[test]
    fn api_content_type_guard_allows_leptos_payloads() {
        assert!(supported_api_content_type(None));
        assert!(supported_api_content_type(Some(
            "application/x-www-form-urlencoded;charset=UTF-8"
        )));
        assert!(supported_api_content_type(Some("application/json")));
        assert!(supported_api_content_type(Some(
            "multipart/form-data; boundary=abc"
        )));
        assert!(!supported_api_content_type(Some("text/plain")));
    }

    #[test]
    fn api_origin_policy_requires_origin_or_same_origin_fetch_metadata() {
        assert!(same_origin_api_policy_allows(
            Some("same-origin"),
            None,
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            Some("same-site"),
            None,
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            None,
            None,
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            Some("same-origin"),
            None,
            None
        ));
    }

    #[test]
    fn api_origin_policy_matches_origin_and_honors_fetch_metadata_blocks() {
        assert!(same_origin_api_policy_allows(
            Some("same-site"),
            Some("https://Example.COM:443"),
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            Some("same-origin"),
            Some("https://evil.example"),
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            Some("cross-site"),
            Some("https://example.com"),
            Some("https://example.com")
        ));
        assert!(!same_origin_api_policy_allows(
            Some("none"),
            Some("https://example.com"),
            Some("https://example.com")
        ));
    }

    #[test]
    fn public_document_paths_cover_static_dynamic_and_compatibility_routes() {
        for path in [
            "/",
            "/start",
            "/architecture",
            "/patterns",
            "/lab",
            "/about",
            "/contact",
            "/lab/42",
            "/todo/42",
            "/start/",
            "/lab/",
        ] {
            assert!(
                is_public_document_path(path),
                "expected known route: {path}"
            );
        }
    }

    #[test]
    fn public_document_paths_reject_wildcards_and_extra_segments() {
        for path in [
            "/outside-field-guide",
            "/lab/42/edit",
            "/todo/42/edit",
            "/start/here",
            "/lab//",
        ] {
            assert!(
                !is_public_document_path(path),
                "expected wildcard route: {path}"
            );
        }
    }
}
