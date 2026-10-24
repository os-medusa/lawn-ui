use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, Hsla, ImageSource, IntoElement, ParentElement, Pixels, Refineable, RenderOnce,
    SharedString, StyleRefinement, Styled, StyledImage, Window, div, img, px,
};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    Xs,  // 20px
    Sm,  // 24px
    Md,  // 32px (default)
    Lg,  // 40px
    Xl,  // 48px
    Xxl, // 64px
    Custom(u32),
}

impl AvatarSize {
    /// Returns the nominal dimension at the 14 px application baseline.
    pub fn to_pixels(self) -> Pixels {
        match self {
            Self::Xs => px(20.),
            Self::Sm => px(24.),
            Self::Md => px(32.),
            Self::Lg => px(40.),
            Self::Xl => px(48.),
            Self::Xxl => px(64.),
            Self::Custom(val) => px(val as f32),
        }
    }

    pub fn dot_size(self) -> Pixels {
        match self {
            Self::Xs | Self::Sm => px(6.),
            Self::Md => px(8.),
            Self::Lg | Self::Xl => px(10.),
            Self::Xxl => px(12.),
            Self::Custom(val) => px((val as f32 * 0.25).max(6.)),
        }
    }

    fn resolved_size(self, window: &Window) -> Pixels {
        match self {
            Self::Custom(_) => self.to_pixels(),
            _ => self.to_pixels() * (window.rem_size() / px(14.)),
        }
    }

    fn resolved_dot_size(self, window: &Window) -> Pixels {
        match self {
            Self::Custom(_) => self.dot_size(),
            _ => self.dot_size() * (window.rem_size() / px(14.)),
        }
    }
}

impl From<Pixels> for AvatarSize {
    fn from(px: Pixels) -> Self {
        AvatarSize::Custom(f32::from(px) as u32)
    }
}

/// A bounded profile image or initials, optionally with a presence or status indicator.
#[derive(IntoElement)]
pub struct Avatar {
    image: Option<ImageSource>,
    fallback: Option<SharedString>,
    size: AvatarSize,
    presence: Option<PresenceStatus>,
    status_color: Option<Hsla>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            image: None,
            fallback: None,
            size: AvatarSize::Md,
            presence: None,
            status_color: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn image(mut self, src: impl Into<ImageSource>) -> Self {
        self.image = Some(src.into());
        self
    }

    /// Displays an explicit custom label inside the avatar's clipped boundary.
    pub fn fallback(mut self, text: impl Into<SharedString>) -> Self {
        self.fallback = Some(text.into());
        self
    }

