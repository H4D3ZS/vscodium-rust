use gpui_kit::gpui::{rgb, rgba, Rgba};

#[derive(Clone, Debug)]
pub struct Theme {
    // Surfaces (Exact Cursor Dark / Obsidian palette)
    pub bg_app: Rgba,
    pub bg_titlebar: Rgba,
    pub bg_activity_bar: Rgba,
    pub bg_sidebar: Rgba,
    pub bg_editor: Rgba,
    pub bg_status_bar: Rgba,
    pub bg_raised: Rgba,
    pub bg_composer: Rgba,
    pub bg_hover: Rgba,
    pub bg_active: Rgba,
    pub bg_card: Rgba,
    pub bg_surface: Rgba,
    pub bg_input: Rgba,
    pub bg_panel: Rgba,

    // Glassmorphism & Translucency
    pub bg_glass_card: Rgba,
    pub bg_glass_overlay: Rgba,
    pub border_glass: Rgba,
    pub active_line_bg: Rgba,
    pub active_line_border: Rgba,

    // Borders
    pub border_subtle: Rgba,
    pub border_focus: Rgba,
    pub border_composer: Rgba,

    // Typography
    pub text_primary: Rgba,
    pub text_muted: Rgba,
    pub text_subtle: Rgba,
    pub text_on_accent: Rgba,

    // Accent & Highlights
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub accent_glow: Rgba,
    pub glow_accent: Rgba,
    pub glow_subtle: Rgba,

    // Depth & Shadows (colors fed into gpui BoxShadow)
    pub shadow_soft: Rgba,
    pub shadow_strong: Rgba,
    pub glow_hover: Rgba,

    // Focus Ring (TS --color-border-focus parity)
    pub ring_focus: Rgba,

    // Pills (translucent bordered surface, TS `.pill` parity)
    pub pill_bg: Rgba,
    pub pill_border: Rgba,
    pub pill_hover_bg: Rgba,
    pub pill_hover_border: Rgba,

    // Accent Gradient (primary actions / hero buttons)
    pub accent_gradient_from: Rgba,
    pub accent_gradient_to: Rgba,

    // Status Indicators
    pub status_green: Rgba,
    pub status_yellow: Rgba,
    pub status_red: Rgba,
    pub status_blue: Rgba,

    // Syntax Highlighting Tokens (Monaco / Cursor Parity)
    pub syn_keyword: Rgba,
    pub syn_function: Rgba,
    pub syn_type: Rgba,
    pub syn_string: Rgba,
    pub syn_comment: Rgba,
    pub syn_number: Rgba,
    pub syn_variable: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Self::cursor_dark()
    }
}

impl Theme {
    /// Authentic VSCodium / Cursor Dark palette with vibrant depth and Monaco surfaces
    pub fn cursor_dark() -> Self {
        Self {
            bg_app: rgb(0x181818),
            bg_titlebar: rgb(0x222222),
            bg_activity_bar: rgb(0x1c1c1c),
            bg_sidebar: rgb(0x252526),
            bg_editor: rgb(0x1e1e1e),
            bg_status_bar: rgb(0x181818),
            bg_raised: rgb(0x2d2d30),
            bg_composer: rgb(0x252526),
            bg_hover: rgba(0xffffff12),
            bg_active: rgba(0x007acc40),
            bg_card: rgb(0x252526),
            bg_surface: rgb(0x252526),
            bg_input: rgb(0x1e1e1e),
            bg_panel: rgb(0x1e1e1e),

            bg_glass_card: rgba(0x252526f8),
            bg_glass_overlay: rgba(0x181818f0),
            border_glass: rgba(0xffffff20),
            active_line_bg: rgba(0xffffff08),
            active_line_border: rgba(0x007acc50),

            border_subtle: rgb(0x333333),
            border_focus: rgb(0x007acc),
            border_composer: rgb(0x3c3c3c),

            text_primary: rgb(0xcccccc),
            text_muted: rgb(0x9d9d9d),
            text_subtle: rgb(0x757575),
            text_on_accent: rgb(0xffffff),

            accent: rgb(0x007acc),
            accent_hover: rgb(0x1f8ad2),
            accent_glow: rgba(0x007acc50),
            glow_accent: rgba(0x007acc60),
            glow_subtle: rgba(0x007acc30),

            shadow_soft: rgba(0x0000005c),
            shadow_strong: rgba(0x00000070),
            glow_hover: rgba(0x007acc28),

            ring_focus: rgb(0x007fd4),

            pill_bg: rgba(0xffffff0a),
            pill_border: rgba(0xffffff1a),
            pill_hover_bg: rgba(0xffffff14),
            pill_hover_border: rgba(0xffffff24),

            accent_gradient_from: rgb(0x1f8ad2),
            accent_gradient_to: rgb(0x0068ae),

            status_green: rgb(0x4ade80),
            status_yellow: rgb(0xfbbf24),
            status_red: rgb(0xf87171),
            status_blue: rgb(0x60a5fa),

            syn_keyword: rgb(0xf43f5e),
            syn_function: rgb(0x38bdf8),
            syn_type: rgb(0x4ade80),
            syn_string: rgb(0xce9178),
            syn_comment: rgb(0x6a9955),
            syn_number: rgb(0xb5cea8),
            syn_variable: rgb(0x9cdcfe),
        }
    }

