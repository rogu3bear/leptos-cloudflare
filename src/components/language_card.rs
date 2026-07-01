use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::SignatureRow;
use crate::data::Language;

#[component]
pub fn LanguageCard(lang: &'static Language) -> impl IntoView {
    let href = format!("/language/{}", lang.id);
    let total = lang.total_segments();
    let tone_label = lang
        .tones
        .map(|t| format!("{t} tones"))
        .unwrap_or_else(|| "no contrastive tone".to_string());

    view! {
        <A href=href attr:class="card" attr:aria-label=format!("Open {}", lang.name)>
            <article class="card-inner">
                <header class="card-head">
                    <h3 class="card-name">
                        <span class="endonym">{lang.endonym}</span>
                        <span class="english">{lang.name}</span>
                    </h3>
                    <p class="card-family">{lang.family}</p>
                </header>

                <SignatureRow symbols=lang.signature_phonemes/>

                <dl class="card-stats">
                    <div>
                        <dt>"Consonants"</dt>
                        <dd>{lang.consonant_count}</dd>
                    </div>
                    <div>
                        <dt>"Vowels"</dt>
                        <dd>{lang.vowel_count}</dd>
                    </div>
                    <div>
                        <dt>"Total segments"</dt>
                        <dd>{total}</dd>
                    </div>
                </dl>

                <p class="card-tone">{tone_label}</p>
            </article>
        </A>
    }
}
