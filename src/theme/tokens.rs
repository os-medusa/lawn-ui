use gpui::{Hsla, Pixels, hsla, px};

/// Complete set of semantic design tokens matching modern shadcn/ui.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct ThemeColors {
    // Base surfaces
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,

    // Primary & Secondary actions
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub secondary_hover: Hsla,

    // Muted & Accent states
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,

    // Destructive / Danger
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub danger: Hsla,
    pub danger_foreground: Hsla,
    pub danger_active: Hsla,

    // Status & Warnings
    pub warning: Hsla,
    pub warning_foreground: Hsla,
    pub warning_text: Hsla,
    pub chart_bullish: Hsla,
    pub success: Hsla,
    pub success_foreground: Hsla,

    // Luma surface roles, sourced from the FigCraft variable collections.
    pub control_fill: Hsla,
    pub checkbox_fill: Hsla,
    pub control_hover: Hsla,
    pub item_fill: Hsla,
    pub active_tab: Hsla,
    pub card_outline: Hsla,
    pub canvas: Hsla,
    pub metadata: Hsla,

    // Chart scale tokens (from shadcn DS Manager)
    pub chart_1: Hsla,
    pub chart_2: Hsla,
    pub chart_3: Hsla,
    pub chart_4: Hsla,
    pub chart_5: Hsla,

    // Borders, Inputs & Focus Rings
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
    pub selection: Hsla,

    // Dedicated Modern Shadcn Sidebar tokens
    pub sidebar_background: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_primary: Hsla,
    pub sidebar_primary_foreground: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_accent_foreground: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_ring: Hsla,

    // Messenger presence tokens
    pub presence_online: Hsla,
    pub presence_away: Hsla,
    pub presence_busy: Hsla,
    pub presence_offline: Hsla,
}

impl ThemeColors {
    /// Shadcn Luma Light Palette (pure contrasts, refined monochrome surfaces)
    pub fn luma_light() -> Self {
        let destructive = hsla(357.0 / 360.0, 1.0, 0.453, 1.0);
        let destructive_foreground = hsla(0.0, 0.0, 0.98, 1.0);
        let online = hsla(142.0 / 360.0, 0.76, 0.36, 1.0);
        let away = hsla(38.0 / 360.0, 0.92, 0.50, 1.0);

        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),
            foreground: hsla(0.0, 0.0, 0.039, 1.0),
            card: hsla(0.0, 0.0, 1.0, 1.0),
            card_foreground: hsla(0.0, 0.0, 0.039, 1.0),
            popover: hsla(0.0, 0.0, 1.0, 1.0),
            popover_foreground: hsla(0.0, 0.0, 0.039, 1.0),

            primary: hsla(0.0, 0.0, 0.09, 1.0),
            primary_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            secondary: hsla(0.0, 0.0, 0.961, 1.0),
            secondary_foreground: hsla(0.0, 0.0, 0.09, 1.0),
            secondary_hover: gpui::rgba(0xf5f5f5cc).into(),

            muted: hsla(0.0, 0.0, 0.961, 1.0),
            muted_foreground: hsla(0.0, 0.0, 0.451, 1.0),
            accent: hsla(0.0, 0.0, 0.961, 1.0),
            accent_foreground: hsla(0.0, 0.0, 0.09, 1.0),

            destructive,
            destructive_foreground,
            danger: destructive,
            danger_foreground: destructive_foreground,
            danger_active: hsla(357.0 / 360.0, 1.0, 0.38, 1.0),

            warning: away,
            warning_foreground: hsla(0.0, 0.0, 0.09, 1.0),
            warning_text: gpui::rgb(0x92400e).into(),
            chart_bullish: online,
            success: gpui::rgb(0x166534).into(),
            success_foreground: gpui::rgb(0xfafafa).into(),

            control_fill: gpui::rgba(0xe5e5e580).into(),
            checkbox_fill: gpui::rgba(0xe5e5e5e6).into(),
            control_hover: gpui::rgba(0xe5e5e5b3).into(),
            item_fill: gpui::rgba(0xf5f5f580).into(),
            active_tab: gpui::rgb(0xffffff).into(),
            card_outline: gpui::rgba(0x0a0a0a0d).into(),
            canvas: gpui::rgb(0xf5f5f5).into(),
            metadata: gpui::rgb(0x525252).into(),