    /// Tokyo Night Dark palette
    pub fn tokyo_night() -> Self {
        Self {
            bg_app: rgb(0x16161e),
            bg_titlebar: rgb(0x1a1b26),
            bg_activity_bar: rgb(0x16161e),
            bg_sidebar: rgb(0x1a1b26),
            bg_editor: rgb(0x1a1b26),
            bg_status_bar: rgb(0x16161e),
            bg_raised: rgb(0x24283b),
            bg_composer: rgb(0x1f2335),
            bg_hover: rgba(0xffffff10),
            bg_active: rgba(0xffffff18),
            bg_card: rgb(0x1f2335),
            bg_surface: rgb(0x1f2335),
            bg_input: rgb(0x16161e),
            bg_panel: rgb(0x16161e),

            bg_glass_card: rgba(0x1f2335f0),
            bg_glass_overlay: rgba(0x16161ee6),
            border_glass: rgba(0xffffff15),
            active_line_bg: rgba(0x292e4266),
            active_line_border: rgba(0x7aa2f740),

            border_subtle: rgb(0x292e42),
            border_focus: rgb(0x7aa2f7),
            border_composer: rgb(0x3b4261),

            text_primary: rgb(0xc0caf5),
            text_muted: rgb(0xa9b1d6),
            text_subtle: rgb(0x565f89),
            text_on_accent: rgb(0x1a1b26),

            accent: rgb(0x7aa2f7),
            accent_hover: rgb(0xbb9af7),
            accent_glow: rgba(0x7aa2f735),
            glow_accent: rgba(0x7aa2f740),
            glow_subtle: rgba(0xbb9af720),

            shadow_soft: rgba(0x00000059),
            shadow_strong: rgba(0x0000006e),
            glow_hover: rgba(0x7aa2f726),

            ring_focus: rgb(0x7aa2f7),

            pill_bg: rgba(0xffffff0a),
            pill_border: rgba(0xffffff15),
            pill_hover_bg: rgba(0xffffff12),
            pill_hover_border: rgba(0xffffff20),

            accent_gradient_from: rgb(0x7aa2f7),
            accent_gradient_to: rgb(0x5a86e8),

            status_green: rgb(0x9ece6a),
            status_yellow: rgb(0xe0af68),
            status_red: rgb(0xf7768e),
            status_blue: rgb(0x7aa2f7),

            syn_keyword: rgb(0xbb9af7),
            syn_function: rgb(0x7aa2f7),
            syn_type: rgb(0x2ac3de),
            syn_string: rgb(0x9ece6a),
            syn_comment: rgb(0x565f89),
            syn_number: rgb(0xff9e64),
            syn_variable: rgb(0xc0caf5),
        }
    }

    /// Pure Obsidian Black palette
    pub fn obsidian() -> Self {
        Self {
            bg_app: rgb(0x0a0a0a),
            bg_titlebar: rgb(0x101010),
            bg_activity_bar: rgb(0x080808),
            bg_sidebar: rgb(0x101010),
            bg_editor: rgb(0x0d0d0d),
            bg_status_bar: rgb(0x080808),
            bg_raised: rgb(0x161616),
            bg_composer: rgb(0x141414),
            bg_hover: rgba(0xffffff0f),
            bg_active: rgba(0xffffff1a),
            bg_card: rgb(0x141414),
            bg_surface: rgb(0x121212),
            bg_input: rgb(0x161616),
            bg_panel: rgb(0x0d0d0d),

            bg_glass_card: rgba(0x141414f5),
            bg_glass_overlay: rgba(0x0a0a0ae6),
            border_glass: rgba(0xffffff12),
            active_line_bg: rgba(0xffffff08),
            active_line_border: rgba(0x60a5fa40),

            border_subtle: rgb(0x202020),
            border_focus: rgb(0x60a5fa),
            border_composer: rgb(0x282828),

            text_primary: rgb(0xf5f5f5),
            text_muted: rgb(0xa3a3a3),
            text_subtle: rgb(0x737373),
            text_on_accent: rgb(0x000000),

            accent: rgb(0x60a5fa),
            accent_hover: rgb(0x93c5fd),
            accent_glow: rgba(0x60a5fa35),
            glow_accent: rgba(0x60a5fa40),
            glow_subtle: rgba(0x60a5fa20),

            shadow_soft: rgba(0x00000059),
            shadow_strong: rgba(0x00000070),
            glow_hover: rgba(0x60a5fa22),

            ring_focus: rgb(0x60a5fa),

            pill_bg: rgba(0xffffff08),
            pill_border: rgba(0xffffff12),
            pill_hover_bg: rgba(0xffffff10),
            pill_hover_border: rgba(0xffffff1c),

            accent_gradient_from: rgb(0x60a5fa),
            accent_gradient_to: rgb(0x3b82f6),

            status_green: rgb(0x22c55e),
            status_yellow: rgb(0xeab308),
            status_red: rgb(0xf43f5e),
            status_blue: rgb(0x60a5fa),

            syn_keyword: rgb(0xf43f5e),
            syn_function: rgb(0x60a5fa),
            syn_type: rgb(0x38bdf8),
            syn_string: rgb(0x4ade80),
            syn_comment: rgb(0x737373),
            syn_number: rgb(0xfbbf24),
            syn_variable: rgb(0xf5f5f5),
        }
    }
}
