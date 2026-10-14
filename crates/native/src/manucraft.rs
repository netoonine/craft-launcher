//! MANUCRAFT: what this fork changes in the desktop backend, kept in one file so syncing with
//! upstream stays easy (see `MANUCRAFT.md` at the repository root).
//!
//! - [`NO_FEED`]: the launcher never asks ArtCraft's release feed (`api.storyteller.ai`); every
//!   app's latest release comes from GitHub (the API, or the quota-free `releases/latest`
//!   redirect plus `SHA256SUMS.txt`).
//! - [`with_build`]: a `.app` only reports its upstream version (`0.9.0`); the launcher's own
//!   record of what it installed carries the MANUCRAFT build (`0.9.0+m2`), so the next build of
//!   the same upstream version is offered as an update, and an installed one is up to date.
//! - [`signature_intact`]: MANUCRAFT builds are ad-hoc signed, so an app is accepted with any
//!   intact code signature (`codesign --verify --deep --strict`) instead of only the ArtCraft
//!   team's Developer ID. The download's SHA-256 is still checked against the release first.

use artcraft_launcher_core::Installed;
use artcraft_launcher_core::version;

/// The release feed is never asked.
pub const NO_FEED: bool = true;

/// `found` (read from the app itself) with the MANUCRAFT build of the release the launcher
/// recorded installing, when both name the same upstream version and `found` has no build of its
/// own. Anything else (the app was replaced by another version since) is left as found.
pub fn with_build(mut found: Installed, recorded: &str) -> Installed {
    if let (Some(v), Some(rec)) = (found.version.as_ref(), version::parse(recorded))
        && v.build.is_empty()
        && v.cmp_precedence(&rec).is_eq()
    {
        found.version = Some(rec);
    }
    found
}

/// Whether `bundle` carries an intact code signature (ad-hoc or a developer's), nested code
/// included: nothing in it was changed after signing. Needs no network.
#[cfg(target_os = "macos")]
pub fn signature_intact(bundle: &std::path::Path) -> bool {
    let Some(path) = bundle.to_str() else { return false };
    crate::process::capture("/usr/bin/codesign", &["--verify", "--deep", "--strict", path], std::time::Duration::from_secs(120)).is_some()
}

#[cfg(test)]
mod tests {
    use artcraft_launcher_core::InstallSource;

    use super::*;

    fn found(v: &str) -> Installed {
        Installed { version: version::parse(v), source: InstallSource::MacApp, location: "/Applications/PhotoCraft.app".into(), package: None }
    }

    #[test]
    fn the_recorded_build_completes_the_bundle_version() {
        assert_eq!(with_build(found("0.9.0"), "0.9.0+m2").version, version::parse("0.9.0+m2"));
        // The app was replaced by another upstream version: what's there wins.
        assert_eq!(with_build(found("0.9.1"), "0.9.0+m2").version, version::parse("0.9.1"));
        assert_eq!(with_build(found("0.9.0"), "garbage").version, version::parse("0.9.0"));
        assert_eq!(with_build(found("garbage"), "0.9.0+m2").version, None);
    }

    /// A real release DMG goes through the launcher's macOS install checks (signature intact, the
    /// app's bundle id, the release's version) into a scratch Applications folder:
    ///
    /// ```sh
    /// MANUCRAFT_DMG=dist/release/artcraft-launcher-0.4.1-m1-macos-aarch64.dmg MANUCRAFT_APP=artcraft-launcher \
    ///   MANUCRAFT_VERSION=0.4.1+m1 cargo test -p artcraft-launcher-native -- --ignored installs_a_real_dmg
    /// ```
    ///
    /// `MANUCRAFT_APP` is an app id (`photocraft`) or `artcraft-launcher`; `MANUCRAFT_VERSION` the
    /// release tag's version.
    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "needs a DMG: MANUCRAFT_DMG, MANUCRAFT_APP, MANUCRAFT_VERSION"]
    fn installs_a_real_dmg() {
        use artcraft_launcher_core::catalog::{LAUNCHER, LAUNCHER_ID};
        use artcraft_launcher_core::{CraftApp, Release};
        let dmg = std::path::PathBuf::from(std::env::var("MANUCRAFT_DMG").unwrap());
        let id = std::env::var("MANUCRAFT_APP").unwrap();
        let app = if id == LAUNCHER_ID { &LAUNCHER } else { CraftApp::by_id(&id).unwrap() };
        let release = Release { version: version::parse(&std::env::var("MANUCRAFT_VERSION").unwrap()), ..Release::default() };
        let dir = tempfile::tempdir().unwrap();
        let apps = dir.path().join("Applications");
        std::fs::create_dir_all(&apps).unwrap();
        let target = apps.join(format!("{}.app", app.name));
        let existing = Installed { version: None, source: InstallSource::MacApp, location: target.display().to_string(), package: None };
        let paths = crate::paths::Paths::under(dir.path());
        let placed =
            crate::install::macos::install_dmg(app, &dmg, Some(&existing), &paths, &crate::install::macos::Expect::release(&release)).unwrap();
        assert_eq!(placed, target);
        let found = crate::detect::macos::read_bundle(app, &placed).unwrap();
        let shown = with_build(found, &release.version_label());
        assert_eq!(shown.version, release.version, "the launcher would show {:?}", shown.version);
        assert!(signature_intact(&placed));
    }

    /// An unsigned bundle fails, an ad-hoc signed one passes, and a change after signing fails.
    #[cfg(target_os = "macos")]
    #[test]
    fn ad_hoc_signatures_pass_and_tampering_fails() {
        let dir = tempfile::tempdir().unwrap();
        let app = dir.path().join("PhotoCraft.app");
        let contents = app.join("Contents");
        std::fs::create_dir_all(contents.join("MacOS")).unwrap();
        std::fs::write(
            contents.join("Info.plist"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><plist version=\"1.0\"><dict>\
             <key>CFBundleIdentifier</key><string>ai.storyteller.photocraft</string>\
             <key>CFBundleExecutable</key><string>photocraft</string></dict></plist>",
        )
        .unwrap();
        std::fs::copy("/usr/bin/true", contents.join("MacOS/photocraft")).unwrap();
        assert!(!signature_intact(&app), "an unsigned bundle passed");
        let ok = std::process::Command::new("codesign").args(["--force", "--sign", "-"]).arg(&app).status().unwrap().success();
        assert!(ok, "codesign failed");
        assert!(signature_intact(&app), "an ad-hoc signature failed");
        std::fs::write(contents.join("Info.plist"), "<plist version=\"1.0\"><dict></dict></plist>").unwrap();
        assert!(!signature_intact(&app), "a bundle changed after signing passed");
    }
}
