#![doc(html_root_url = "https://docs.rs/is-vowel/0.1.0")]

//! Test for vowels in [Romance languages](https://en.wikipedia.org/wiki/Romance_languages).
//!
//! Deciding whether some grapheme is a "vowel" (represents a "vowel sound") is language
//! dependent, and the author is not aware of any standardization efforts for vowel
//! identification.
//!
//! Even for the Romance languages, the situation is a bit complicated. The basic Romance vowels
//! are "a", "e", "i", "o", "u". However, for example:
//!
//!   * Uppercase versions are also vowels.
//!   * Accented versions are also vowels.
//!   * "Sometimes 'y', sometimes 'w'." These letters are consonant in some situations, vowels
//!     in others.
//!
//! This code attempts to provide reasonable heuristic answers as to the
//! Romance-language-"vowelness" of a Unicode codepoint.
//!
//! No attempt is made here to deal with non-Romance languages, even though some non-Romance
//! vowel letters are borrowed in Romance languages: for example, "æ" and "Æ" are used in some
//! British English. It would be great to use vowel letter tables for a variety of languages,
//! but so far the author has been unable to locate such tables (surprisingly).

use std::collections::HashSet;

use once_cell::sync::Lazy;
use unicode_normalization::UnicodeNormalization;

pub trait IsRomanceVowel: private::Sealed {
    /// Return `true` if `self` appears to be a "vowel" in a Romance language.
    ///
    /// This function first decomposes its argument according to [Unicode Normalization Form
    /// KD](https://unicode.org/reports/tr15/#Norm_Forms) to remove accents.  It then considers
    /// the argument to be a vowel if the first codepoint in the decomposition of `self` is "a",
    /// "e", "i", "o", "u" or their uppercase equivalents.
    ///
    /// The author believes that this produces a reasonable approximation of "vowelness"
    /// for Romance languages. Counterexamples are welcome.
    ///
    /// # Examples
    ///
    ///     # use is_vowel::IsRomanceVowel;
    ///     for c in "xÆ".chars() {
    ///         assert!(!c.is_romance_vowel());
    ///     }
    ///     for c in "aAáÁÅ".chars() {
    ///         assert!(c.is_romance_vowel());
    ///     }
    #[allow(clippy::wrong_self_convention)]
    fn is_romance_vowel(self) -> bool;

    /// Behave as [`is_romance_vowel`][IsRomanceVowel::is_romance_vowel], but also include the
    /// characters in `extra_vowels` as vowels.
    ///
    /// # Examples
    ///
    ///     # use is_vowel::IsRomanceVowel;
    ///     # use std::collections::HashSet;
    ///     let extra_vowels: HashSet<char> = "yYwW".chars().collect();
    ///     for c in "yW".chars() {
    ///         assert!(c.is_romance_vowel_including(&extra_vowels));
    ///     }
    #[allow(clippy::wrong_self_convention)]
    fn is_romance_vowel_including(self, extra_vowels: &HashSet<char>) -> bool;
}

/// "Base" set of vowels.
static BASE_VOWELS: Lazy<HashSet<char>> = Lazy::new(|| "aeiouAEIOU".chars().collect());

/// Given a Unicode codepoint `c`, decompose `c` according to Unicode Normalization Form NKD and
/// return the first codepoint.
fn decompose_starting(c: char) -> char {
    // Sadly, the `unicode-normalization` crate wants an `&str`.
    let s = c.to_string();
    s.nfkd().next().unwrap()
}

/// Return `true` if `c` is in `vowels` after transformation via `decompose_starting
fn is_romance_vowel_with(c: char, vowels: &HashSet<char>) -> bool {
    let c = decompose_starting(c);
    vowels.contains(&c)
}

impl IsRomanceVowel for char {
    fn is_romance_vowel(self) -> bool {
        is_romance_vowel_with(self, &BASE_VOWELS)
    }

    fn is_romance_vowel_including(self, extra_vowels: &HashSet<char>) -> bool {
        let vowels: HashSet<char> = BASE_VOWELS.union(extra_vowels).cloned().collect();
        is_romance_vowel_with(self, &vowels)
    }
}

// https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for char {}
}
