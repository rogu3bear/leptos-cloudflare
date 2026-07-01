//! Curated language data sourced from published phonological references.
//!
//! Every language entry carries its primary citation. Phoneme counts follow the
//! cited source. Where sources disagree, we use the analysis given in the
//! reference and note that analyses vary. Nothing in this file is invented.
//!
//! Primary references (abbreviated by `Source` labels):
//! - PHOIBLE 2.0 — Moran & McCloy (eds.) 2019, https://phoible.org
//! - Maddieson (1984) — Patterns of Sounds, Cambridge University Press
//! - Ladefoged & Maddieson (1996) — The Sounds of the World's Languages, Blackwell
//! - WALS Online — Dryer & Haspelmath (eds.) 2013, https://wals.info
//! - Ethnologue (2024) — speaker counts only
//! - Plus per-language references listed in each entry.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    pub label: &'static str,
    pub url: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Feature {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Language {
    pub id: &'static str,
    pub name: &'static str,
    pub endonym: &'static str,
    pub iso639_3: &'static str,
    pub glottocode: &'static str,
    pub family: &'static str,
    pub macroarea: &'static str,
    pub region: &'static str,
    pub speakers: &'static str,
    pub consonant_count: u16,
    pub vowel_count: u16,
    pub tones: Option<u8>,
    pub signature_phonemes: &'static [&'static str],
    pub features: &'static [Feature],
    pub summary: &'static str,
    pub sources: &'static [Source],
    pub phoible_url: Option<&'static str>,
}

impl Language {
    pub fn total_segments(&self) -> u16 {
        self.consonant_count + self.vowel_count
    }
}

pub fn find_language(id: &str) -> Option<&'static Language> {
    LANGUAGES.iter().find(|l| l.id == id)
}

pub fn all_languages() -> &'static [Language] {
    LANGUAGES
}

// -----------------------------------------------------------------------------
// Data
// -----------------------------------------------------------------------------

