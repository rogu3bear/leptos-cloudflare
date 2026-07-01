use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::all_languages;

#[component]
pub fn Home() -> impl IntoView {
    let langs = all_languages();

    let max_segments = langs.iter().map(|l| l.total_segments()).max().unwrap_or(0);
    let min_segments = langs.iter().map(|l| l.total_segments()).min().unwrap_or(0);
    let tonal_count = langs.iter().filter(|l| l.tones.is_some()).count();
    let featured: Vec<_> = ["taa", "rotokas", "piraha", "georgian", "ubykh", "mandarin"]
        .iter()
        .filter_map(|id| langs.iter().find(|l| &l.id == id))
        .collect();

    view! {
        <section class="hero">
            <div class="container hero-inner">
                <p class="eyebrow">"Vocal capability divergence"</p>
                <h1 class="display">
                    "The human voice "
                    <em>"is not one instrument."</em>
                </h1>
                <p class="lede">
                    "Every language is a choice — which sounds to contrast, which to let fall together. "
                    "Some languages work with a dozen phonemes. Others, more than a hundred. "
                    "This is a grounded-truth look at how far that range actually goes."
                </p>
                <div class="hero-cta">
                    <A href="/explore" attr:class="btn primary">"Explore the languages"</A>
                    <A href="/compare" attr:class="btn ghost">"Compare two side-by-side"</A>
                </div>
            </div>
        </section>

        <section class="container numbers">
            <Stat label="languages in the corpus" value=format!("{}", langs.len())/>
            <Stat label="smallest inventory (segments)" value=format!("{}", min_segments)/>
            <Stat label="largest inventory (segments)" value=format!("{}", max_segments)/>
            <Stat label="with contrastive tone" value=format!("{}/{}", tonal_count, langs.len())/>
        </section>

        <section class="container thesis">
            <h2>"Three axes of divergence"</h2>
            <div class="axis-grid">
                <article class="axis">
                    <span class="axis-mark" aria-hidden="true">"01"</span>
                    <h3>"Inventory size"</h3>
                    <p>
                        "Rotokas manages with six consonants; !Xóõ carries more than eighty, plus five click types. "
                        "Inventory size is not a measure of expressiveness — it's a measure of where a language spends its complexity budget."
                    </p>
                </article>
                <article class="axis">
                    <span class="axis-mark" aria-hidden="true">"02"</span>
                    <h3>"Airstream & phonation"</h3>
                    <p>
                        "Most languages use outward pulmonic air. A minority add ejectives (glottalic egressive), "
                        "implosives (glottalic ingressive), or clicks (velaric ingressive) — each recruits the vocal tract differently."
                    </p>
                </article>
                <article class="axis">
                    <span class="axis-mark" aria-hidden="true">"03"</span>
                    <h3>"Suprasegmental load"</h3>
                    <p>
                        "Vocal divergence is not only segments. Tone, length, stress, phonation and pitch-accent each do "
                        "phonological work in some languages and not others — Vietnamese carries six tones distinguished as much by voice quality as pitch."
                    </p>
                </article>
            </div>
        </section>

        <section class="container featured">
            <h2>"Start here"</h2>
            <p class="section-lede">"Six languages that mark the edges of the atlas."</p>
            <div class="featured-grid">
                {featured
                    .iter()
                    .map(|l| {
                        let href = format!("/language/{}", l.id);
                        let name = l.name;
                        let note = l.features.first().map(|f| f.name).unwrap_or("");
                        view! {
                            <A href=href attr:class="featured-tile">
                                <span class="featured-name">{name}</span>
                                <span class="featured-note">{note}</span>
                            </A>
                        }
                    })
                    .collect_view()
                }
            </div>
        </section>
    }
}

#[component]
fn Stat(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="stat">
            <div class="stat-value">{value}</div>
            <div class="stat-label">{label}</div>
        </div>
    }
}
