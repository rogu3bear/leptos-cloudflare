use leptos::prelude::*;

use crate::components::LanguageCard;
use crate::data::{all_languages, Language};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sort {
    Alphabetical,
    SegmentsAsc,
    SegmentsDesc,
}

#[component]
pub fn Explorer() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let macroarea = RwSignal::new(String::from("all"));
    let feature = RwSignal::new(String::from("all"));
    let sort = RwSignal::new(Sort::Alphabetical);

    let langs = all_languages();
    let macroareas = collect_macroareas(langs);

    let filtered = Memo::new(move |_| {
        let q_raw = query.get().to_lowercase();
        let q = q_raw.trim().to_string();
        let area = macroarea.get();
        let feat = feature.get();
        let s = sort.get();

        let mut out: Vec<&'static Language> = langs
            .iter()
            .filter(|l| match q.as_str() {
                "" => true,
                term => {
                    l.name.to_lowercase().contains(term)
                        || l.endonym.to_lowercase().contains(term)
                        || l.family.to_lowercase().contains(term)
                        || l.iso639_3.contains(term)
                }
            })
            .filter(|l| area == "all" || l.macroarea == area)
            .filter(|l| matches_feature(l, &feat))
            .collect();

        match s {
            Sort::Alphabetical => out.sort_by_key(|l| l.name),
            Sort::SegmentsAsc => out.sort_by_key(|l| l.total_segments()),
            Sort::SegmentsDesc => out.sort_by_key(|l| std::cmp::Reverse(l.total_segments())),
        }
        out
    });

    view! {
        <section class="container explore">
            <header class="page-head">
                <p class="eyebrow">"Explore"</p>
                <h1 class="display small">"Twenty languages, plotted by phonological signature."</h1>
                <p class="lede">
                    "Search by name or family. Filter by region or by a specific phonological feature. "
                    "Sort by segment count to see the full range from Rotokas (16) to !Xóõ (118)."
                </p>
            </header>

            <div class="filter-bar" role="search">
                <label class="field grow">
                    <span class="field-label">"Search"</span>
                    <input
                        type="search"
                        placeholder="English, Yoruba, stan1293, Bantu…"
                        prop:value=move || query.get()
                        on:input=move |ev| query.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field-label">"Region"</span>
                    <select
                        prop:value=move || macroarea.get()
                        on:change=move |ev| macroarea.set(event_target_value(&ev))
                    >
                        <option value="all">"All regions"</option>
                        {macroareas
                            .iter()
                            .map(|m| view! { <option value={*m}>{*m}</option> })
                            .collect_view()
                        }
                    </select>
                </label>
                <label class="field">
                    <span class="field-label">"Feature"</span>
                    <select
                        prop:value=move || feature.get()
                        on:change=move |ev| feature.set(event_target_value(&ev))
                    >
                        <option value="all">"Any"</option>
                        <option value="tone">"Lexical tone"</option>
                        <option value="click">"Clicks"</option>
                        <option value="ejective">"Ejectives"</option>
                        <option value="implosive">"Implosives"</option>
                        <option value="small">"Small inventory (≤ 20 segments)"</option>
                        <option value="large">"Large inventory (≥ 60 segments)"</option>
                    </select>
                </label>
                <label class="field">
                    <span class="field-label">"Sort"</span>
                    <select
                        on:change=move |ev| {
                            sort.set(match event_target_value(&ev).as_str() {
                                "segments-asc" => Sort::SegmentsAsc,
                                "segments-desc" => Sort::SegmentsDesc,
                                _ => Sort::Alphabetical,
                            });
                        }
                    >
                        <option value="alpha">"A → Z"</option>
                        <option value="segments-desc">"Most segments first"</option>
                        <option value="segments-asc">"Fewest segments first"</option>
                    </select>
                </label>
            </div>

            <p class="result-count">
                {move || {
                    let n = filtered.with(|f| f.len());
                    format!("{n} {}", if n == 1 { "language" } else { "languages" })
                }}
            </p>

            <div class="card-grid">
                {move || filtered.with(|list| {
                    list.iter()
                        .map(|l| view! { <LanguageCard lang={*l}/> })
                        .collect_view()
                })}
            </div>
        </section>
    }
}

fn collect_macroareas(langs: &'static [Language]) -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for l in langs {
        if !seen.contains(&l.macroarea) {
            seen.push(l.macroarea);
        }
    }
    seen.sort();
    seen
}

fn matches_feature(lang: &Language, feat: &str) -> bool {
    match feat {
        "all" => true,
        "tone" => lang.tones.is_some(),
        "click" => lang.signature_phonemes.iter().any(|p| {
            p.contains('ǀ')
                || p.contains('ǁ')
                || p.contains('ǃ')
                || p.contains('ǂ')
                || p.contains('ʘ')
        }),
        "ejective" => lang
            .signature_phonemes
            .iter()
            .any(|p| p.contains('ʼ') || p.ends_with('\'')),
        "implosive" => lang
            .signature_phonemes
            .iter()
            .any(|p| matches!(*p, "ɓ" | "ɗ" | "ʄ" | "ɠ" | "ʛ")),
        "small" => lang.total_segments() <= 20,
        "large" => lang.total_segments() >= 60,
        _ => true,
    }
}
