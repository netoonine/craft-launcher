//! MANUCRAFT: what this fork changes in the interface, kept in one file so syncing with upstream
//! stays easy (see `MANUCRAFT.md` at the repository root).
//!
//! The product is called MANUCRAFT. Rather than editing every English string in the code and
//! every row of the eleven catalogs (which upstream changes all the time), the name is swapped
//! where strings are looked up: each catalog's translations go through [`rebrand`] as it loads,
//! and English, which has no catalog upstream, gets one made of just the strings that name the
//! product ([`catalog_source`]). Messages and plural strings are rebranded on the way out too.
//!
//! The ArtCraft community links (Discord, website, social accounts) are hidden
//! ([`community_links`]); the one link left, "GitHub", points at the MANUCRAFT project
//! (`artcraft_launcher_core::manucraft::PROJECT_URL`).

use std::borrow::Cow;

use crate::i18n::LangInfo;

/// What the product used to be called, longest first, and what it is called now.
const NAMES: [(&str, &str); 2] = [("ArtCraft Launcher", "MANUCRAFT"), ("ArtCraft", "MANUCRAFT")];

/// `text` with the product's old names replaced. A name glued to a suffix (Hungarian
/// "Launcherből") gets a hyphen: "MANUCRAFT-ből".
pub fn rebrand(text: &str) -> Cow<'_, str> {
    if !text.contains("ArtCraft") {
        return Cow::Borrowed(text);
    }
    let mut out = text.to_string();
    for (old, new) in NAMES {
        let mut result = String::with_capacity(out.len());
        let mut rest = out.as_str();
        while let Some(at) = rest.find(old) {
            result.push_str(&rest[..at]);
            result.push_str(new);
            rest = &rest[at + old.len()..];
            if rest.starts_with(char::is_alphabetic) {
                result.push('-');
            }
        }
        result.push_str(rest);
        out = result;
    }
    Cow::Owned(out)
}

/// The TSV a language's catalog is parsed from: the language's own with every translation
/// rebranded, or, for English (no catalog upstream), a row for each English string any catalog
/// has that names the product, translated to its rebranded self.
pub fn catalog_source(lang: &LangInfo) -> Cow<'static, str> {
    if lang.source.is_empty() {
        let mut rows = std::collections::BTreeSet::new();
        for other in crate::i18n::LANGUAGES.iter() {
            for (context, source, _) in other.source.lines().filter_map(split_row) {
                if source.contains("ArtCraft") {
                    rows.insert(format!("{context}\t{source}\t{}", rebrand(source)));
                }
            }
        }
        return Cow::Owned(rows.into_iter().collect::<Vec<_>>().join("\n"));
    }
    if !lang.source.contains("ArtCraft") {
        return Cow::Borrowed(lang.source);
    }
    let lines: Vec<Cow<'_, str>> = lang
        .source
        .lines()
        .map(|line| match split_row(line) {
            Some((context, source, translation)) if translation.contains("ArtCraft") => {
                Cow::Owned(format!("{context}\t{source}\t{}", rebrand(translation)))
            }
            _ => Cow::Borrowed(line),
        })
        .collect();
    Cow::Owned(lines.join("\n"))
}

/// A catalog row's (context, source, translation), escapes left as they are.
fn split_row(line: &str) -> Option<(&str, &str, &str)> {
    let line = line.trim_end();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut fields = line.split('\t');
    let row = (fields.next()?, fields.next()?, fields.next()?);
    fields.next().is_none().then_some(row)
}

/// Whether the ArtCraft community links (Discord, website, social accounts) are shown: never.
pub fn community_links() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{Lang, message, tr, trn};

    #[test]
    fn rebrands_names_in_any_position() {
        assert_eq!(rebrand("Quit ArtCraft Launcher"), "Quit MANUCRAFT");
        assert_eq!(rebrand("ArtCraft Launcher {version} is available."), "MANUCRAFT {version} is available.");
        assert_eq!(rebrand("Kilépés az ArtCraft Launcherből"), "Kilépés az MANUCRAFT-ből");
        assert_eq!(rebrand("ArtCraft on GitHub"), "MANUCRAFT on GitHub");
        assert_eq!(rebrand("Craft App Suite"), "Craft App Suite");
    }

    #[test]
    fn every_language_shows_the_new_name() {
        for lang in Lang::all() {
            assert_eq!(tr(lang, "ArtCraft Launcher"), "MANUCRAFT", "{}", lang.code());
            assert!(!tr(lang, "Quit ArtCraft Launcher").contains("ArtCraft"), "{}", lang.code());
            assert!(!tr(lang, "ArtCraft Launcher {version} is available.").contains("ArtCraft"), "{}", lang.code());
            assert!(!trn(lang, 2, "ArtCraft Launcher · {n} update available", "ArtCraft Launcher · {n} updates available").contains("ArtCraft"));
            assert!(!message(lang, "ArtCraft Launcher is restarting to finish its update. Try again once it's back.").contains("ArtCraft"));
        }
        assert_eq!(tr(Lang::EN, "Show ArtCraft Launcher"), "Show MANUCRAFT");
        // Everything else in English is still the source text.
        assert_eq!(tr(Lang::EN, "Check for Updates"), "Check for Updates");
    }

    #[test]
    fn community_links_are_hidden() {
        assert!(!community_links());
    }
}
