pub mod tokens;

use gpui::{App, Context, Global, Pixels, Window, px};
use std::ops::Deref;
pub use tokens::{AccentTheme, ThemeColors, ThemeRadius, ThemeTokens};

use crate::primitives::traits::ThemeMode;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeAppearance {
    Light,
    #[default]
    Dark,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Theme {
    pub appearance: ThemeAppearance,
    pub colors: ThemeColors,
    pub tokens: ThemeTokens,
    pub font_size: Pixels,
    pub radius: Pixels,
    pub radius_sm: Pixels,
    pub radius_checkbox: Pixels,
    pub radius_md: Pixels,
    pub radius_lg: Pixels,
    pub radius_xl: Pixels,
    pub radius_2xl: Pixels,
    pub radius_3xl: Pixels,
    pub radius_4xl: Pixels,
    pub shadow: bool,
}

impl Deref for Theme {
    type Target = ThemeColors;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Global for Theme {}

impl Theme {
    pub fn luma_light() -> Self {
        let colors = ThemeColors::luma_light();
        let tokens = ThemeTokens::from_colors(&colors);
        let radius = ThemeRadius::default();
        Self {
            appearance: ThemeAppearance::Light,
            colors,
            tokens,
            font_size: px(14.),
            radius: radius.md,
            radius_sm: radius.sm,
            radius_checkbox: px(5.),
            radius_md: radius.md,
            radius_lg: radius.lg,
            radius_xl: radius.xl,
            radius_2xl: radius.radius_2xl,
            radius_3xl: radius.radius_3xl,
            radius_4xl: radius.radius_4xl,
            shadow: true,
        }
    }

    pub fn luma_dark() -> Self {
        let colors = ThemeColors::luma_dark();
        let tokens = ThemeTokens::from_colors(&colors);
        let radius = ThemeRadius::default();
        Self {
            appearance: ThemeAppearance::Dark,
            colors,
            tokens,
            font_size: px(14.),
            radius: radius.md,
            radius_sm: radius.sm,
            radius_checkbox: px(5.),
            radius_md: radius.md,
            radius_lg: radius.lg,
            radius_xl: radius.xl,
            radius_2xl: radius.radius_2xl,
            radius_3xl: radius.radius_3xl,
            radius_4xl: radius.radius_4xl,
            shadow: true,
        }
    }

    pub fn light() -> Self {
        Self::luma_light()
    }

    pub fn dark() -> Self {
        Self::luma_dark()
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        if !cx.has_global::<Self>() {
            cx.set_global(Self::default());
        }
        cx.global_mut::<Self>()
    }

    pub fn set_global(cx: &mut App, theme: Self) {
        let mut theme = theme;
        theme.tokens = ThemeTokens::from_colors(&theme.colors);
        cx.set_global(theme);
        Self::sync_base(cx);
    }

    /// Applies the user's base size and corner preference to the complete Luma scale.
    pub fn set_scale(&mut self, font_size: Pixels, radius: Pixels) {
        self.font_size = font_size;
        let scale = f32::from(font_size) / 14. * f32::from(radius) / 8.;
        let radii = ThemeRadius::default();
        self.radius = radii.md * scale;
        self.radius_sm = radii.sm * scale;
        self.radius_checkbox = px(5.) * scale;
        self.radius_md = radii.md * scale;
        self.radius_lg = radii.lg * scale;
        self.radius_xl = radii.xl * scale;
        self.radius_2xl = radii.radius_2xl * scale;
        self.radius_3xl = radii.radius_3xl * scale;
        self.radius_4xl = radii.radius_4xl * scale;
    }

    pub fn sync_system_appearance(window: Option<&mut Window>, cx: &mut App) {
        let is_light = if let Some(w) = window {
            matches!(
                w.appearance(),
                gpui::WindowAppearance::Light | gpui::WindowAppearance::VibrantLight
            )
        } else {
            let mut found_light = None;
            for handle in cx.windows() {
                let _ = handle.update(cx, |_view, window, _cx| {
                    found_light = Some(matches!(
                        window.appearance(),
                        gpui::WindowAppearance::Light | gpui::WindowAppearance::VibrantLight
                    ));
                });
                if found_light.is_some() {
                    break;
                }
            }
            found_light.unwrap_or(false)
        };
        let theme = if is_light { Self::light() } else { Self::dark() };
        Self::set_global(cx, theme);
    }

    pub fn change(mode: ThemeMode, _window: Option<&mut Window>, cx: &mut App) {
        let theme = match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        };
        Self::set_global(cx, theme);
    }

    pub fn sync_base(cx: &mut App) {
        if !cx.has_global::<Self>() || !cx.has_global::<gpui_base::Theme>() {
            return;
        }

        let theme = Self::global(cx).clone();
        let mut base_theme = gpui_base::Theme::global(cx);
        base_theme.appearance = match theme.appearance {
            ThemeAppearance::Dark => gpui_base::ThemeAppearance::Dark,
            ThemeAppearance::Light => gpui_base::ThemeAppearance::Light,
        };
        base_theme.tokens.typography.sans = "Inter".into();
        base_theme.tokens.colors.background = theme.background;
        base_theme.tokens.colors.foreground = theme.foreground;
        base_theme.tokens.colors.primary = theme.primary;
        base_theme.tokens.colors.primary_foreground = theme.primary_foreground;
        base_theme.tokens.colors.secondary = theme.secondary;
        base_theme.tokens.colors.secondary_foreground = theme.secondary_foreground;
        base_theme.tokens.colors.muted = theme.muted;
        base_theme.tokens.colors.muted_foreground = theme.muted_foreground;
        base_theme.tokens.colors.accent = theme.accent;
        base_theme.tokens.colors.accent_foreground = theme.accent_foreground;
        base_theme.tokens.colors.destructive = theme.destructive;
        base_theme.tokens.colors.destructive_foreground = theme.destructive_foreground;
        base_theme.tokens.colors.border = theme.border;
        base_theme.tokens.colors.input = theme.input;
        base_theme.tokens.colors.ring = theme.ring;
        base_theme.tokens.colors.selection = theme.selection;

        base_theme.tokens.radius.sm = theme.radius_sm;
        base_theme.tokens.radius.md = theme.radius;
        base_theme.tokens.radius.lg = theme.radius_lg;
        base_theme.tokens.radius.xl = theme.radius_xl;
        base_theme.tokens.radius.full = gpui::px(9999.);

        cx.set_global(base_theme);
    }
}

/// Helper trait to access current theme tokens ergonomically.
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    #[inline(always)]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

impl<'a, T> ActiveTheme for Context<'a, T> {
    #[inline(always)]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_luma_constructors() {
        let light = Theme::luma_light();
        assert_eq!(light.appearance, ThemeAppearance::Light);
        assert_eq!(light.radius, px(8.));
        assert_eq!(light.radius_lg, px(10.));
        assert_eq!(light.radius_xl, px(14.));
        assert_eq!(light.radius_2xl, px(18.));
        assert_eq!(light.radius_3xl, px(22.));
        assert_eq!(light.radius_4xl, px(26.));

        let dark = Theme::luma_dark();
        assert_eq!(dark.appearance, ThemeAppearance::Dark);
        assert_eq!(dark.radius, px(8.));
        assert_eq!(dark.radius_lg, px(10.));
        assert_eq!(dark.radius_xl, px(14.));
        assert_eq!(dark.radius_2xl, px(18.));
        assert_eq!(dark.radius_3xl, px(22.));
        assert_eq!(dark.radius_4xl, px(26.));
    }

    #[test]
    fn test_theme_deref_to_colors() {
        let dark = Theme::dark();
        assert_eq!(dark.card, dark.colors.card);
        assert_eq!(dark.border, dark.colors.border);
        assert_eq!(dark.primary, dark.colors.primary);
    }

    #[test]
    fn scale_updates_controls_and_surfaces_together() {
        let mut theme = Theme::light();
        theme.set_scale(px(21.), px(8.));
        assert_eq!(theme.font_size, px(21.));
        assert_eq!(theme.radius, px(12.));
        assert_eq!(theme.radius_checkbox, px(7.5));
        assert_eq!(theme.radius_4xl, px(39.));
        theme.set_scale(px(14.), px(0.));
        assert_eq!(theme.radius_sm, px(0.));
        assert_eq!(theme.radius_4xl, px(0.));
    }
}
