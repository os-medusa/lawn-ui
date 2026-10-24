use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Pixels, Refineable, RenderOnce, StyleRefinement,
    Styled, Window, div, px,
};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

/// Modern Shadcn Sheet (Slide-over panel)
#[derive(IntoElement)]
pub struct Sheet {
    side: SheetSide,
    size: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            side: SheetSide::Right,
            size: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = Some(size);
        self
    }

    pub(crate) fn side_value(&self) -> SheetSide {
        self.side
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Sheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Sheet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sheet {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let default_size = px(380.);
        let size = self.size.unwrap_or(default_size);

        let mut el = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .bg(theme.background)
            .text_color(theme.foreground)
            .when(theme.shadow, |surface| surface.shadow_xl());

        el = match self.side {
            SheetSide::Right => el.w(size).h_full().border_l_1().border_color(theme.border),
            SheetSide::Left => el.w(size).h_full().border_r_1().border_color(theme.border),
            SheetSide::Top => el.h(size).w_full().border_b_1().border_color(theme.border),
            SheetSide::Bottom => el.h(size).w_full().border_t_1().border_color(theme.border),
        };

        el.map(|mut el| {
            el.style().refine(&self.style);
            el
        })
        .children(self.children)
    }
}

/// Header section of a Sheet.
#[derive(IntoElement)]
pub struct SheetHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetHeader {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SheetHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetHeader {
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

/// Title element inside a Sheet.
#[derive(IntoElement)]
pub struct SheetTitle {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetTitle {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SheetTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_lg()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(theme.foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Description text inside a Sheet.
#[derive(IntoElement)]
pub struct SheetDescription {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetDescription {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SheetDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_sm()
            .text_color(theme.muted_foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Main content section of a Sheet.
#[derive(IntoElement)]
pub struct SheetContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetContent {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SheetContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .py_2()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Action footer section of a Sheet.
#[derive(IntoElement)]
pub struct SheetFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetFooter {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SheetFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetFooter {
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