            chart_1: hsla(0.0, 0.0, 0.83, 1.0),
            chart_2: hsla(0.0, 0.0, 0.45, 1.0),
            chart_3: hsla(0.0, 0.0, 0.32, 1.0),
            chart_4: hsla(0.0, 0.0, 0.25, 1.0),
            chart_5: hsla(0.0, 0.0, 0.15, 1.0),

            border: hsla(0.0, 0.0, 0.898, 1.0),
            input: hsla(0.0, 0.0, 0.898, 1.0),
            ring: hsla(0.0, 0.0, 0.63, 1.0),
            selection: hsla(217.0 / 360.0, 0.91, 0.60, 0.20),

            // Dedicated Modern Shadcn Sidebar tokens
            sidebar_background: hsla(0.0, 0.0, 0.98, 1.0),
            sidebar_foreground: hsla(0.0, 0.0, 0.039, 1.0),
            sidebar_primary: hsla(0.0, 0.0, 0.09, 1.0),
            sidebar_primary_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            sidebar_accent: hsla(0.0, 0.0, 0.961, 1.0),
            sidebar_accent_foreground: hsla(0.0, 0.0, 0.09, 1.0),
            sidebar_border: hsla(0.0, 0.0, 0.898, 1.0),
            sidebar_ring: hsla(0.0, 0.0, 0.63, 1.0),

            // Presence
            presence_online: online,
            presence_away: away,
            presence_busy: destructive,
            presence_offline: hsla(0.0, 0.0, 0.451, 1.0),
        }
    }

    /// Shadcn Luma Dark Palette (deep monochrome blacks, elevated luminous card/popover surfaces, ultra-fine micro-borders)
    pub fn luma_dark() -> Self {
        let destructive = hsla(359.0 / 360.0, 1.0, 0.696, 1.0);
        let destructive_foreground = hsla(0.0, 0.0, 0.039, 1.0);
        let online = hsla(142.0 / 360.0, 0.76, 0.45, 1.0);
        let away = hsla(38.0 / 360.0, 0.92, 0.50, 1.0);

        Self {
            background: hsla(0.0, 0.0, 0.039, 1.0),
            foreground: hsla(0.0, 0.0, 0.98, 1.0),
            card: hsla(0.0, 0.0, 0.09, 1.0),
            card_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            popover: hsla(0.0, 0.0, 0.09, 1.0),
            popover_foreground: hsla(0.0, 0.0, 0.98, 1.0),

            primary: hsla(0.0, 0.0, 0.898, 1.0),
            primary_foreground: hsla(0.0, 0.0, 0.09, 1.0),
            secondary: hsla(0.0, 0.0, 0.149, 1.0),
            secondary_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            secondary_hover: gpui::rgba(0x262626cc).into(),

            muted: hsla(0.0, 0.0, 0.149, 1.0),
            muted_foreground: hsla(0.0, 0.0, 0.63, 1.0),
            accent: hsla(0.0, 0.0, 0.149, 1.0),
            accent_foreground: hsla(0.0, 0.0, 0.98, 1.0),

            destructive,
            destructive_foreground,
            danger: destructive,
            danger_foreground: destructive_foreground,
            danger_active: hsla(359.0 / 360.0, 1.0, 0.58, 1.0),

            warning: away,
            warning_foreground: hsla(0.0, 0.0, 0.09, 1.0),
            warning_text: gpui::rgb(0xfcd34d).into(),
            chart_bullish: online,
            success: gpui::rgb(0x86efac).into(),
            success_foreground: gpui::rgb(0x0a0a0a).into(),

            control_fill: gpui::rgba(0xffffff13).into(),
            checkbox_fill: gpui::rgba(0xffffff22).into(),
            control_hover: gpui::rgba(0xffffff20).into(),
            item_fill: gpui::rgba(0x26262680).into(),
            active_tab: gpui::rgba(0xffffff0b).into(),
            card_outline: gpui::rgba(0xfafafa1a).into(),
            canvas: gpui::rgb(0x0a0a0a).into(),
            metadata: gpui::rgb(0xd4d4d4).into(),

            chart_1: hsla(0.0, 0.0, 0.83, 1.0),
            chart_2: hsla(0.0, 0.0, 0.45, 1.0),
            chart_3: hsla(0.0, 0.0, 0.32, 1.0),
            chart_4: hsla(0.0, 0.0, 0.25, 1.0),
            chart_5: hsla(0.0, 0.0, 0.15, 1.0),

            border: hsla(0.0, 0.0, 1.0, 0.10),
            input: hsla(0.0, 0.0, 1.0, 0.15),
            ring: hsla(0.0, 0.0, 0.451, 1.0),
            selection: hsla(217.0 / 360.0, 0.91, 0.60, 0.30),

            // Dedicated Modern Shadcn Sidebar tokens
            sidebar_background: hsla(0.0, 0.0, 0.09, 1.0),
            sidebar_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            sidebar_primary: hsla(225.0 / 360.0, 0.84, 0.49, 1.0),
            sidebar_primary_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            sidebar_accent: hsla(0.0, 0.0, 0.149, 1.0),
            sidebar_accent_foreground: hsla(0.0, 0.0, 0.98, 1.0),
            sidebar_border: hsla(0.0, 0.0, 1.0, 0.10),
            sidebar_ring: hsla(0.0, 0.0, 0.451, 1.0),

            // Presence
            presence_online: online,
            presence_away: away,
            presence_busy: destructive,
            presence_offline: hsla(0.0, 0.0, 0.63, 1.0),
        }
    }

    /// Modern Shadcn Luma Light Palette
    pub fn light() -> Self {
        Self::luma_light()
    }

    /// Modern Shadcn Luma Dark Palette
    pub fn dark() -> Self {
        Self::luma_dark()
    }
}

