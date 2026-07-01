//! IPA phoneme classification for display purposes.
//!
//! Phonemes are rendered with layout cues based on their articulatory class.
//! This module does NOT define phoneme inventories — those live per-language
//! in `languages.rs` with primary-source citations.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhonemeKind {
    Consonant,
    Vowel,
    Tone,
    Click,
    Ejective,
    Implosive,
}

/// Classify an IPA symbol heuristically for rendering.
/// This is *presentational*, not phonological — for a true inventory
/// analysis, consult the cited source for the language.
pub fn classify(symbol: &str) -> PhonemeKind {
    // Click consonants
    if symbol.contains('ǀ')
        || symbol.contains('ǁ')
        || symbol.contains('ǂ')
        || symbol.contains('ǃ')
        || symbol.contains('ʘ')
    {
        return PhonemeKind::Click;
    }
    // Ejectives (apostrophe modifier)
    if symbol.ends_with('ʼ') || symbol.ends_with('\'') {
        return PhonemeKind::Ejective;
    }
    // Implosives
    if matches!(symbol, "ɓ" | "ɗ" | "ʄ" | "ɠ" | "ʛ") {
        return PhonemeKind::Implosive;
    }
    // Tone marks
    if symbol.starts_with('˦')
        || symbol.starts_with('˨')
        || symbol.starts_with('˧')
        || symbol.starts_with('˩')
        || symbol.starts_with('˥')
        || symbol.contains("tone")
    {
        return PhonemeKind::Tone;
    }

    // Vowels: canonical IPA vowel glyphs
    let first = symbol.chars().next().unwrap_or(' ');
    if "aeiouɑɐɒæɛɜəɘɵɤɪɨʊʌɔøœyɯɒ".contains(first) {
        return PhonemeKind::Vowel;
    }
    PhonemeKind::Consonant
}

impl PhonemeKind {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Consonant => "k-consonant",
            Self::Vowel => "k-vowel",
            Self::Tone => "k-tone",
            Self::Click => "k-click",
            Self::Ejective => "k-ejective",
            Self::Implosive => "k-implosive",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Consonant => "consonant",
            Self::Vowel => "vowel",
            Self::Tone => "tone",
            Self::Click => "click",
            Self::Ejective => "ejective",
            Self::Implosive => "implosive",
        }
    }
}
