use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

use crate::primitives::traits::{Sizable, Size};
use crate::theme::ActiveTheme;

#[derive(IntoElement, Default)]
pub struct AttachmentMedia {
    child: Option<AnyElement>,
}

impl AttachmentMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for AttachmentMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .size_10()
            .rounded(theme.radius)
            .bg(theme.muted)
            .flex()
            .items_center()
            .justify_center()
            .children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct AttachmentTitle {
    child: Option<AnyElement>,
}

impl AttachmentTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { child: Some(text.into().into_any_element()) }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for AttachmentTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_base()
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(theme.foreground)
            .truncate()
            .children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct AttachmentDescription {
    child: Option<AnyElement>,
}

impl AttachmentDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { child: Some(text.into().into_any_element()) }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for AttachmentDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div().text_sm().text_color(theme.metadata).truncate().children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct AttachmentContent {
    title: Option<AttachmentTitle>,
    description: Option<AttachmentDescription>,
}

impl AttachmentContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: AttachmentTitle) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, desc: AttachmentDescription) -> Self {
        self.description = Some(desc);
        self
    }
}

impl RenderOnce for AttachmentContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_hidden()
            .children(self.title)
            .children(self.description)
    }
}

#[derive(IntoElement, Default)]
pub struct Attachment {
    size: Size,
    media: Option<AttachmentMedia>,
    content: Option<AttachmentContent>,
    action: Option<AnyElement>,
}

impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn media(mut self, media: AttachmentMedia) -> Self {
        self.media = Some(media);
        self
    }

    pub fn content(mut self, content: AttachmentContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl Sizable for Attachment {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Attachment {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .items_center()
            .gap_3()
            .p_2p5()
            .rounded_xl()
            .bg(theme.card)
            .border_1()
            .border_color(theme.border)
            .children(self.media)
            .children(self.content)
            .children(self.action)
    }
}