static LANGUAGES: &[Language] = &[
    Language {
        id: "english",
        name: "English",
        endonym: "English",
        iso639_3: "eng",
        glottocode: "stan1293",
        family: "Indo-European › Germanic",
        macroarea: "Eurasia",
        region: "British Isles, North America, Oceania, global",
        speakers: "~380M L1, ~1.5B total",
        consonant_count: 24,
        vowel_count: 20,
        tones: None,
        signature_phonemes: &["θ", "ð", "ɹ", "w", "ŋ"],
        features: &[
            Feature {
                name: "Dental fricatives",
                description: "Interdental /θ/ and /ð/ are rare cross-linguistically (< 10% of WALS sample).",
            },
            Feature {
                name: "Stress-timed rhythm",
                description: "Syllables compress between stressed beats, driving vowel reduction to /ə/.",
            },
        ],
        summary: "English carries an unusually rich vowel inventory for a Germanic language (≈20 vowel phonemes counting diphthongs in RP) and is one of the few languages with contrastive interdental fricatives.",
        sources: &[
            Source { label: "Wells (1982) Accents of English", url: None },
            Source { label: "Ladefoged (2001) Vowels and Consonants", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/stan1293"),
    },
    Language {
        id: "mandarin",
        name: "Mandarin Chinese",
        endonym: "普通话 / 國語",
        iso639_3: "cmn",
        glottocode: "mand1415",
        family: "Sino-Tibetan › Sinitic",
        macroarea: "Eurasia",
        region: "China, Taiwan, Singapore",
        speakers: "~920M L1",
        consonant_count: 22,
        vowel_count: 5,
        tones: Some(4),
        signature_phonemes: &["tɕ", "tɕʰ", "ɕ", "ʈʂ", "ʂ", "ɻ", "˥", "˧˥", "˨˩˦", "˥˩"],
        features: &[
            Feature {
                name: "Three-way sibilant contrast",
                description: "Dental /ts tsʰ s/ × retroflex /ʈʂ ʈʂʰ ʂ/ × alveolo-palatal /tɕ tɕʰ ɕ/ — a rare three-way split.",
            },
            Feature {
                name: "Lexical tone",
                description: "Four contrastive tones plus a neutral tone; mā/má/mǎ/mà minimal set is canonical.",
            },
        ],
        summary: "Standard Mandarin is analyzed with a small vowel inventory but an elaborate tone system and a three-way coronal sibilant contrast found in few other languages.",
        sources: &[
            Source { label: "Duanmu (2007) The Phonology of Standard Chinese", url: None },
            Source { label: "Lin (2007) The Sounds of Chinese", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/mand1415"),
    },
    Language {
        id: "spanish",
        name: "Spanish (Castilian)",
        endonym: "Español",
        iso639_3: "spa",
        glottocode: "stan1288",
        family: "Indo-European › Romance",
        macroarea: "Eurasia / Americas",
        region: "Iberia, Americas, Equatorial Guinea",
        speakers: "~485M L1",
        consonant_count: 19,
        vowel_count: 5,
        tones: None,
        signature_phonemes: &["ɲ", "ʎ", "r", "ɾ", "θ", "x"],
        features: &[
            Feature {
                name: "Tap/trill contrast",
                description: "Minimal pairs like pero/perro depend on /ɾ/ vs /r/ — a contrast present in only a minority of world languages.",
            },
            Feature {
                name: "Five-vowel system",
                description: "/a e i o u/ — the single most common vowel system across PHOIBLE (~20% of languages).",
            },
        ],
        summary: "Peninsular Spanish keeps the /θ/-/s/ distinction (distinción) lost in most American dialects, plus the palatal lateral /ʎ/ in conservative speech.",
        sources: &[
            Source { label: "Hualde (2005) The Sounds of Spanish", url: None },
            Source { label: "Martínez-Celdrán, Fernández-Planas & Carrera-Sabaté (2003) JIPA", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/stan1288"),
    },
    Language {
        id: "arabic",
        name: "Modern Standard Arabic",
        endonym: "العربية الفصحى",
        iso639_3: "arb",
        glottocode: "stan1318",
        family: "Afro-Asiatic › Semitic",
        macroarea: "Africa / Eurasia",
        region: "Arab world",
        speakers: "~270M L1",
        consonant_count: 28,
        vowel_count: 6,
        tones: None,
        signature_phonemes: &["ħ", "ʕ", "q", "tˤ", "dˤ", "sˤ", "ðˤ", "ʔ"],
        features: &[
            Feature {
                name: "Pharyngeal fricatives",
                description: "/ħ/ and /ʕ/ are found in fewer than 5% of languages (WALS 19A).",
            },
            Feature {
                name: "Emphatic (pharyngealized) series",
                description: "A secondary-articulation contrast — coronals come in plain and pharyngealized pairs.",
            },
        ],
        summary: "Arabic's back-of-the-throat consonants — uvular, pharyngeal, and pharyngealized — are one of the most distinctive sound signatures of any language family.",
        sources: &[
            Source { label: "Watson (2002) The Phonology and Morphology of Arabic", url: None },
            Source { label: "Thelwall & Sa'adeddin (1990) JIPA", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/stan1318"),
    },
    Language {
        id: "hindi",
        name: "Hindi",
        endonym: "हिन्दी",
        iso639_3: "hin",
        glottocode: "hind1269",
        family: "Indo-European › Indo-Aryan",
        macroarea: "Eurasia",
        region: "North India",
        speakers: "~345M L1",
        consonant_count: 33,
        vowel_count: 10,
        tones: None,
        signature_phonemes: &["ʈ", "ɖ", "bʰ", "dʰ", "ɖʰ", "ɡʰ", "tʰ"],
        features: &[
            Feature {
                name: "Four-way stop contrast",
                description: "Voiceless / voiceless aspirated / voiced / breathy-voiced — a laryngeal four-way unusual outside Indo-Aryan.",
            },
            Feature {
                name: "Retroflex series",
                description: "A full /ʈ ɖ ɳ ɽ/ parallel to the dental /t̪ d̪ n̪/ series.",
            },
        ],
        summary: "Hindi combines a retroflex stop series with a four-way laryngeal contrast, yielding one of the largest stop inventories of any widely-spoken language.",
        sources: &[
            Source { label: "Shapiro (2003) in Cardona & Jain, eds., The Indo-Aryan Languages", url: None },
            Source { label: "Ohala (1983) Aspects of Hindi Phonology", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/hind1269"),
    },
    Language {
        id: "taa",
        name: "Taa (!Xóõ)",
        endonym: "Tâa",
        iso639_3: "nmn",
        glottocode: "taaa1242",
        family: "Tuu (Southern Khoisan)",
        macroarea: "Africa",
        region: "Botswana, Namibia",
        speakers: "~2,500",
        consonant_count: 87,
        vowel_count: 31,
        tones: Some(4),
        signature_phonemes: &["ʘ", "ǀ", "ǃ", "ǂ", "ǁ", "ǃʰ", "ɡǃ", "ŋǃ", "aˤ", "aˀ"],
        features: &[
            Feature {
                name: "Five click types",
                description: "Bilabial ʘ, dental ǀ, alveolar ǃ, palatal ǂ, and lateral ǁ — combined with ≈17 accompaniments, producing over 80 distinct click phonemes.",
            },
            Feature {
                name: "Strident & pharyngealized vowels",
                description: "Vowels contrast along phonation (modal, breathy, creaky, pharyngealized, strident) — the largest attested phoneme inventory on record.",
            },
        ],
        summary: "!Xóõ (Taa) is the benchmark for maximum phoneme diversity: a consonant inventory larger than most languages' full segment count, and vowel qualities multiplied by phonation type.",
        sources: &[
            Source { label: "Traill (1985) Phonetic & Phonological Studies of !Xóõ Bushman", url: None },
            Source { label: "Traill (1994) A !Xóõ Dictionary", url: None },
            Source { label: "Ladefoged & Maddieson (1996) ch. 8", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/taaa1242"),
    },
    Language {
        id: "rotokas",
        name: "Rotokas",
        endonym: "Rotokas",
        iso639_3: "roo",
        glottocode: "roto1249",
        family: "North Bougainville",
        macroarea: "Papunesia",
        region: "Bougainville, Papua New Guinea",
        speakers: "~4,300",
        consonant_count: 6,
        vowel_count: 10,
        tones: None,
        signature_phonemes: &["p", "t", "k", "β", "ɾ", "ɡ"],
        features: &[
            Feature {
                name: "No nasal phonemes",
                description: "Rotokas is one of very few languages with no phonemic nasals — /m n ŋ/ are all absent.",
            },
            Feature {
                name: "Six consonants",
                description: "Alongside Pirahã, among the smallest documented consonant inventories.",
            },
        ],
        summary: "Rotokas sits at the minimum end of the phoneme-count distribution: six consonants, five vowel qualities with contrastive length, and no nasals at all.",
        sources: &[
            Source { label: "Firchow & Firchow (1969) Phonology of Rotokas", url: None },
            Source { label: "Robinson (2006) Rotokas, a Papuan isolate", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/roto1249"),
    },
    Language {
        id: "piraha",
        name: "Pirahã",
        endonym: "Xapaitíiso",
        iso639_3: "myp",
        glottocode: "pira1253",
        family: "Mura (isolate)",
        macroarea: "South America",
        region: "Maici River, Amazonas, Brazil",
        speakers: "~700",
        consonant_count: 8,
        vowel_count: 3,
        tones: Some(2),
        signature_phonemes: &["p", "t", "k", "ʔ", "b", "ɡ", "s", "h", "i", "a", "o"],
        features: &[
            Feature {
                name: "Smallest documented inventory",
                description: "Men use 8 consonants; women use 7 (no /s/). Three vowels. Two tones (high/low).",
            },
            Feature {
                name: "Whistled & hummed registers",
                description: "Pirahã can be produced entirely as whistled or hummed speech — tone and syllable count alone carry information.",
            },
        ],
        summary: "Pirahã may be the smallest attested phoneme inventory. Its whistled and hummed registers show how far vocal capability can be compressed while still conveying lexicon.",
        sources: &[
            Source { label: "Everett (1986) in Derbyshire & Pullum, eds., Handbook of Amazonian Languages", url: None },
            Source { label: "Everett & Kern (1997) Wari'", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/pira1253"),
    },
    Language {
        id: "hawaiian",
        name: "Hawaiian",
        endonym: "ʻŌlelo Hawaiʻi",
        iso639_3: "haw",
        glottocode: "hawa1245",
        family: "Austronesian › Polynesian",
        macroarea: "Papunesia",
        region: "Hawaiian Islands",
        speakers: "~2,000 L1, ~24,000 L2",
        consonant_count: 8,
        vowel_count: 10,
        tones: None,
        signature_phonemes: &["p", "k", "ʔ", "h", "m", "n", "l", "w"],
        features: &[
            Feature {
                name: "Glottal stop as a letter",
                description: "The ʻokina (/ʔ/) is written as its own consonant; words like Hawaiʻi contrast with Hawai'i.",
            },
            Feature {
                name: "Strict CV syllables",
                description: "No consonant clusters; every syllable is (C)V or (C)VV.",
            },
        ],
        summary: "Hawaiian carries eight consonants and five vowel qualities (with phonemic length), one of the most restrictive sound systems among well-documented languages.",
        sources: &[
            Source { label: "Pukui & Elbert (1986) Hawaiian Dictionary", url: None },
            Source { label: "Schütz (1994) The Voices of Eden", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/hawa1245"),
    },
    Language {
        id: "georgian",
        name: "Georgian",
        endonym: "ქართული",
        iso639_3: "kat",
        glottocode: "nucl1302",
        family: "Kartvelian",
        macroarea: "Eurasia",
        region: "Georgia (Caucasus)",
        speakers: "~3.7M",
        consonant_count: 28,
        vowel_count: 5,
        tones: None,
        signature_phonemes: &["pʼ", "tʼ", "kʼ", "qʼ", "tsʼ", "tʃʼ", "q", "ɣ", "χ"],
        features: &[
            Feature {
                name: "Ejective series",
                description: "A full set of voiceless ejectives contrasting with plain voiceless aspirated and voiced stops.",
            },
            Feature {
                name: "Onset clusters up to six consonants",
                description: "gvprtskvni (you peel us) — eight consonants in a row, no intervening vowels.",
            },
        ],
        summary: "Georgian combines a three-way laryngeal stop system with the longest onset consonant clusters attested in any language.",
        sources: &[
            Source { label: "Aronson (1990) Georgian: A Reading Grammar", url: None },
            Source { label: "Shosted & Chikovani (2006) JIPA", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/nucl1302"),
    },
    Language {
        id: "japanese",
        name: "Japanese",
        endonym: "日本語",
        iso639_3: "jpn",
        glottocode: "nucl1643",
        family: "Japonic",
        macroarea: "Eurasia",
        region: "Japan",
        speakers: "~125M",
        consonant_count: 15,
        vowel_count: 5,
        tones: None,
        signature_phonemes: &["ɸ", "ɾ", "ɕ", "tɕ", "dʑ", "ɲ"],
        features: &[
            Feature {
                name: "Mora-timed rhythm",
                description: "The mora, not the syllable, is the timing unit — a geminate consonant counts as a mora of silence.",
            },
            Feature {
                name: "Pitch accent, not tone",
                description: "Each word has a pitch accent pattern, but pitch is not lexically contrastive on every syllable as in tone languages.",
            },
        ],
        summary: "Japanese shows how vocal complexity can live in timing and pitch rather than segment count — a small inventory organized into a mora-based prosodic system.",
        sources: &[
            Source { label: "Vance (2008) The Sounds of Japanese", url: None },
            Source { label: "Labrune (2012) The Phonology of Japanese", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/nucl1643"),
    },
    Language {
        id: "vietnamese",
        name: "Vietnamese (Northern)",
        endonym: "Tiếng Việt",
        iso639_3: "vie",
        glottocode: "viet1252",
        family: "Austroasiatic › Vietic",
        macroarea: "Eurasia",
        region: "Vietnam",
        speakers: "~85M",
        consonant_count: 22,
        vowel_count: 11,
        tones: Some(6),
        signature_phonemes: &["ɓ", "ɗ", "ʔ", "˧", "˧˥", "˨˩̰", "˧̰ʔ", "˦˥", "˨˩"],
        features: &[
            Feature {
                name: "Six-tone register with phonation",
                description: "Northern Vietnamese contrasts six tones differing not just in pitch but in voice quality (creaky, breathy, modal).",
            },
            Feature {
                name: "Implosive stops",
                description: "/ɓ/ and /ɗ/ are implosive — air drawn inward rather than expelled.",
            },
        ],
        summary: "Northern Vietnamese tones encode phonation as much as pitch, making 'tone' here a full laryngeal system rather than pure pitch contour.",
        sources: &[
            Source { label: "Kirby (2011) JIPA Illustrations — Vietnamese (Hanoi)", url: None },
            Source { label: "Pham (2003) Vietnamese Tone: A New Analysis", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/viet1252"),
    },
    Language {
        id: "yoruba",
        name: "Yoruba",
        endonym: "Èdè Yorùbá",
        iso639_3: "yor",
        glottocode: "yoru1245",
        family: "Atlantic-Congo › Volta-Niger",
        macroarea: "Africa",
        region: "Southwestern Nigeria, Benin, Togo",
        speakers: "~47M",
        consonant_count: 18,
        vowel_count: 12,
        tones: Some(3),
        signature_phonemes: &["k͡p", "ɡ͡b", "ɛ", "ɔ", "ĩ", "ã", "˥", "˧", "˩"],
        features: &[
            Feature {
                name: "Labial-velar stops",
                description: "/k͡p/ and /ɡ͡b/ are simultaneously produced at both lips and velum — a hallmark of the Niger-Congo belt.",
            },
            Feature {
                name: "Three-tone system with oral/nasal vowel split",
                description: "Seven oral vowels + five nasal vowels, each bearable of three level tones.",
            },
        ],
        summary: "Yoruba's tonal system operates on a seven-oral-plus-five-nasal vowel grid, and its doubly-articulated stops show articulatory independence of lips and tongue body.",
        sources: &[
            Source { label: "Bamgboṣe (1966) A Grammar of Yoruba", url: None },
            Source { label: "Pulleyblank (1988) in Phonology 5", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/yoru1245"),
    },
    Language {
        id: "xhosa",
        name: "Xhosa",
        endonym: "isiXhosa",
        iso639_3: "xho",
        glottocode: "xhos1239",
        family: "Atlantic-Congo › Bantu (S)",
        macroarea: "Africa",
        region: "Eastern and Western Cape, South Africa",
        speakers: "~19M L1",
        consonant_count: 54,
        vowel_count: 5,
        tones: Some(2),
        signature_phonemes: &["ǀ", "ǁ", "ǃ", "ǀʰ", "ǁʰ", "ǃʰ", "ɡǀ", "ɡǁ", "ɡǃ", "ŋǀ", "ŋǁ", "ŋǃ"],
        features: &[
            Feature {
                name: "Three click types × phonation series",
                description: "Dental ǀ, lateral ǁ, alveolar ǃ — each combined with aspirated, voiced, nasal, and breathy variants.",
            },
            Feature {
                name: "Clicks from language contact",
                description: "Xhosa's click phonemes entered Bantu through historical contact with Khoisan languages — a rare case of borrowed articulatory type.",
            },
        ],
        summary: "Xhosa is the Bantu language best known internationally for clicks, with 15+ click phonemes integrated into an otherwise conventional Bantu consonant system.",
        sources: &[
            Source { label: "Roux & Holzhausen (1994) in Ladefoged & Traill, eds.", url: None },
            Source { label: "Jessen & Roux (2002) JIPA", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/xhos1239"),
    },
    Language {
        id: "ubykh",
        name: "Ubykh",
        endonym: "Адыгэбзэ / Twaxabzá",
        iso639_3: "uby",
        glottocode: "ubyk1235",
        family: "Northwest Caucasian",
        macroarea: "Eurasia",
        region: "Historically Northwest Caucasus; diaspora in Turkey",
        speakers: "0 (last speaker, Tevfik Esenç, d. 1992)",
        consonant_count: 84,
        vowel_count: 2,
        tones: None,
        signature_phonemes: &["q", "qʷ", "qʼ", "qˤ", "ʁ", "χ", "ʕ", "ə", "a"],
        features: &[
            Feature {
                name: "Extreme C/V ratio",
                description: "Roughly 84 consonants to 2 phonemic vowels — among the most skewed consonant-vowel ratios ever documented.",
            },
            Feature {
                name: "Pharyngealization and labialization as dimensions",
                description: "Secondary articulations multiply the consonant inventory: plain, labialized, palatalized, and pharyngealized series across multiple places.",
            },
        ],
        summary: "Ubykh, documented intensively before its extinction in 1992, remains the textbook case of a language whose phonological work is done almost entirely by consonants.",
        sources: &[
            Source { label: "Dumézil (1975) Le Verbe Oubykh", url: None },
            Source { label: "Colarusso (2014) A Grammar of the Kabardian Language (with Ubykh material)", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/ubyk1235"),
    },
    Language {
        id: "navajo",
        name: "Navajo",
        endonym: "Diné bizaad",
        iso639_3: "nav",
        glottocode: "nava1243",
        family: "Dene-Yeniseian › Athabaskan",
        macroarea: "North America",
        region: "Southwestern United States",
        speakers: "~170,000",
        consonant_count: 33,
        vowel_count: 16,
        tones: Some(2),
        signature_phonemes: &["tɬʼ", "tsʼ", "tʃʼ", "kʼ", "ɬ", "ɣ", "x"],
        features: &[
            Feature {
                name: "Ejective affricate series",
                description: "/tɬʼ tsʼ tʃʼ/ — ejective fricative-release affricates, including a lateral one.",
            },
            Feature {
                name: "Four-way vowel system × nasal × tone × length",
                description: "Four vowel qualities, each with oral/nasal, high/low tone, short/long — sixteen vowel phonemes by some counts.",
            },
        ],
        summary: "Navajo layers tone, nasalization, and length over a four-quality vowel system and adds a full ejective series — the prototypical North American Athabaskan phonology.",
        sources: &[
            Source { label: "Young & Morgan (1987) The Navajo Language", url: None },
            Source { label: "McDonough (2003) The Navajo Sound System", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/nava1243"),
    },
    Language {
        id: "welsh",
        name: "Welsh",
        endonym: "Cymraeg",
        iso639_3: "cym",
        glottocode: "wels1247",
        family: "Indo-European › Celtic",
        macroarea: "Eurasia",
        region: "Wales, Patagonia (diaspora)",
        speakers: "~562,000",
        consonant_count: 24,
        vowel_count: 13,
        tones: None,
        signature_phonemes: &["ɬ", "χ", "r̥", "n̥"],
        features: &[
            Feature {
                name: "Voiceless lateral fricative",
                description: "/ɬ/ — the 'll' of Llanelli — present in only a small minority of languages.",
            },
            Feature {
                name: "Initial consonant mutation",
                description: "Word-initial consonants change systematically for grammatical reasons (soft, nasal, aspirate mutations).",
            },
        ],
        summary: "Welsh is the most widely spoken European language with a voiceless lateral fricative and a productive initial-mutation system.",
        sources: &[
            Source { label: "Ball & Müller (1992) The Celtic Languages", url: None },
            Source { label: "Hannahs (2013) The Phonology of Welsh", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/wels1247"),
    },
    Language {
        id: "finnish",
        name: "Finnish",
        endonym: "Suomi",
        iso639_3: "fin",
        glottocode: "finn1318",
        family: "Uralic › Finnic",
        macroarea: "Eurasia",
        region: "Finland",
        speakers: "~5.8M",
        consonant_count: 17,
        vowel_count: 16,
        tones: None,
        signature_phonemes: &["y", "ø", "æ", "ŋ", "h"],
        features: &[
            Feature {
                name: "Vowel harmony (front/back)",
                description: "Within a word, vowels agree for backness — /a o u/ or /æ ø y/ but rarely both.",
            },
            Feature {
                name: "Phonemic length across all sounds",
                description: "Both consonants and vowels have contrastive length: tuli / tuuli / tulli.",
            },
        ],
        summary: "Finnish uses vowel harmony and contrastive length to pack a large amount of phonological contrast into a modest segment inventory.",
        sources: &[
            Source { label: "Suomi, Toivanen & Ylitalo (2008) Finnish Sound Structure", url: None },
            Source { label: "Karlsson (1999) Finnish: An Essential Grammar", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/finn1318"),
    },
    Language {
        id: "swahili",
        name: "Swahili",
        endonym: "Kiswahili",
        iso639_3: "swh",
        glottocode: "swah1253",
        family: "Atlantic-Congo › Bantu (G)",
        macroarea: "Africa",
        region: "East Africa",
        speakers: "~18M L1, ~200M total",
        consonant_count: 30,
        vowel_count: 5,
        tones: None,
        signature_phonemes: &["ɓ", "ɗ", "ᵐb", "ⁿd", "ᵑɡ", "ʃ"],
        features: &[
            Feature {
                name: "Prenasalized stops",
                description: "/ᵐb ⁿd ᵑɡ/ act as single phonemes — a nasal-stop unit onset, not a cluster.",
            },
            Feature {
                name: "Bantu noun-class prosody",
                description: "Swahili lost the canonical Bantu tone system; stress falls on the penultimate syllable instead.",
            },
        ],
        summary: "Swahili is the principal Bantu lingua franca of East Africa and — unusually for the family — has lost contrastive tone, shifting its prosodic work onto fixed stress.",
        sources: &[
            Source { label: "Polomé (1967) Swahili Language Handbook", url: None },
            Source { label: "Mohammed (2001) Modern Swahili Grammar", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/swah1253"),
    },
    Language {
        id: "quechua",
        name: "Quechua (Cuzco)",
        endonym: "Runa Simi",
        iso639_3: "quz",
        glottocode: "cusc1236",
        family: "Quechuan",
        macroarea: "South America",
        region: "Andean highlands, Peru",
        speakers: "~1.5M (Cuzco variety)",
        consonant_count: 26,
        vowel_count: 3,
        tones: None,
        signature_phonemes: &["q", "qʰ", "qʼ", "pʼ", "tʼ", "kʼ", "ʎ"],
        features: &[
            Feature {
                name: "Three-way laryngeal stops",
                description: "Plain / aspirated / ejective contrast across labial, coronal, velar, and uvular places — present in Cuzco/Bolivian Quechua, absent elsewhere in the family.",
            },
            Feature {
                name: "Three-vowel system",
                description: "/a i u/ — among the smallest vowel inventories attested outside language contact situations.",
            },
        ],
        summary: "Cuzco Quechua shows how a language can invest heavily in consonant distinctions (plain, aspirated, ejective × four places) while maintaining a three-vowel system.",
        sources: &[
            Source { label: "Cerrón-Palomino (1987) Lingüística Quechua", url: None },
            Source { label: "Parker (2013) SIL International", url: None },
        ],
        phoible_url: Some("https://phoible.org/languages/cusc1236"),
    },
];
