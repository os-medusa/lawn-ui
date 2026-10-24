use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, AnyView, App, AppContext, Context, IntoElement, ParentElement, Refineable, Render,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn Tooltip content box
#[derive(IntoElement)]
pub struct Tooltip {
    text: Option<SharedString>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Tooltip {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: Some(text.into()), style: StyleRefinement::default(), children: Vec::new() }
    }

    pub fn empty() -> Self {
        Self { text: None, style: StyleRefinement::default(), children: Vec::new() }
    }

    pub fn build(self, _window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| self).into()
    }
}

impl Styled for Tooltip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Render for Tooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = div()
            .flex()
            .items_center()
            .px_3()
            .py_1()
            .rounded(theme.radius_sm)
            .bg(theme.primary)
            .text_color(theme.primary_foreground)
            .text_xs()
            .when(theme.shadow, |surface| surface.shadow_md())
            .flex_shrink_0();

        if let Some(ref text) = self.text {
            el = el.child(text.clone());
        }

        el.map(|mut el| {
            el.style().refine(&self.style);
            el
        })
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = div()
            .flex()
            .items_center()
            .px_3()
            .py_1()
            .rounded(theme.radius_sm)
            .bg(theme.primary)
            .text_color(theme.primary_foreground)
            .text_xs()
            .when(theme.shadow, |surface| surface.shadow_md())
            .flex_shrink_0();

        if let Some(text) = self.text {
            el = el.child(text);
        }

        el.map(|mut el| {
            el.style().refine(&self.style);
            el
        })
        .children(self.children)
    }
}
