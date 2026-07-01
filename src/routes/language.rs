use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::SignatureRow;
use crate::data::{find_language, Language};

#[component]
pub fn LanguageDetail() -> impl IntoView {
    let params = use_params_map();
    let lang = Memo::new(move |_| params.read().get("id").and_then(|id| find_language(&id)));

    view! {
        <section class="container detail">
            {move || match lang.get() {
                Some(l) => view! { <Detail lang=l/> }.into_any(),
                None => view! {
                    <div class="missing">
                        <h1>"Language not found."</h1>
                        <p>"That identifier isn't in the corpus yet."</p>
                        <A href="/explore" attr:class="btn ghost">"Back to explorer"</A>
                    </div>
                }.into_any(),
            }}
        </section>
    }
}

#[component]
fn Detail(lang: &'static Language) -> impl IntoView {
    let tone_block = lang.tones.map(|t| {
        view! {
            <div class="pill-row">
                <span class="pill pill-tone">{format!("{t} contrastive tones")}</span>
            </div>
        }
    });

    let compare_href = format!("/compare?a={}", lang.id);

    view! {
        <nav class="crumbs" aria-label="Breadcrumb">
            <A href="/explore">"Explore"</A>
            <span class="crumb-sep" aria-hidden="true">"·"</span>
            <span aria-current="page">{lang.name}</span>
        </nav>

        <header class="detail-head">
            <p class="eyebrow">{lang.family}</p>
            <h1 class="display small">
                <span class="endonym">{lang.endonym}</span>
                <span class="english">{lang.name}</span>
            </h1>
            <p class="lede">{lang.summary}</p>

            <dl class="meta-grid">
                <div><dt>"ISO 639-3"</dt><dd class="mono">{lang.iso639_3}</dd></div>
                <div><dt>"Glottocode"</dt><dd class="mono">{lang.glottocode}</dd></div>
                <div><dt>"Region"</dt><dd>{lang.region}</dd></div>
                <div><dt>"Speakers"</dt><dd>{lang.speakers}</dd></div>
            </dl>
        </header>

        <section class="detail-inventory">
            <h2>"Phoneme inventory"</h2>
            <div class="inventory-grid">
                <Tile label="Consonants" value=lang.consonant_count/>
                <Tile label="Vowels" value=lang.vowel_count/>
                <Tile label="Total segments" value=lang.total_segments()/>
            </div>
            {tone_block}
        </section>

        <section class="detail-signature">
            <h2>"Signature phonemes"</h2>
            <p class="section-lede">
                "A selection of sounds that typify this language — not a complete inventory. "
                "Consult the cited source or "
                {lang.phoible_url.map(|u| view! {
                    <>
                        <a href=u rel="noopener">"PHOIBLE"</a>
                        " for the full phoneme set."
                    </>
                })}
            </p>
            <SignatureRow symbols=lang.signature_phonemes/>
        </section>

        <section class="detail-features">
            <h2>"Notable features"</h2>
            <ul class="feature-list">
                {lang.features.iter().map(|f| view! {
                    <li class="feature">
                        <h3>{f.name}</h3>
                        <p>{f.description}</p>
                    </li>
                }).collect_view()}
            </ul>
        </section>

        <section class="detail-sources">
            <h2>"Sources"</h2>
            <ul class="source-list">
                {lang.sources.iter().map(|s| view! {
                    <li>
                        {match s.url {
                            Some(u) => view! { <a href=u rel="noopener">{s.label}</a> }.into_any(),
                            None => view! { <span>{s.label}</span> }.into_any(),
                        }}
                    </li>
                }).collect_view()}
            </ul>
        </section>

        <div class="detail-cta">
            <A href=compare_href attr:class="btn primary">"Compare with another language"</A>
            <A href="/explore" attr:class="btn ghost">"Back to explorer"</A>
        </div>
    }
}

#[component]
fn Tile(#[prop(into)] label: String, value: u16) -> impl IntoView {
    view! {
        <div class="tile">
            <div class="tile-value">{value}</div>
            <div class="tile-label">{label}</div>
        </div>
    }
}
