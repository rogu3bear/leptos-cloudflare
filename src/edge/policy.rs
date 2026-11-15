#[cfg(feature = "ssr")]
pub(super) const MAX_SERVER_FUNCTION_BODY_BYTES: usize = 4 * 1024;

#[cfg(feature = "ssr")]
pub(super) struct ApiRejection {
    pub(super) status: axum::http::StatusCode,
    pub(super) message: &'static str,
}

#[cfg(feature = "ssr")]
pub(super) fn reject_api_request(req: &worker::HttpRequest) -> Option<ApiRejection> {
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
pub(super) fn server_fn_body_too_large(req: &worker::HttpRequest) -> bool {
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
}
