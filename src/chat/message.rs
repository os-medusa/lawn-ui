use gpui::prelude::*;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Refineable, RenderOnce, StyleRefinement, Styled,
    Window, div,
};

pub use crate::chat::bubble::MessageAlignment;

#[derive(IntoElement, Default)]
pub struct MessageHeader {
    child: Option<AnyElement>,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for MessageHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct MessageContent {
    child: Option<AnyElement>,
}

impl MessageContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for MessageContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct MessageFooter {
    child: Option<AnyElement>,
}

impl MessageFooter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content_inset(self, _inset: bool) -> Self {
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for MessageFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().children(self.child)
    }
}

#[derive(IntoElement, Default)]
pub struct Message {
    alignment: MessageAlignment,
    avatar: Option<AnyElement>,
    header: Option<MessageHeader>,
    content: Option<MessageContent>,
    footer: Option<MessageFooter>,
    style: StyleRefinement,
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alignment(mut self, alignment: MessageAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn avatar(mut self, avatar: impl IntoElement) -> Self {
        self.avatar = Some(avatar.into_any_element());
        self
    }

    pub fn header(mut self, header: MessageHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn content(mut self, content: MessageContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn footer(mut self, footer: MessageFooter) -> Self {
        self.footer = Some(footer);
        self
    }
}

impl Styled for Message {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Message {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_end = matches!(self.alignment, MessageAlignment::End);
        let content_col = div()
            .flex()
            .flex_col()
            .flex_1()
            .children(self.header)
            .children(self.content)
            .children(self.footer);

        let (avatar_left, avatar_right) =
            if is_end { (None, self.avatar) } else { (self.avatar, None) };

        div()
            .flex()
            .flex_row()
            .w_full()
            .gap_2()
            .items_start()
            .when(is_end, |this| this.justify_end())
            .when(!is_end, |this| this.justify_start())
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(avatar_left)
            .child(content_col)
            .children(avatar_right)
    }
}