/// Border radius scale matching shadcn/ui Luma (soft, luminous corners).
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct ThemeRadius {
    pub none: Pixels,
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
    pub radius_2xl: Pixels,
    pub radius_3xl: Pixels,
    pub radius_4xl: Pixels,
    pub full: Pixels,
}

impl Default for ThemeRadius {
    fn default() -> Self {
        Self {
            none: px(0.),
            sm: px(6.),
            md: px(8.),
            lg: px(10.), // 0.625rem Luma base radius
            xl: px(14.),
            radius_2xl: px(18.),
            radius_3xl: px(22.),
            radius_4xl: px(26.),
            full: px(9999.),
        }
    }
}

/// The 17 Accent Color themes supported by shadcn/ui DS Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccentTheme {
    #[default]
    Neutral,
    Amber,
    Blue,
    Cyan,
    Emerald,
    Fuchsia,
    Green,
    Indigo,
    Lime,
    Orange,
    Pink,
    Purple,
    Red,
    Rose,
    Sky,
    Teal,
    Violet,
    Yellow,
}

impl AccentTheme {
    /// Returns the primary accent color for this theme.
    pub fn color(&self) -> Hsla {
        match self {
            Self::Neutral => hsla(0.0, 0.0, 0.09, 1.0),
            Self::Amber => hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
            Self::Blue => hsla(217.0 / 360.0, 0.91, 0.60, 1.0),
            Self::Cyan => hsla(189.0 / 360.0, 0.94, 0.43, 1.0),
            Self::Emerald => hsla(160.0 / 360.0, 0.84, 0.39, 1.0),
            Self::Fuchsia => hsla(292.0 / 360.0, 0.84, 0.61, 1.0),
            Self::Green => hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
            Self::Indigo => hsla(239.0 / 360.0, 0.84, 0.67, 1.0),
            Self::Lime => hsla(84.0 / 360.0, 0.81, 0.44, 1.0),
            Self::Orange => hsla(25.0 / 360.0, 0.95, 0.53, 1.0),
            Self::Pink => hsla(330.0 / 360.0, 0.81, 0.60, 1.0),
            Self::Purple => hsla(271.0 / 360.0, 0.91, 0.65, 1.0),
            Self::Red => hsla(0.0, 0.84, 0.60, 1.0),
            Self::Rose => hsla(350.0 / 360.0, 0.89, 0.60, 1.0),
            Self::Sky => hsla(199.0 / 360.0, 0.89, 0.48, 1.0),
            Self::Teal => hsla(173.0 / 360.0, 0.80, 0.40, 1.0),
            Self::Violet => hsla(258.0 / 360.0, 0.90, 0.66, 1.0),
            Self::Yellow => hsla(48.0 / 360.0, 0.96, 0.53, 1.0),
        }
    }
}

impl From<ThemeRadius> for Pixels {
    fn from(r: ThemeRadius) -> Self {
        r.md
    }
}

/// Grouped tokens for sidebar and surface components.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct ThemeTokens {
    pub sidebar: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_primary: Hsla,
}

impl ThemeTokens {
    pub fn from_colors(c: &ThemeColors) -> Self {
        Self {
            sidebar: c.sidebar_background,
            sidebar_accent: c.sidebar_accent,
            sidebar_border: c.sidebar_border,
            sidebar_foreground: c.sidebar_foreground,
            sidebar_primary: c.sidebar_primary,
        }
    }
}

