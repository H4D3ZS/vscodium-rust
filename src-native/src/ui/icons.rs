use gpui_kit::gpui::*;
pub use gpui_kit_assets::IconName;

/// Render a styled Lucide vector icon with precise pixel sizing and coloring.
pub fn ui_icon(name: IconName, size_px: f32, color: Rgba) -> impl IntoElement {
    gpui_kit::component::Icon::new(name)
        .size(px(size_px))
        .text_color(color)
}

/// Helper for standard 16px interactive icons
#[allow(dead_code)]
pub fn icon_16(name: IconName, color: Rgba) -> impl IntoElement {
    ui_icon(name, 16.0, color)
}

/// Helper for 18-20px activity bar icons
#[allow(dead_code)]
pub fn icon_20(name: IconName, color: Rgba) -> impl IntoElement {
    ui_icon(name, 20.0, color)
}

/// Helper for 12-14px micro UI indicators (chevrons, close x)
pub fn icon_12(name: IconName, color: Rgba) -> impl IntoElement {
    ui_icon(name, 12.0, color)
}

pub fn icon_14(name: IconName, color: Rgba) -> impl IntoElement {
    ui_icon(name, 14.0, color)
}

static APP_ICON: std::sync::OnceLock<std::sync::Arc<RenderImage>> = std::sync::OnceLock::new();

pub fn get_app_icon() -> std::sync::Arc<RenderImage> {
    APP_ICON
        .get_or_init(|| {
            let bytes = include_bytes!("../../app_icon.png");
            let dyn_img = image::load_from_memory(bytes).expect("failed to decode app_icon.png");
            let mut rgba = dyn_img.into_rgba8();
            for pixel in rgba.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }
            std::sync::Arc::new(RenderImage::new(
                smallvec::SmallVec::from_elem(image::Frame::new(rgba), 1),
            ))
        })
        .clone()
}
