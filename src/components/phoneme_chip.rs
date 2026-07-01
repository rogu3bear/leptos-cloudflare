use leptos::prelude::*;

use crate::data::ipa::classify;

#[component]
pub fn PhonemeChip(#[prop(into)] symbol: String) -> impl IntoView {
    let kind = classify(&symbol);
    let class = format!("chip {}", kind.class_name());
    let label = kind.label();

    view! {
        <span class=class title=label>
            <span class="chip-glyph">{symbol}</span>
        </span>
    }
}
