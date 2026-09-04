use serde::Serialize;

const TELEMETRY_SCHEMA_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RequestBoundary {
    Ssr,
    ServerFunction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RequestOutcome {
    Success,
    ClientError,
    ServerError,
    Exception,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum MethodFamily {
    Get,
    Head,
    Post,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RouteFamily {
    Home,
    Start,
    Architecture,
    Patterns,
    Lab,
    LabDetail,
    About,
    Contact,
    NotFound,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ServerFunction {
    ListTodos,
    CreateTodo,
    ToggleTodo,
    DeleteTodo,
    GetTodo,
    SubmitContact,
    Unknown,
}

#[derive(Debug, Serialize)]
struct CompletionEvent {
    schema_version: u8,
    event: &'static str,
    boundary: RequestBoundary,
    route_family: Option<RouteFamily>,
    server_function: Option<ServerFunction>,
    method: MethodFamily,
    outcome: RequestOutcome,
    status_code: Option<u16>,
    duration_ms: u64,
}

/// One low-cardinality completion observation for the request that reaches
/// Leptos. Static assets and the template WebSocket lane terminate in the
/// generated JavaScript shim and therefore never enter this contract.
pub struct RequestObservation {
    boundary: RequestBoundary,
    route_family: Option<RouteFamily>,
    server_function: Option<ServerFunction>,
    method: MethodFamily,
    started_at_ms: u64,
}

impl RequestObservation {
    #[must_use]
    pub fn start(path: &str, method: &str) -> Self {
        let (boundary, route_family, server_function) = classify_request(path);
        Self {
            boundary,
            route_family,
            server_function,
            method: classify_method(method),
            started_at_ms: now_millis(),
        }
    }

    #[cfg(feature = "ssr")]
    pub fn finish(self, status_code: Option<u16>, exception: bool) {
        let event = self.event(status_code, exception, now_millis());
        let line = serde_json::to_string(&event).unwrap_or_else(|_| {
            r#"{"schema_version":1,"event":"leptos_cf.telemetry_encoding_failed"}"#.to_owned()
        });
        worker::console_log!("{}", line);
    }

    fn event(
        &self,
        status_code: Option<u16>,
        exception: bool,
        finished_at_ms: u64,
    ) -> CompletionEvent {
        CompletionEvent {
            schema_version: TELEMETRY_SCHEMA_VERSION,
            event: "leptos_cf.request.complete",
            boundary: self.boundary,
            route_family: self.route_family,
            server_function: self.server_function,
            method: self.method,
            outcome: classify_outcome(status_code, exception),
            status_code,
            duration_ms: finished_at_ms.saturating_sub(self.started_at_ms),
        }
    }
}

fn classify_request(path: &str) -> (RequestBoundary, Option<RouteFamily>, Option<ServerFunction>) {
    if path.starts_with("/api/") {
        return (
            RequestBoundary::ServerFunction,
            None,
            Some(classify_server_function(path)),
        );
    }

    (RequestBoundary::Ssr, Some(classify_route(path)), None)
}

fn classify_route(path: &str) -> RouteFamily {
    let normalized = path
        .strip_suffix('/')
        .filter(|without_trailing_slash| !without_trailing_slash.is_empty())
        .unwrap_or(path);
    match normalized {
        "/" => RouteFamily::Home,
        "/start" => RouteFamily::Start,
        "/architecture" => RouteFamily::Architecture,
        "/patterns" => RouteFamily::Patterns,
        "/lab" => RouteFamily::Lab,
        "/about" => RouteFamily::About,
        "/contact" => RouteFamily::Contact,
        _ if has_single_path_parameter(normalized, "/lab/")
            || has_single_path_parameter(normalized, "/todo/") =>
        {
            RouteFamily::LabDetail
        }
        _ => RouteFamily::NotFound,
    }
}

fn classify_server_function(path: &str) -> ServerFunction {
    let path = path.strip_suffix('/').unwrap_or(path);
    // Leptos appends its generated decimal u64 hash to these known function
    // names. Normalize only that suffix; arbitrary paths remain a closed Unknown.
    let name = path.trim_end_matches(|character: char| character.is_ascii_digit());
    let suffix = &path[name.len()..];
    if !suffix.is_empty() && (suffix.len() > 20 || suffix.parse::<u64>().is_err()) {
        return ServerFunction::Unknown;
    }
    match name {
        "/api/list_todos" => ServerFunction::ListTodos,
        "/api/create_todo" => ServerFunction::CreateTodo,
        "/api/toggle_todo" => ServerFunction::ToggleTodo,
        "/api/delete_todo" => ServerFunction::DeleteTodo,
        "/api/get_todo" => ServerFunction::GetTodo,
        "/api/submit_contact" => ServerFunction::SubmitContact,
        _ => ServerFunction::Unknown,
    }
}

fn has_single_path_parameter(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|parameter| !parameter.is_empty() && !parameter.contains('/'))
}

fn classify_method(method: &str) -> MethodFamily {
    match method {
        "GET" => MethodFamily::Get,
        "HEAD" => MethodFamily::Head,
        "POST" => MethodFamily::Post,
        _ => MethodFamily::Other,
    }
}

const fn classify_outcome(status_code: Option<u16>, exception: bool) -> RequestOutcome {
    if exception {
        return RequestOutcome::Exception;
    }
    match status_code {
        Some(400..=499) => RequestOutcome::ClientError,
        Some(500..=599) | None => RequestOutcome::ServerError,
        Some(_) => RequestOutcome::Success,
    }
}

#[cfg(all(feature = "ssr", not(test)))]
fn now_millis() -> u64 {
    worker::Date::now().as_millis()
}

#[cfg(test)]
fn now_millis() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event_for(path: &str, method: &str, status: Option<u16>) -> serde_json::Value {
        let observation = RequestObservation::start(path, method);
        serde_json::to_value(observation.event(status, false, 17)).expect("telemetry JSON")
    }

    #[test]
    fn route_and_server_function_dimensions_are_closed() {
        assert_eq!(
            event_for("/lab/private-id", "GET", Some(200))["route_family"],
            "lab_detail"
        );
        assert_eq!(
            event_for("/api/create_todo", "POST", Some(200))["server_function"],
            "create_todo"
        );
        assert_eq!(
            event_for("/api/private-value", "DELETE", Some(405))["server_function"],
            "unknown"
        );
        assert_eq!(
            event_for("/private-value", "GET", Some(404))["route_family"],
            "not_found"
        );
    }

    #[test]
    fn generated_function_hashes_preserve_closed_names() {
        // Observed in the built Worker/browser server-function URLs.
        for name in [
            "list_todos",
            "create_todo",
            "toggle_todo",
            "delete_todo",
            "get_todo",
            "submit_contact",
        ] {
            let path = format!("/api/{name}13596420688598566242");
            let event = event_for(&path, "POST", Some(200));
            assert_eq!(event["server_function"], name);
            assert!(!event.to_string().contains("13596420688598566242"));
        }
        for path in [
            "/api/delete_todo/123",
            "/api/delete_todo-private123",
            "/api/delete_todo123?secret=value",
            "/api/delete_todo１２３",
            "/api/delete_todo18446744073709551616",
            "/api/private123",
        ] {
            assert_eq!(
                event_for(path, "POST", Some(400))["server_function"],
                "unknown"
            );
        }
    }

    #[test]
    fn event_schema_excludes_request_and_identity_material() {
        let event = event_for(
            "/api/submit_contact?email=private@example.com",
            "POST",
            Some(500),
        );
        assert_eq!(event["schema_version"], TELEMETRY_SCHEMA_VERSION);
        assert_eq!(event["boundary"], "server_function");
        assert_eq!(event["outcome"], "server_error");
        let encoded = event.to_string();
        for forbidden in [
            "private@example.com",
            "query",
            "url",
            "path",
            "header",
            "cookie",
            "session",
            "database",
            "error_message",
        ] {
            assert!(!encoded.contains(forbidden), "telemetry leaked {forbidden}");
        }
    }

    #[test]
    fn status_and_exception_outcomes_are_explicit() {
        assert_eq!(classify_outcome(Some(204), false), RequestOutcome::Success);
        assert_eq!(
            classify_outcome(Some(422), false),
            RequestOutcome::ClientError
        );
        assert_eq!(
            classify_outcome(Some(503), false),
            RequestOutcome::ServerError
        );
        assert_eq!(classify_outcome(None, true), RequestOutcome::Exception);
    }
}
