//! Bundled images: each Craft App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Craft App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "photocraft" => egui::include_image!("../../../assets/app-icons/photocraft.png"),
        "vectorcraft" => egui::include_image!("../../../assets/app-icons/vectorcraft.png"),
        "filmcraft" => egui::include_image!("../../../assets/app-icons/filmcraft.png"),
        "lightcraft" => egui::include_image!("../../../assets/app-icons/lightcraft.png"),
        "pdfcraft" => egui::include_image!("../../../assets/app-icons/pdfcraft.png"),
        "effectcraft" => egui::include_image!("../../../assets/app-icons/effectcraft.png"),
        "designcraft" => egui::include_image!("../../../assets/app-icons/designcraft.png"),
        "wordcraft" => egui::include_image!("../../../assets/app-icons/wordcraft.png"),
        "deckcraft" => egui::include_image!("../../../assets/app-icons/deckcraft.png"),
        "gridcraft" => egui::include_image!("../../../assets/app-icons/gridcraft.png"),
        "cadcraft" => egui::include_image!("../../../assets/app-icons/cadcraft.png"),
        "soundcraft" => egui::include_image!("../../../assets/app-icons/soundcraft.png"),
        "figcraft" => egui::include_image!("../../../assets/app-icons/figcraft.png"),
        _ => return None,
    })
}

/// MANUCRAFT: the MANUCRAFT lockup (`assets/manucraft/`: the flat red "M" and the wordmark), dark
/// ink for light backgrounds, white for dark ones. Replaces the ArtCraft wordmark, a trademark not licensed for forks.
pub fn artcraft_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../assets/manucraft/manucraft-lockup-white.svg") // MANUCRAFT
    } else {
        egui::include_image!("../../../assets/manucraft/manucraft-lockup.svg") // MANUCRAFT
    }
}

/// The lockup's width / height (its SVG view box: 1282 × 102).
pub const ARTCRAFT_LOGO_ASPECT: f32 = 1282.0 / 102.0; // MANUCRAFT

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// MANUCRAFT: the MANUCRAFT mark, also the launcher's app icon (was the ArtCraft "A" mark).
pub fn artcraft_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/manucraft/manucraft-mark.svg") // MANUCRAFT
}
