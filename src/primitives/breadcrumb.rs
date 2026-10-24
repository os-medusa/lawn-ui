use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// Navigation container for breadcrumb trails.
#[derive(IntoElement, Default)]
pub struct Breadcrumb {
    children: Vec<AnyElement>,
}

impl Breadcrumb {
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

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().children(self.children)
    }
}

/// The list wrapper of breadcrumb items.
#[derive(IntoElement, Default)]
pub struct BreadcrumbList {
    children: Vec<AnyElement>,
}

impl BreadcrumbList {
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

impl RenderOnce for BreadcrumbList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(rems(6. / 14.))
            .text_size(gpui::rems(1.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// An individual item in a breadcrumb trail.
#[derive(IntoElement, Default)]
pub struct BreadcrumbItem {
    children: Vec<AnyElement>,
}

impl BreadcrumbItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(rems(6. / 14.))
            .children(self.children)
    }
}

/// A clickable link within a [`BreadcrumbItem`].
#[derive(IntoElement)]
pub struct BreadcrumbLink {
    id: Option<ElementId>,
    label: SharedString,
    on_click: Option<Rc<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl BreadcrumbLink {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            label: label.into(),
            on_click: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for BreadcrumbLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let id = self.id.unwrap_or_else(|| ElementId::Name("breadcrumb-link".into()));

        let mut el = div()
            .id(id)
            .text_size(gpui::rems(1.))
            .text_color(theme.muted_foreground)
            .hover(|s| s.text_color(theme.foreground).underline())
            .cursor_pointer();

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |ev, window, cx| (on_click)(ev, window, cx));
        }

        el.child(self.label)
    }
}

/// The current page in a breadcrumb trail.
#[derive(IntoElement)]
pub struct BreadcrumbPage {
    label: SharedString,
}

impl BreadcrumbPage {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl RenderOnce for BreadcrumbPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(gpui::rems(1.))
            .font_normal()
            .text_color(theme.foreground)
            .child(self.label)
    }
}

/// A separator between breadcrumb items, displaying a chevron by default.
#[derive(IntoElement, Default)]
pub struct BreadcrumbSeparator;

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div().flex().items_center().child(
            Icon::new(IconName::ChevronRight)
                .size(IconSize::Xs)
                .text_color(theme.muted_foreground),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_trail_construction() {
        let trail = Breadcrumb::new().child(
            BreadcrumbList::new()
                .child(BreadcrumbItem::new().child(BreadcrumbLink::new("Home")))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::new().child(BreadcrumbLink::new("Settings")))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::new().child(BreadcrumbPage::new("Profile"))),
        );

        assert_eq!(trail.children.len(), 1);
    }
}