impl From<ThemeRadius> for gpui::AbsoluteLength {
    fn from(r: ThemeRadius) -> Self {
        r.md.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luma_light_palette() {
        let light = ThemeColors::light();
        assert_eq!(light, ThemeColors::luma_light());
        assert_eq!(light.background.l, 1.0);
        assert_eq!(light.card.l, 1.0);
        assert_eq!(light.popover.l, 1.0);
        assert!((light.foreground.l - 0.039).abs() < 0.001);
        assert!((light.border.l - 0.898).abs() < 0.002);
    }

    #[test]
    fn test_luma_dark_palette_micro_borders() {
        let dark = ThemeColors::dark();
        assert_eq!(dark, ThemeColors::luma_dark());
        assert!((dark.background.l - 0.039).abs() < 0.001);
        assert!((dark.card.l - 0.09).abs() < 0.001);
        assert!((dark.popover.l - 0.09).abs() < 0.001);
        assert!((dark.foreground.l - 0.98).abs() < 0.001);

        // Verify ultra-fine micro-borders
        assert_eq!(dark.border.a, 0.10);
        assert_eq!(dark.input.a, 0.15);
        assert_eq!(dark.sidebar_border.a, 0.10);
    }

    #[test]
    fn test_luma_radius_scale() {
        let radius = ThemeRadius::default();
        assert_eq!(radius.none, px(0.));
        assert_eq!(radius.sm, px(6.));
        assert_eq!(radius.md, px(8.));
        assert_eq!(radius.lg, px(10.));
        assert_eq!(radius.xl, px(14.));
        assert_eq!(radius.radius_2xl, px(18.));
        assert_eq!(radius.radius_3xl, px(22.));
        assert_eq!(radius.radius_4xl, px(26.));
        assert_eq!(radius.full, px(9999.));
        assert_eq!(Pixels::from(radius), px(8.));
    }

    #[test]
    fn test_theme_tokens_projection() {
        let dark = ThemeColors::dark();
        let tokens = ThemeTokens::from_colors(&dark);
        assert_eq!(tokens.sidebar, dark.sidebar_background);
        assert_eq!(tokens.sidebar_border, dark.sidebar_border);
        assert_eq!(tokens.sidebar_primary, dark.sidebar_primary);
    }

    #[test]
    fn palette_matches_figcraft_variable_snapshot() {
        let source: serde_json::Value =
            serde_json::from_str(include_str!("../../design/figcraft-tokens.json")).unwrap();
        for (mode, colors) in [("Light", ThemeColors::light()), ("Dark", ThemeColors::dark())] {
            for (name, actual) in [
                ("background", colors.background),
                ("foreground", colors.foreground),
                ("primary", colors.primary),
                ("primary-foreground", colors.primary_foreground),
                ("border", colors.border),
                ("sidebar", colors.sidebar_background),
                ("control-fill", colors.control_fill),
                ("checkbox-fill", colors.checkbox_fill),
                ("secondary-hover", colors.secondary_hover),
                ("control-hover", colors.control_hover),
                ("item-fill", colors.item_fill),
                ("active-tab", colors.active_tab),
                ("card-outline", colors.card_outline),
                ("canvas", colors.canvas),
                ("flow/metadata", colors.metadata),
                ("flow/success", colors.success),
                ("flow/decision", colors.warning_text),
            ] {
                let actual: gpui::Rgba = actual.into();
                let expected = &source["tokens"][name]["values"][mode];
                for (channel, actual) in
                    [("r", actual.r), ("g", actual.g), ("b", actual.b), ("a", actual.a)]
                {
                    let expected = expected[channel].as_f64().unwrap() as f32;
                    assert!((actual - expected).abs() < 0.003, "{mode} {name} {channel}");
                }
            }
        }
    }

    #[test]
    fn filled_status_controls_keep_readable_labels_in_both_themes() {
        fn luminance(color: Hsla) -> f32 {
            let color: gpui::Rgba = color.into();
            let linear = |c: f32| {
                if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
            };
            0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
        }
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for (foreground, background) in [
                (colors.primary_foreground, colors.primary),
                (colors.destructive_foreground, colors.destructive),
                (colors.warning_foreground, colors.warning),
                (colors.success_foreground, colors.success),
            ] {
                let a = luminance(foreground);
                let b = luminance(background);
                let contrast = (a.max(b) + 0.05) / (a.min(b) + 0.05);
                assert!(contrast >= 4.5, "label contrast {contrast}");
            }
        }
    }
}
