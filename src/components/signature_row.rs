use leptos::prelude::*;

use crate::components::PhonemeChip;

#[component]
pub fn SignatureRow(symbols: &'static [&'static str]) -> impl IntoView {
    view! {
        <div class="signature-row" role="list" aria-label="Signature phonemes">
            {symbols
                .iter()
                .map(|s| view! {
                    <div role="listitem">
                        <PhonemeChip symbol={*s}/>
                    </div>
                })
                .collect_view()
            }
        </div>
    }
}
