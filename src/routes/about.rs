use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="container about">
            <header class="page-head">
                <p class="eyebrow">"About"</p>
                <h1 class="display small">"Method, sources, and the limits of this atlas."</h1>
            </header>

            <div class="prose">
                <h2>"What this is"</h2>
                <p>
                    "Langual is a reading-room sized atlas of phonological divergence — a hand-picked set of twenty languages "
                    "chosen to span the extremes of the world's sound systems. Every number is drawn from published phonological "
                    "reference works, not from the web."
                </p>

                <h2>"How the corpus was chosen"</h2>
                <p>
                    "The corpus is deliberately small. Twenty entries let each one carry its own source citation and commentary. "
                    "The selection covers the recognized macroareas (Africa, Eurasia, Papunesia, the Americas) and crosses multiple "
                    "typological axes: smallest and largest inventories, clicks, ejectives, implosives, pharyngeals, "
                    "retroflex series, vowel harmony, and tone (both pitch and phonation)."
                </p>

                <h2>"Primary sources"</h2>
                <ul>
                    <li>
                        <a href="https://phoible.org" rel="noopener">"PHOIBLE 2.0"</a>
                        " — Moran, S. & McCloy, D. (eds.) 2019. Max Planck Institute for the Science of Human History. "
                        "The aggregate phoneme-inventory database used to cross-check segment counts."
                    </li>
                    <li>
                        "Maddieson, I. (1984). "
                        <em>"Patterns of Sounds."</em>
                        " Cambridge University Press."
                    </li>
                    <li>
                        "Ladefoged, P. & Maddieson, I. (1996). "
                        <em>"The Sounds of the World's Languages."</em>
                        " Blackwell."
                    </li>
                    <li>
                        <a href="https://wals.info" rel="noopener">"WALS Online"</a>
                        " — Dryer, M. & Haspelmath, M. (eds.) 2013. Max Planck Institute."
                    </li>
                    <li>
                        "Per-language references (e.g., Traill 1985, Vance 2008, Young & Morgan 1987) are listed on each language page."
                    </li>
                </ul>

                <h2>"What is deliberately excluded"</h2>
                <p>
                    "This site does not attempt to reproduce a full phoneme chart for every language. "
                    "Phonological inventories are not fixed objects — different analyses of the same language can differ by a dozen phonemes. "
                    "We give a single published count and a short list of signature phonemes, with links to PHOIBLE for canonical inventory data."
                </p>

                <h2>"Corrections"</h2>
                <p>
                    "This is an evolving reference. If a number, citation, or feature description is wrong, the authoritative "
                    "answer lives in the cited source — follow the link and write in."
                </p>

                <h2>"Built with"</h2>
                <p>
                    "Rust, Leptos, WebAssembly. Deployed as a static bundle on Cloudflare Pages. "
                    "No runtime database, no telemetry, no third-party fonts."
                </p>
            </div>
        </section>
    }
}