    /// Derives one or two initials from the supplied display name.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        self.fallback = Some(initials(&name).into());
        self
    }

    pub fn size(mut self, size: impl Into<AvatarSize>) -> Self {
        self.size = size.into();
        self
    }

    pub fn xs(self) -> Self {
        self.size(AvatarSize::Xs)
    }

    pub fn sm(self) -> Self {
        self.size(AvatarSize::Sm)
    }

    pub fn md(self) -> Self {
        self.size(AvatarSize::Md)
    }

    pub fn lg(self) -> Self {
        self.size(AvatarSize::Lg)
    }

    pub fn xl(self) -> Self {
        self.size(AvatarSize::Xl)
    }

    pub fn xsmall(self) -> Self {
        self.size(AvatarSize::Xs)
    }

    pub fn small(self) -> Self {
        self.size(AvatarSize::Sm)
    }

    pub fn medium(self) -> Self {
        self.size(AvatarSize::Md)
    }

    pub fn large(self) -> Self {
        self.size(AvatarSize::Lg)
    }

    pub fn xxl(self) -> Self {
        self.size(AvatarSize::Xxl)
    }

    /// Sets an explicit status indicator color on the avatar.
    pub fn status_color(mut self, color: impl Into<Hsla>) -> Self {
        self.status_color = Some(color.into());
        self
    }

    /// Alias for [`Self::status_color`].
    pub fn status(self, color: impl Into<Hsla>) -> Self {
        self.status_color(color)
    }

    /// Sets the status dot using standard presence semantic states.
    pub fn presence(mut self, status: Option<PresenceStatus>) -> Self {
        self.presence = status;
        self
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Avatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Avatar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

fn initials(name: &str) -> String {
    let mut words = name
        .split_whitespace()
        .filter_map(|word| word.chars().find(|character| character.is_alphanumeric()));
    let Some(first) = words.next() else {
        return "?".to_string();
    };
    let last = words.next_back();
    std::iter::once(first).chain(last).flat_map(char::to_uppercase).take(2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Context, InteractiveElement, Render, TestAppContext};
    use gpui_base::TestSupportExt;

    #[test]
    fn names_use_bounded_unicode_initials() {
        for (name, expected) in [
            ("Александр Владимирович Лебедев", "АЛ"),
            ("  анна   смирнова  ", "АС"),
            ("оченьдлинноеимябезпробелов", "О"),
            ("山田 太郎", "山太"),
            ("产品设计团队", "产"),
            ("", "?"),
            (" \t\n ", "?"),
            ("… — 🙂", "?"),
            ("ßeta team", "SS"),
        ] {
            let avatar = Avatar::new().name(name);
            assert_eq!(avatar.fallback.as_deref(), Some(expected), "name: {name}");
            assert!(avatar.fallback.as_ref().unwrap().chars().count() <= 2);
        }
    }

    #[test]
    fn explicit_fallback_keeps_the_supplied_label() {
        let avatar = Avatar::new().name("Александр Лебедев").fallback("Команда продукта");
        assert_eq!(avatar.fallback.as_deref(), Some("Команда продукта"));
    }

    struct Avatars;

    impl Render for Avatars {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .items_start()
                .child(
                    div()
                        .id("semantic-avatar")
                        .test_support()
                        .flex()
                        .child(Avatar::new().name("Длинное название команды продукта").md()),
                )
                .child(
                    div()
                        .id("custom-avatar")
                        .test_support()
                        .flex()
                        .child(Avatar::new().fallback("Custom label").size(px(36.))),
                )
        }
    }

    #[gpui::test]
    fn semantic_avatars_zoom_and_custom_dimensions_stay_fixed(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, _| Avatars);
        for (rem, expected_semantic) in [(14., 32.), (21., 48.)] {
            cx.update(|window, cx| {
                window.set_rem_size(px(rem));
                window.draw(cx).clear(cx);
                let semantic =
                    gpui_base::test_support::find(window, &[], &"semantic-avatar".into())
                        .unwrap()
                        .bounds();
                let custom = gpui_base::test_support::find(window, &[], &"custom-avatar".into())
                    .unwrap()
                    .bounds();
                assert_eq!(semantic.size.width, px(expected_semantic));
                assert_eq!(semantic.size.height, px(expected_semantic));
                assert_eq!(custom.size.width, px(36.));
                assert_eq!(custom.size.height, px(36.));
            });
        }
    }
}

impl RenderOnce for Avatar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let px_size = self.size.resolved_size(window);

        let avatar_circle = if let Some(src) = self.image {
            div()
                .size_full()
                .rounded_full()
                .overflow_hidden()
                .bg(theme.muted)
                .child(img(src).size_full().object_fit(gpui::ObjectFit::Cover))
        } else {
            let initials =
                self.fallback.filter(|text| !text.trim().is_empty()).unwrap_or_else(|| "?".into());
            div()
                .size_full()
                .rounded_full()
                .overflow_hidden()
                .bg(theme.muted)
                .text_color(theme.muted_foreground)
                .font_weight(gpui::FontWeight::MEDIUM)
                .flex()
                .items_center()
                .justify_center()
                .text_size(px_size * 0.375)
                .child(div().w_full().min_w_0().text_center().truncate().child(initials))
        };

        let container = div()
            .relative()
            .size(px_size)
            .flex_shrink_0()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .child(avatar_circle);

        let effective_status = self.status_color.or_else(|| {
            self.presence.map(|presence| match presence {
                PresenceStatus::Online => theme.presence_online,
                PresenceStatus::Away => theme.presence_away,
                PresenceStatus::Busy => theme.presence_busy,
                PresenceStatus::Offline => theme.presence_offline,
            })
        });

        if let Some(dot_color) = effective_status {
            let dot_size = self.size.resolved_dot_size(window);

            container.child(
                div()
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .size(dot_size)
                    .rounded_full()
                    .bg(dot_color)
                    .border_1()
                    .border_color(theme.background),
            )
        } else {
            container
        }
    }
}
