use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Pixels, Refineable, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn Dialog Container
#[derive(IntoElement)]
pub struct Dialog {
    width: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new() -> Self {
        Self { width: None, style: StyleRefinement::default(), children: Vec::new() }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.children.push(DialogTitle::new().child(title.into()).into_any_element());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.children.push(DialogDescription::new().child(description.into()).into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.children.push(footer.into_any_element());
        self
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Dialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let width = self.width.unwrap_or(px(480.));

        div()
            .flex()
            .flex_col()
            .gap_4()
            .w(width)
            .p_6()
            .rounded(theme.radius_4xl)
            .border_1()
            .border_color(theme.card_outline)
            .bg(theme.popover)
            .text_color(theme.foreground)
            .when(theme.shadow, |surface| surface.shadow_lg())
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Header section of a Dialog.
#[derive(IntoElement)]
pub struct DialogHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogHeader {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1p5()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Title element inside a Dialog.
#[derive(IntoElement)]
pub struct DialogTitle {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogTitle {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for DialogTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(gpui::rems(18. / 14.))
            .font_weight(gpui::FontWeight::NORMAL)
            .text_color(theme.foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Description text inside a Dialog.
#[derive(IntoElement)]
pub struct DialogDescription {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogDescription {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for DialogDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(gpui::rems(1.))
            .text_color(theme.muted_foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Main content section of a Dialog.
#[derive(IntoElement)]
pub struct DialogContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogContent {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for DialogContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .py_2()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Action footer section of a Dialog.
#[derive(IntoElement)]
pub struct DialogFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_end()
            .gap_2()
            .pt_2()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}
