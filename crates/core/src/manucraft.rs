//! MANUCRAFT: what this fork changes in the core, kept in one file so syncing with upstream stays
//! easy (see `MANUCRAFT.md` at the repository root).
//!
//! - Where the apps and the launcher come from: [`GITHUB_OWNER`]'s forks on GitHub.
//! - The launcher's own identity ([`LAUNCHER_NAME`], [`LAUNCHER_BUNDLE_ID`]).
//! - The release build number: MANUCRAFT tags `v<upstream version>+m<N>` (for example
//!   `v0.9.0+m2`) to publish several builds of the same upstream version. Semver ignores build
//!   metadata when ordering, so [`build_order`] breaks the tie: `0.9.0` < `0.9.0+m1` <
//!   `0.9.0+m2` < `0.9.1`. Build metadata never makes a release a pre-release.

use std::cmp::Ordering;

use semver::Version;

/// The GitHub account every app (and the launcher) is installed from.
pub const GITHUB_OWNER: &str = "netoonine";
/// The MANUCRAFT project on GitHub: the one link the launcher shows.
pub const PROJECT_URL: &str = "https://github.com/netoonine/manucraft";
/// The launcher's own repository (releases of the launcher itself).
pub const LAUNCHER_REPO: &str = "netoonine/craft-launcher";
/// The launcher's display name.
pub const LAUNCHER_NAME: &str = "MANUCRAFT";
/// The launcher's macOS bundle id (and app id elsewhere). The Craft apps keep `ai.storyteller.<id>`.
pub const LAUNCHER_BUNDLE_ID: &str = "ai.manucraft.launcher";
/// The notice shown in Settings and the README.
pub const ABOUT: &str = "MANUCRAFT is based on ArtCraft Launcher by the ArtCraft team; not affiliated with or endorsed by them.";

/// The MANUCRAFT build number in a version's build metadata: `m2` in `0.9.0+m2`. Anything else
/// (no build metadata, `+abc`, `+m`) counts as build 0.
pub fn build_number(v: &Version) -> u64 {
    v.build.as_str().strip_prefix('m').filter(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit())).and_then(|n| n.parse().ok()).unwrap_or(0)
}

/// Orders two versions of equal precedence by their MANUCRAFT build number.
pub fn build_order(a: &Version, b: &Version) -> Ordering {
    build_number(a).cmp(&build_number(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::{is_newer, is_update, parse};

    fn v(s: &str) -> Version {
        parse(s).unwrap()
    }

    #[test]
    fn reads_build_numbers() {
        assert_eq!(build_number(&v("v0.9.0+m2")), 2);
        assert_eq!(build_number(&v("0.9.0+m12")), 12);
        assert_eq!(build_number(&v("0.9.0")), 0);
        assert_eq!(build_number(&v("0.9.0+abc")), 0);
        assert_eq!(build_number(&v("0.9.0+m")), 0);
        assert_eq!(build_number(&v("0.9.0+m2x")), 0);
    }

    #[test]
    fn builds_order_within_an_upstream_version() {
        assert!(is_newer(&v("v0.9.0+m2"), &v("0.9.0+m1")));
        assert!(is_newer(&v("v0.9.0+m10"), &v("0.9.0+m9")));
        assert!(is_newer(&v("v0.9.0+m1"), &v("0.9.0")));
        assert!(is_newer(&v("v0.9.1+m1"), &v("0.9.0+m7")));
        assert!(!is_newer(&v("v0.9.0+m2"), &v("0.9.0+m2")));
        // A plain upstream copy is offered the MANUCRAFT build; the same build is up to date.
        assert!(is_update(&v("v0.9.0+m2"), &v("0.9.0")));
        assert!(!is_update(&v("v0.9.0+m2"), &v("0.9.0+m2")));
        assert!(!is_update(&v("v0.9.0+m2"), &v("0.9.0+m3")));
        // Build metadata never makes a pre-release.
        assert!(crate::version::is_stable(&v("v0.9.0+m2")));
    }

    #[test]
    fn the_newest_build_wins_in_a_release_list() {
        let json = r#"[
            {"tag_name":"v0.9.0+m2","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.9.0+m10","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.9.0+m9","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.8.0+m30","prerelease":false,"draft":false,"assets":[]}]"#;
        let best = crate::release::parse_github_release_list(json, "netoonine/photocraft", false).unwrap().unwrap();
        assert_eq!(best.tag, "v0.9.0+m10");
    }
}
