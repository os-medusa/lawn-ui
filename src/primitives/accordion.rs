use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// An accordion component allowing expandable/collapsible content sections.
#[derive(IntoElement, Default)]
pub struct Accordion {
    children: Vec<AnyElement>,
}

impl Accordion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .w_full()
            .flex()
            .flex_col()
            .rounded(theme.radius_2xl)
            .bg(theme.item_fill)
            .overflow_hidden()
            .children(self.children)
    }
}

/// An individual section within an [`Accordion`].
#[derive(IntoElement, Default)]
pub struct AccordionItem {
    children: Vec<AnyElement>,
}

impl AccordionItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl RenderOnce for AccordionItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .w_full()
            .border_b_1()
            .border_color(theme.border)
            .children(self.children)
    }
}

/// The clickable header that toggles an [`AccordionItem`].
#[derive(IntoElement)]
pub struct AccordionTrigger {
    id: ElementId,
    open: bool,
    title: SharedString,
    on_toggle: Option<Rc<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl AccordionTrigger {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            open: false,
            title: title.into(),
            on_toggle: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for AccordionTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let icon_name = if self.open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let mut el = div()
            .id(self.id)
            .flex()
            .w_full()
            .items_center()
            .justify_between()
            .py(rems(16. / 14.))
            .font_medium()
            .text_size(gpui::rems(1.))
            .text_color(theme.foreground)
            .hover(|s| s.underline())
            .cursor_pointer();

        if let Some(on_toggle) = self.on_toggle {
            el = el.on_click(move |ev, window, cx| (on_toggle)(ev, window, cx));
        }

        el.child(self.title).child(
            Icon::new(icon_name)
                .size(IconSize::Sm)
                .text_color(theme.muted_foreground),
        )
    }
}

/// The collapsible body content of an [`AccordionItem`].
#[derive(IntoElement, Default)]
pub struct AccordionContent {
    open: bool,
    children: Vec<AnyElement>,
}

impl AccordionContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl RenderOnce for AccordionContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        if !self.open {
            return div().into_any_element();
        }

        div()
            .w_full()
            .pb(rems(16. / 14.))
            .pt(rems(2. / 14.))
            .text_size(gpui::rems(1.))
            .text_color(theme.muted_foreground)
            .children(self.children)
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_construction() {
        let acc = Accordion::new()
            .child(
                AccordionItem::new()
                    .child(AccordionTrigger::new("q1", "Question 1").open(true))
                    .child(AccordionContent::new().open(true).child("Answer 1")),
            )
            .child(
                AccordionItem::new()
                    .child(AccordionTrigger::new("q2", "Question 2").open(false))
                    .child(AccordionContent::new().open(false).child("Answer 2")),
            );

        assert_eq!(acc.children.len(), 2);
    }
}
