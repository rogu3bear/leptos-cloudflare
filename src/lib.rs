mod api;
mod app;
mod asset_hashes;
mod components;
#[cfg(feature = "ssr")]
mod server;

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
const MAX_SERVER_FUNCTION_BODY_BYTES: usize = 2 * 1024;

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
    let request_identity = request_identity(&req)?;
    let content_security_policy = content_security_policy(&leptos_options)?;

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
        .leptos_routes_with_context(&state, routes, || {}, {
            let leptos_options = leptos_options.clone();
            move || app::shell(leptos_options.clone())
        })
        .with_state(state);

    let mut response = router.call(req).await?;
    apply_response_headers(&mut response, &content_security_policy, &request_identity)?;

    Ok(response)
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
    options: &leptos::prelude::LeptosOptions,
) -> worker::Result<axum::http::header::HeaderValue> {
    let script_sources = if cfg!(debug_assertions) {
        "'self' 'unsafe-inline' 'wasm-unsafe-eval'".to_string()
    } else {
        let hash = hydration_script_hash(options);
        format!("'self' 'sha256-{hash}' 'wasm-unsafe-eval'")
    };
    let value = format!(
        "default-src 'self'; base-uri 'none'; object-src 'none'; frame-ancestors 'none'; form-action 'self'; img-src 'self' data:; connect-src 'self' ws: wss:; style-src 'self' 'unsafe-inline'; script-src {script_sources};"
    );
    axum::http::header::HeaderValue::from_str(&value)
        .map_err(|error| worker::Error::RustError(error.to_string()))
}

#[cfg(feature = "ssr")]
fn hydration_script_hash(options: &leptos::prelude::LeptosOptions) -> String {
    use base64::Engine;
    use sha2::{Digest, Sha256};

    let digest = Sha256::digest(hydration_script(options).as_bytes());
    base64::engine::general_purpose::STANDARD.encode(digest)
}

#[cfg(feature = "ssr")]
fn hydration_script(options: &leptos::prelude::LeptosOptions) -> String {
    let js_href = asset_href(options, "js", crate::asset_hashes::JS_HASH);
    let wasm_href = asset_href(options, "wasm", crate::asset_hashes::WASM_HASH);
    format!(
        "import({js_href:?}).then(mod => {{ mod.default({{ module_or_path: {wasm_href:?} }}).then(() => {{ mod.hydrate(); }}); }});"
    )
}

#[cfg(feature = "ssr")]
fn asset_href(options: &leptos::prelude::LeptosOptions, extension: &str, hash: &str) -> String {
    let output_name = options.output_name.as_ref();
    let pkg_dir = options.site_pkg_dir.as_ref();

    if hash.is_empty() {
        format!("/{pkg_dir}/{output_name}.{extension}")
    } else {
        format!("/{pkg_dir}/{output_name}.{hash}.{extension}")
    }
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

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
