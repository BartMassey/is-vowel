![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/BartMassey/is-vowel/actions/workflows/main.yml/badge.svg)](https://github.com/BartMassey/is-vowel/actions)
[![crates-io](https://img.shields.io/crates/v/is-vowel.svg)](https://crates.io/crates/is-vowel)
[![api-docs](https://docs.rs/is-vowel/badge.svg)](https://docs.rs/is-vowel)

# is-vowel: heuristically test whether a character is a vowel letter
Bart Massey 2021 (version 0.1.0)

Test for vowels in [Romance languages](https://en.wikipedia.org/wiki/Romance_languages).

Deciding whether some grapheme is a "vowel" (represents a "vowel sound") is language
dependent, and the author is not aware of any standardization efforts for vowel
identification.

Even for the Romance languages, the situation is a bit complicated. The basic Romance vowels
are "a", "e", "i", "o", "u". However, for example:

  * Uppercase versions are also vowels.
  * Accented versions are also vowels.
  * "Sometimes 'y', sometimes 'w'." These letters are consonant in some situations, vowels
    in others.

This code attempts to provide reasonable heuristic answers as to the
Romance-language-"vowelness" of a Unicode codepoint.

No attempt is made here to deal with non-Romance languages, even though some non-Romance
vowel letters are borrowed in Romance languages: for example, "æ" and "Æ" are used in some
British English. It would be great to use vowel letter tables for a variety of languages,
but so far the author has been unable to locate such tables (surprisingly).

# License

This crate is made available under the "MIT
license". Please see the file `LICENSE` in this distribution
for license terms.

# Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.
