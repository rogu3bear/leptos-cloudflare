//! Request routing for the Worker entry. No JavaScript value enters this
//! module, so every decision is covered by unit tests on the host target.

use axum::http::Uri;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

pub(super) const REALTIME_SOCKET_PATH: &str = "/realtime/socket";

const STATIC_ASSET_PATHS: [&str; 7] = [
    "/asset-manifest.json",
    "/app-icon.svg",
    "/app-icon-192.png",
    "/app-icon-512.png",
    "/apple-touch-icon.png",
    "/favicon.svg",
    "/site.webmanifest",
];

/// Pages uploads the server modules beside the static files. They are never
/// public, whatever spelling of the path a request uses.
const PRIVATE_MODULE_PREFIXES: [&str; 2] = ["/_server", "/functions"];

const SERVER_FUNCTION_PREFIX: &str = "/api/";

/// Build-generated routing data published by the generated `_worker.js`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct EdgeManifest {
    asset_prefix: String,
    /// Exact asset inventory. Pages sends every request to the Worker, so it
    /// needs the list; Workers Static Assets answers matches before the Worker.
    /// The key is required: `null` means "no inventory", absence is an error.
    #[serde(deserialize_with = "Option::deserialize")]
    asset_paths: Option<HashSet<String>>,
    /// Server-function URLs of retained clients, mapped to the active ones.
    aliases: HashMap<String, String>,
    block_server_modules: bool,
}

impl EdgeManifest {
    pub(super) fn from_json(json: &str) -> Result<Self, String> {
        let manifest: Self = serde_json::from_str(json).map_err(|error| error.to_string())?;
        let prefix = &manifest.asset_prefix;
        if prefix.len() < 2 || !prefix.starts_with('/') || !prefix.ends_with('/') {
            return Err(format!("asset prefix {prefix:?} is not a directory path"));
        }
        let outside_server_functions = |path: &String| !path.starts_with(SERVER_FUNCTION_PREFIX);
        if manifest.aliases.keys().any(outside_server_functions)
            || manifest.aliases.values().any(outside_server_functions)
        {
            return Err("aliases may only map server-function paths".to_owned());
        }
        Ok(manifest)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Route {
    /// A Pages-private Worker module: `404`.
    PrivateModule,
    /// A WebSocket upgrade for the template socket path.
    RealtimeSocket,
    /// A WebSocket upgrade for any other path: `404`.
    UnknownSocket,
    /// Plain HTTP on the socket path: `426`.
    UpgradeRequired,
    /// Answered by the `ASSETS` binding, untouched.
    Asset,
    /// Leptos SSR or a server function.
    Application {
        alias: Option<String>,
        isolate_body: bool,
    },
}

pub(super) fn is_websocket_upgrade(upgrade: Option<&str>) -> bool {
    upgrade.is_some_and(|value| value.eq_ignore_ascii_case("websocket"))
}

/// `pathname` is the URL path as parsed by the platform: dot segments are
/// already resolved and escapes are still encoded.
pub(super) fn decide(
    pathname: &str,
    websocket_upgrade: bool,
    has_body: bool,
    manifest: &EdgeManifest,
) -> Route {
    let decoded = decode_component(pathname);
    let decoded = decoded.as_deref();

    if manifest.block_server_modules
        && (is_private_module(pathname) || decoded.is_some_and(decodes_to_private_module))
    {
        return Route::PrivateModule;
    }

    if websocket_upgrade {
        return if pathname == REALTIME_SOCKET_PATH {
            Route::RealtimeSocket
        } else {
            Route::UnknownSocket
        };
    }
    if pathname == REALTIME_SOCKET_PATH {
        return Route::UpgradeRequired;
    }

    let application_path = decoded.is_some_and(|path| {
        path == "/api" || path.starts_with(SERVER_FUNCTION_PREFIX) || path == REALTIME_SOCKET_PATH
    });
    let inventoried_asset = !application_path
        && decoded.is_some_and(|path| {
            manifest
                .asset_paths
                .as_ref()
                .is_some_and(|paths| paths.contains(path))
        });
    if inventoried_asset
        || STATIC_ASSET_PATHS.contains(&pathname)
        || pathname.starts_with(&manifest.asset_prefix)
    {
        return Route::Asset;
    }

    Route::Application {
        alias: manifest.aliases.get(pathname).cloned(),
        isolate_body: has_body && pathname.starts_with(SERVER_FUNCTION_PREFIX),
    }
}

/// Replace the path of an absolute or origin-form URI, keeping its query.
pub(super) fn with_path(uri: &Uri, path: &str) -> Result<Uri, axum::http::Error> {
    let path_and_query = match uri.query() {
        Some(query) => format!("{path}?{query}"),
        None => path.to_owned(),
    };
    let mut parts = uri.clone().into_parts();
    parts.path_and_query = Some(path_and_query.parse()?);
    Ok(Uri::from_parts(parts)?)
}

fn is_private_module(path: &str) -> bool {
    PRIVATE_MODULE_PREFIXES.iter().any(|prefix| {
        path.strip_prefix(prefix)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    })
}

/// Whether a decoded path names a private module once separators are unified
/// and the URL parser has resolved it. An unparseable path is treated as private.
fn decodes_to_private_module(decoded: &str) -> bool {
    let mut unified = String::with_capacity(decoded.len());
    for character in decoded.chars() {
        let character = if character == '\\' { '/' } else { character };
        if character == '/' && unified.ends_with('/') {
            continue;
        }
        unified.push(character);
    }
    worker::Url::parse(&format!("https://assets.invalid{unified}"))
        .map_or(true, |url| is_private_module(url.path()))
}

/// Strict percent-decoding: a malformed escape or invalid UTF-8 yields `None`.
fn decode_component(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = hex_value(*bytes.get(index + 1)?)?;
            let low = hex_value(*bytes.get(index + 2)?)?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn hex_value(byte: u8) -> Option<u8> {
    char::from(byte)
        .to_digit(16)
        .and_then(|digit| u8::try_from(digit).ok())
}

#[cfg(test)]
mod tests;
