const SESSION_COOKIE_NAME: &str = "leptos_cf_session";
const SESSION_COOKIE_MAX_AGE_SECONDS: u32 = 60 * 60 * 24 * 30;
const SESSION_ID_BYTES: usize = 32;

#[derive(Clone)]
pub(super) struct RequestIdentity {
    secure_cookie: bool,
    pub(super) session_id: String,
    pub(super) client_ip: Option<String>,
    pub(super) set_cookie: bool,
}

pub(super) fn request_identity(req: &worker::HttpRequest) -> worker::Result<RequestIdentity> {
    let session_id = request_session_id(req);
    Ok(match session_id {
        Some(session_id) => RequestIdentity {
            secure_cookie: is_secure_request(req),
            client_ip: client_ip(req),
            session_id,
            set_cookie: false,
        },
        None => RequestIdentity {
            secure_cookie: is_secure_request(req),
            client_ip: client_ip(req),
            session_id: random_session_id()?,
            set_cookie: true,
        },
    })
}

fn request_session_id(req: &worker::HttpRequest) -> Option<String> {
    req.headers()
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .and_then(parse_session_cookie)
}

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

fn valid_session_id(value: &str) -> bool {
    value.len() == SESSION_ID_BYTES * 2
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn random_session_id() -> worker::Result<String> {
    let mut bytes = [0u8; SESSION_ID_BYTES];
    getrandom::fill(&mut bytes).map_err(|error| worker::Error::RustError(error.to_string()))?;
    Ok(hex_encode(&bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

pub(super) fn session_cookie_header_value(
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

fn is_secure_request(req: &worker::HttpRequest) -> bool {
    req.uri().scheme_str() == Some("https")
        || req
            .headers()
            .get("x-forwarded-proto")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.eq_ignore_ascii_case("https"))
}

fn client_ip(req: &worker::HttpRequest) -> Option<String> {
    req.headers()
        .get("cf-connecting-ip")?
        .to_str()
        .ok()?
        .parse::<std::net::IpAddr>()
        .ok()
        .map(|ip| ip.to_string())
}
