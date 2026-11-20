use super::*;

fn manifest(json: &str) -> EdgeManifest {
    EdgeManifest::from_json(json).expect("valid manifest fixture")
}

fn workers() -> EdgeManifest {
    manifest(r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{},"blockServerModules":false}"#)
}

fn pages() -> EdgeManifest {
    manifest(
        r#"{"assetPrefix":"/pkg/","assetPaths":["/logo.svg","/api/save","/realtime/socket","/my logo.svg"],"aliases":{},"blockServerModules":true}"#,
    )
}

fn legacy() -> EdgeManifest {
    manifest(
        r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{"/api/create_todo111":"/api/create_todo222"},"blockServerModules":false}"#,
    )
}

const fn application(isolate_body: bool) -> Route {
    Route::Application {
        alias: None,
        isolate_body,
    }
}

fn get(path: &str, manifest: &EdgeManifest) -> Route {
    decide(path, false, false, manifest)
}

#[test]
fn assets_leave_before_the_application() {
    let workers = workers();
    assert_eq!(get("/pkg/app.js", &workers), Route::Asset);
    assert_eq!(get("/pkg/", &workers), Route::Asset);
    // Spelled out here on purpose: removing an identity file from the routing
    // list must fail a test, not silently send that file to the application.
    let identity_files = [
        "/asset-manifest.json",
        "/app-icon.svg",
        "/app-icon-192.png",
        "/app-icon-512.png",
        "/apple-touch-icon.png",
        "/favicon.svg",
        "/site.webmanifest",
    ];
    assert_eq!(STATIC_ASSET_PATHS, identity_files);
    for path in identity_files {
        assert_eq!(get(path, &workers), Route::Asset, "{path}");
    }
    assert_eq!(get("/pkg", &workers), application(false));
    assert_eq!(get("/pkgs/app.js", &workers), application(false));
    assert_eq!(get("/favicon.svg/extra", &workers), application(false));
    assert_eq!(get("/new-route", &workers), application(false));
}

#[test]
fn only_server_function_bodies_are_isolated() {
    let workers = workers();
    assert_eq!(
        decide("/api/save", false, true, &workers),
        application(true)
    );
    assert_eq!(
        decide("/api/save", false, false, &workers),
        application(false)
    );
    assert_eq!(
        decide("/contact", false, true, &workers),
        application(false)
    );
    assert_eq!(decide("/api", false, true, &workers), application(false));
}

#[test]
fn the_socket_lane_is_one_exact_path() {
    let workers = workers();
    assert_eq!(get("/realtime/socket", &workers), Route::UpgradeRequired);
    assert_eq!(
        decide("/realtime/socket", true, false, &workers),
        Route::RealtimeSocket
    );
    for path in [
        "/wrong",
        "/realtime/socket/",
        "/realtime",
        "/pkg/app.js",
        "/api/save",
    ] {
        assert_eq!(
            decide(path, true, false, &workers),
            Route::UnknownSocket,
            "{path}"
        );
    }
    assert_eq!(get("/realtime/socket/", &workers), application(false));
}

#[test]
fn upgrade_detection_matches_the_header_value_exactly() {
    assert!(is_websocket_upgrade(Some("websocket")));
    assert!(is_websocket_upgrade(Some("WebSocket")));
    assert!(!is_websocket_upgrade(None));
    assert!(!is_websocket_upgrade(Some("")));
    assert!(!is_websocket_upgrade(Some("h2c")));
    assert!(!is_websocket_upgrade(Some("websocket, h2c")));
}

#[test]
fn pages_serves_only_inventoried_assets_outside_application_paths() {
    let pages = pages();
    assert_eq!(get("/logo.svg", &pages), Route::Asset);
    assert_eq!(get("/my%20logo.svg", &pages), Route::Asset);
    assert_eq!(get("/missing.svg", &pages), application(false));
    assert_eq!(get("/api/save", &pages), application(false));
    assert_eq!(get("/realtime/socket", &pages), Route::UpgradeRequired);
    assert_eq!(get("/pkg/app.js", &pages), Route::Asset);
    // The inventory is a Pages concern; Workers ignores an identical path.
    assert_eq!(get("/logo.svg", &workers()), application(false));
}

