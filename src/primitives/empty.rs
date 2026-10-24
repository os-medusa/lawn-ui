use gpui::prelude::*;
use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px, rems};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Icon,
    Illustration,
}

#[derive(IntoElement)]
pub struct EmptyMedia {
    variant: EmptyMediaVariant,
    child: Option<AnyElement>,
}

impl Default for EmptyMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyMedia {
    pub fn new() -> Self {
        Self { variant: EmptyMediaVariant::Icon, child: None }
    }

    pub fn with_variant(mut self, variant: EmptyMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for EmptyMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let is_icon = matches!(self.variant, EmptyMediaVariant::Icon);

        div()
            .flex()
            .items_center()
            .justify_center()
            .when(is_icon, |this| {
                this.size(rems(40. / 14.))
                    .rounded(theme.radius_2xl)
                    .bg(theme.secondary)
            })
            .children(self.child)
    }
}

#[derive(IntoElement)]
pub struct EmptyTitle {
    child: Option<AnyElement>,
}

impl Default for EmptyTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyTitle {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for EmptyTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(gpui::rems(18. / 14.))
            .font_weight(gpui::FontWeight::NORMAL)
            .text_color(theme.foreground)
            .children(self.child)
    }
}

#[derive(IntoElement)]
pub struct EmptyDescription {
    child: Option<AnyElement>,
}

impl Default for EmptyDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyDescription {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for EmptyDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(gpui::rems(1.))
            .text_color(theme.muted_foreground)
            .max_w(px(320.))
            .text_center()
            .children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct EmptyHeader {
    media: Option<EmptyMedia>,
    title: Option<EmptyTitle>,
    description: Option<EmptyDescription>,
}

impl EmptyHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn media(mut self, media: EmptyMedia) -> Self {
        self.media = Some(media);
        self
    }

    pub fn title(mut self, title: EmptyTitle) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, desc: EmptyDescription) -> Self {
        self.description = Some(desc);
        self
    }
}

impl RenderOnce for EmptyHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .children(self.media)
            .children(self.title)
            .children(self.description)
    }
}

#[derive(IntoElement, Default)]
pub struct Empty {
    header: Option<EmptyHeader>,
    children: Vec<AnyElement>,
}

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, header: EmptyHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for Empty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .p(rems(24. / 14.))
            .rounded(theme.radius_4xl)
            .border_1()
            .border_color(theme.card_outline)
            .bg(theme.card)
            .text_color(theme.card_foreground)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .children(self.header)
            .children(self.children)
    }
}
