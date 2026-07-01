use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::SignatureRow;
use crate::data::{all_languages, find_language, Language};

#[component]
pub fn Compare() -> impl IntoView {
    let query = use_query_map();
    let default_a = query
        .read_untracked()
        .get("a")
        .unwrap_or_else(|| "english".to_string());
    let default_b = query
        .read_untracked()
        .get("b")
        .unwrap_or_else(|| "taa".to_string());

    let a_id = RwSignal::new(default_a);
    let b_id = RwSignal::new(default_b);

    let a_lang = Memo::new(move |_| find_language(&a_id.get()));
    let b_lang = Memo::new(move |_| find_language(&b_id.get()));

    view! {
        <section class="container compare">
            <header class="page-head">
                <p class="eyebrow">"Compare"</p>
                <h1 class="display small">"Two languages, side by side."</h1>
                <p class="lede">
                    "Pick any pair. The shape of the difference — segment count, tone, signature sounds — is the point."
                </p>
            </header>

            <div class="picker-row">
                <LanguagePicker
                    label="Left".to_string()
                    value=a_id
                />
                <div class="versus" aria-hidden="true">"⇌"</div>
                <LanguagePicker
                    label="Right".to_string()
                    value=b_id
                />
            </div>

            <div class="compare-grid">
                {move || {
                    let a = a_lang.get();
                    let b = b_lang.get();
                    view! {
                        <ComparePanel side="left" lang=a/>
                        <ComparePanel side="right" lang=b/>
                    }
                }}
            </div>

            <div class="compare-summary">
                {move || {
                    match (a_lang.get(), b_lang.get()) {
                        (Some(a), Some(b)) => view! { <Summary a=a b=b/> }.into_any(),
                        _ => view! { <p class="muted">"Select two languages to see the delta."</p> }.into_any(),
                    }
                }}
            </div>
        </section>
    }
}

#[component]
fn LanguagePicker(label: String, value: RwSignal<String>) -> impl IntoView {
    let langs = all_languages();
    view! {
        <label class="field grow">
            <span class="field-label">{label}</span>
            <select
                prop:value=move || value.get()
                on:change=move |ev| value.set(event_target_value(&ev))
            >
                {langs
                    .iter()
                    .map(|l| view! {
                        <option value={l.id}>
                            {format!("{} — {}", l.name, l.family)}
                        </option>
                    })
                    .collect_view()
                }
            </select>
        </label>
    }
}

#[component]
fn ComparePanel(side: &'static str, lang: Option<&'static Language>) -> impl IntoView {
    match lang {
        None => view! { <div class=format!("panel panel-{side} missing")><p>"Pick a language."</p></div> }.into_any(),
        Some(l) => {
            let tone_line = l
                .tones
                .map(|t| format!("{t} tones"))
                .unwrap_or_else(|| "no contrastive tone".to_string());
            view! {
                <article class=format!("panel panel-{side}")>
                    <header class="panel-head">
                        <p class="eyebrow">{l.family}</p>
                        <h2 class="panel-name">
                            <span class="endonym">{l.endonym}</span>
                            <span class="english">{l.name}</span>
                        </h2>
                    </header>
                    <dl class="panel-stats">
                        <div><dt>"Consonants"</dt><dd>{l.consonant_count}</dd></div>
                        <div><dt>"Vowels"</dt><dd>{l.vowel_count}</dd></div>
                        <div><dt>"Tones"</dt><dd>{tone_line}</dd></div>
                        <div><dt>"Total"</dt><dd>{l.total_segments()}</dd></div>
                    </dl>
                    <h3 class="panel-sub">"Signature phonemes"</h3>
                    <SignatureRow symbols=l.signature_phonemes/>
                    <h3 class="panel-sub">"Notable features"</h3>
                    <ul class="panel-features">
                        {l.features.iter().map(|f| view! {
                            <li>
                                <strong>{f.name}</strong>
                                <span>{f.description}</span>
                            </li>
                        }).collect_view()}
                    </ul>
                </article>
            }.into_any()
        }
    }
}

#[component]
fn Summary(a: &'static Language, b: &'static Language) -> impl IntoView {
    let delta = (a.total_segments() as i32) - (b.total_segments() as i32);
    let abs = delta.unsigned_abs();
    let direction = if delta == 0 {
        format!("{} and {} are matched on segment count.", a.name, b.name)
    } else if delta > 0 {
        format!("{} carries {} more segments than {}.", a.name, abs, b.name)
    } else {
        format!("{} carries {} more segments than {}.", b.name, abs, a.name)
    };

    let tone_line = match (a.tones, b.tones) {
        (Some(ta), Some(tb)) => format!(
            "{} distinguishes {} tones; {} distinguishes {}.",
            a.name, ta, b.name, tb
        ),
        (Some(ta), None) => format!("{} is tonal ({} tones); {} is not.", a.name, ta, b.name),
        (None, Some(tb)) => format!("{} is tonal ({} tones); {} is not.", b.name, tb, a.name),
        (None, None) => format!("Neither {} nor {} use lexical tone.", a.name, b.name),
    };

    view! {
        <h2>"Where they diverge"</h2>
        <p class="summary-line">{direction}</p>
        <p class="summary-line">{tone_line}</p>
    }
}
