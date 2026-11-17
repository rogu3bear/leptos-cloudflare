use super::policy::MAX_SERVER_FUNCTION_BODY_BYTES;
use axum::{
    body::{to_bytes, Body},
    http::{Request, Response, StatusCode},
};
use std::error::Error;

pub(super) async fn bounded_api_request(
    req: worker::HttpRequest,
) -> Result<Request<Body>, Response<Body>> {
    let req = req.map(Body::new);
    if !req.uri().path().starts_with("/api/") {
        return Ok(req);
    }
    let (parts, body) = req.into_parts();
    match to_bytes(body, MAX_SERVER_FUNCTION_BODY_BYTES).await {
        Ok(bytes) => Ok(Request::from_parts(parts, Body::from(bytes))),
        Err(error) => {
            let too_large = error
                .source()
                .is_some_and(|source| source.is::<http_body_util::LengthLimitError>());
            let (status, message) = if too_large {
                (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "Request payload exceeds the demo limit.",
                )
            } else {
                (StatusCode::BAD_REQUEST, "Request body could not be read.")
            };
            let mut response = Response::new(Body::from(message));
            *response.status_mut() = status;
            Err(response)
        }
    }
}
