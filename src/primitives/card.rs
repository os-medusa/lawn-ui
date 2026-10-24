use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Refineable, RenderOnce, StyleRefinement, Styled,
    Window, div,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn Card container.
#[derive(IntoElement)]
pub struct Card {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_col()
            .bg(theme.card)
            .text_color(theme.card_foreground)
            .border_1()
            .border_color(theme.card_outline)
            .rounded(theme.radius_4xl)
            .when(theme.shadow, |surface| surface.shadow(vec![
                gpui::BoxShadow::new(gpui::px(0.), gpui::px(4.), gpui::black().opacity(0.05))
                    .blur_radius(gpui::px(6.)),
            ]))
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Header section of a Card.
#[derive(IntoElement)]
pub struct CardHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardHeader {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_4()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Title element inside a Card.
#[derive(IntoElement)]
pub struct CardTitle {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardTitle {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for CardTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .font_weight(gpui::FontWeight::NORMAL)
            .text_size(gpui::rems(16. / 14.))
            .text_color(theme.foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Description text inside a Card.
#[derive(IntoElement)]
pub struct CardDescription {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardDescription {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for CardDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardDescription {
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

/// Content section of a Card.
#[derive(IntoElement)]
pub struct CardContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardContent {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for CardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .p_4()
            .pt_0()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Footer section of a Card.
#[derive(IntoElement)]
pub struct CardFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardFooter {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .p_4()
            .pt_0()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}
