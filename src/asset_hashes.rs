pub const JS_HASH: &str = match option_env!("LEPTOS_EDGE_JS_HASH") {
    Some(hash) => hash,
    None => "",
};

pub const WASM_HASH: &str = match option_env!("LEPTOS_EDGE_WASM_HASH") {
    Some(hash) => hash,
    None => "",
};

pub const CSS_HASH: &str = match option_env!("LEPTOS_EDGE_CSS_HASH") {
    Some(hash) => hash,
    None => "",
};
