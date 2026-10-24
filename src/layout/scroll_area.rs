use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
};

/// Modern ScrollArea container with kinetic scrolling
#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ScrollArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Styled for ScrollArea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ScrollArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .flex_1()
            .size_full()
            .overflow_y_scroll()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}