#[test]
fn pages_never_exposes_server_modules() {
    let pages = pages();
    for path in [
        "/_server",
        "/functions",
        "/_server/dispatch.mjs",
        "/_server/index.js",
        "/functions/index_bg.wasm",
        "/pkg/%2f..%2f_server/index_bg.wasm",
        "/pkg/%2f..%2ffunctions/index_bg.wasm",
        "/pkg/%2F..%2F_server",
        "/pkg/%5c..%5c_server/index.js",
        "/pkg/%2f%2f..%2f%2ffunctions/index.js",
        "/pkg/%252e%252e/_server/index.js",
        "/%2f_server/index.js",
        "/%5f%73erver/index.js",
        "/_ser%09ver/index.js",
        "/_server%20",
    ] {
        assert_eq!(get(path, &pages), Route::PrivateModule, "{path}");
        assert_eq!(
            decide(path, true, false, &pages),
            Route::PrivateModule,
            "{path} upgrade"
        );
    }
    for path in [
        "/_serverless",
        "/functions.js",
        "/app/_server/index.js",
        "/functional",
    ] {
        assert_eq!(get(path, &pages), application(false), "{path}");
    }
}

#[test]
fn workers_has_no_private_module_namespace() {
    let workers = workers();
    assert_eq!(get("/_server/index.js", &workers), application(false));
    assert_eq!(get("/functions/index.js", &workers), application(false));
}

#[test]
fn undecodable_paths_are_never_inventory_matches() {
    let pages = pages();
    assert_eq!(get("/%zz", &pages), application(false));
    assert_eq!(get("/logo.svg%", &pages), application(false));
    assert_eq!(get("/%ff", &pages), application(false));
    assert_eq!(get("/%c0%af", &pages), application(false));
    assert_eq!(get("/pkg/%zz", &pages), Route::Asset);
    assert_eq!(decode_component("/a%2Fb%20c"), Some("/a/b c".to_owned()));
    assert_eq!(decode_component("/%E2%9C%93"), Some("/\u{2713}".to_owned()));
    assert_eq!(decode_component("/%E2%9C"), None);
    assert_eq!(decode_component("/%2"), None);
}

#[test]
fn retired_server_function_urls_reach_the_active_function() {
    let legacy = legacy();
    assert_eq!(
        decide("/api/create_todo111", false, true, &legacy),
        Route::Application {
            alias: Some("/api/create_todo222".to_owned()),
            isolate_body: true,
        }
    );
    assert_eq!(get("/api/create_todo333", &legacy), application(false));
    assert_eq!(get("/api/create_todo222", &legacy), application(false));
    assert_eq!(get("/api/create_todo111/", &legacy), application(false));
}

#[test]
fn an_alias_replaces_only_the_path() {
    let absolute: Uri = "https://test/api/create_todo111?q=x".parse().unwrap();
    assert_eq!(
        with_path(&absolute, "/api/create_todo222")
            .unwrap()
            .to_string(),
        "https://test/api/create_todo222?q=x"
    );
    let bare: Uri = "http://127.0.0.1:8787/api/create_todo111".parse().unwrap();
    assert_eq!(
        with_path(&bare, "/api/create_todo222").unwrap().to_string(),
        "http://127.0.0.1:8787/api/create_todo222"
    );
    assert!(with_path(&absolute, "/api/bad path").is_err());
}

#[test]
fn the_manifest_is_a_closed_contract() {
    for (label, json) in [
        ("empty", "{}"),
        ("null", "null"),
        (
            "missing aliases",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"blockServerModules":false}"#,
        ),
        (
            "missing denial flag",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{}}"#,
        ),
        (
            "missing inventory",
            r#"{"assetPrefix":"/pkg/","aliases":{},"blockServerModules":true}"#,
        ),
        (
            "unknown field",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{},"blockServerModules":false,"extra":1}"#,
        ),
        (
            "empty prefix",
            r#"{"assetPrefix":"","assetPaths":null,"aliases":{},"blockServerModules":false}"#,
        ),
        (
            "root prefix",
            r#"{"assetPrefix":"/","assetPaths":null,"aliases":{},"blockServerModules":false}"#,
        ),
        (
            "open prefix",
            r#"{"assetPrefix":"/pkg","assetPaths":null,"aliases":{},"blockServerModules":false}"#,
        ),
        (
            "alias from a page",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{"/contact":"/api/x"},"blockServerModules":false}"#,
        ),
        (
            "alias to a page",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{"/api/x":"/contact"},"blockServerModules":false}"#,
        ),
        (
            "truthy denial flag",
            r#"{"assetPrefix":"/pkg/","assetPaths":null,"aliases":{},"blockServerModules":1}"#,
        ),
    ] {
        assert!(EdgeManifest::from_json(json).is_err(), "{label}");
    }
    let adopted = manifest(
        r#"{"assetPrefix":"/static/","assetPaths":null,"aliases":{},"blockServerModules":false}"#,
    );
    assert_eq!(get("/static/app.js", &adopted), Route::Asset);
    assert_eq!(get("/pkg/app.js", &adopted), application(false));
}
