#[cfg(feature = "ssr")]
mod body;
#[cfg(feature = "ssr")]
mod headers;
mod policy;
#[cfg(feature = "ssr")]
mod session;
#[cfg(feature = "ssr")]
mod shell;

#[cfg(feature = "ssr")]
use self::{
    headers::{apply_response_headers, content_security_policy},
    policy::{reject_api_request, server_fn_body_too_large},
    session::request_identity,
};

#[cfg(feature = "ssr")]
pub(crate) async fn handle(
    req: worker::HttpRequest,
    env: worker::Env,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    use axum::body::Body;
    use axum::http::{Response, StatusCode};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_service::Service;

    let conf =
        get_configuration(None).map_err(|error| worker::Error::RustError(error.to_string()))?;
    let leptos_options = conf.leptos_options;
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

    let req = match body::bounded_api_request(req).await {
        Ok(req) => req,
        Err(mut response) => {
            apply_response_headers(&mut response, &content_security_policy, &request_identity)?;
            return Ok(response);
        }
    };

    let routes = generate_route_list(crate::app::App);
    let state = crate::server::AppState::new(
        leptos_options.clone(),
        env,
        request_identity.session_id.clone(),
        request_identity.client_ip.clone(),
    );

    let mut router = Router::new()
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let request_nonce = request_nonce.clone();
                move || provide_context(request_nonce.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell::shell(leptos_options.clone())
            },
        )
        .with_state(state);

    let mut response = router.call(req).await?;
    apply_response_headers(&mut response, &content_security_policy, &request_identity)?;

    Ok(response)
}
