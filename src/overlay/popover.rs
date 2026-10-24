use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Pixels, Refineable, RenderOnce, StyleRefinement,
    Styled, Window, div, px,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn Popover Card
#[derive(IntoElement)]
pub struct Popover {
    width: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Popover {
    pub fn new() -> Self {
        Self { width: None, style: StyleRefinement::default(), children: Vec::new() }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Popover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let width = self.width.unwrap_or(px(280.));

        div()
            .flex()
            .flex_col()
            .w(width)
            .p_4()
            .rounded(theme.radius_2xl)
            .border_1()
            .border_color(theme.card_outline)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .when(theme.shadow, |surface| surface.shadow_md())
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}
