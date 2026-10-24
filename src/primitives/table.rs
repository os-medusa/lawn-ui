use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, rems,
};
use std::rc::Rc;

use crate::theme::ActiveTheme;

/// A responsive table container adhering to modern shadcn/ui standards.
#[derive(IntoElement, Default)]
pub struct Table {
    id: Option<ElementId>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
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

impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let content = div()
            .w_full()
            .text_size(gpui::rems(12. / 14.))
            .text_color(theme.foreground)
            .flex()
            .flex_col()
            .children(self.children);

        let mut container = div().w_full().overflow_hidden().child(content);
        container.style().refine(&self.style);

        if let Some(id) = self.id {
            container.id(id).into_any_element()
        } else {
            container.into_any_element()
        }
    }
}

/// The header section of a [`Table`].
#[derive(IntoElement, Default)]
pub struct TableHeader {
    children: Vec<AnyElement>,
}

impl TableHeader {
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

impl RenderOnce for TableHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .w_full()
            .border_b_1()
            .border_color(theme.border)
            .bg(theme.item_fill)
            .flex()
            .flex_col()
            .children(self.children)
    }
}

/// The body container holding all data rows of a [`Table`].
#[derive(IntoElement, Default)]
pub struct TableBody {
    children: Vec<AnyElement>,
}

impl TableBody {
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

impl RenderOnce for TableBody {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().w_full().flex().flex_col().children(self.children)
    }
}

/// A row within a table header or body.
#[derive(IntoElement, Default)]
pub struct TableRow {
    id: Option<ElementId>,
    selected: bool,
    clickable: bool,
    on_click: Option<Rc<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl TableRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.clickable = true;
        self.on_click = Some(Rc::new(handler));
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

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let bg_color = if self.selected {
            theme.accent
        } else {
            gpui::hsla(0., 0., 0., 0.)
        };

        let id = self.id.unwrap_or_else(|| ElementId::Integer(0));

        let mut el = div()
            .id(id)
            .flex()
            .w_full()
            .items_center()
            .min_h(rems(48. / 14.))
            .border_b_1()
            .border_color(theme.border)
            .bg(bg_color)
            .hover(|style| style.bg(theme.control_hover));

        if let Some(on_click) = self.on_click {
            el = el
                .cursor_pointer()
                .on_click(move |ev, window, cx| (on_click)(ev, window, cx));
        }

        el.children(self.children)
    }
}

/// A header cell within a [`TableHeader`] row.
#[derive(IntoElement, Default)]
pub struct TableHead {
    width: Option<gpui::DefiniteLength>,
    align_right: bool,
    children: Vec<AnyElement>,
}

impl TableHead {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<gpui::DefiniteLength>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align_right = true;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for TableHead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut el = div()
            .flex()
            .items_center()
            .px(rems(16. / 14.))
            .py(rems(10. / 14.))
            .text_size(gpui::rems(12. / 14.))
            .font_weight(gpui::FontWeight::NORMAL)
            .text_color(theme.muted_foreground)
            .when(self.align_right, |this| this.justify_end());

        if let Some(w) = self.width {
            el = el.w(w).flex_shrink_0();
        } else {
            el = el.flex_1();
        }

        el.children(self.children)
    }
}

/// A standard data cell within a [`TableRow`].
#[derive(IntoElement, Default)]
pub struct TableCell {
    width: Option<gpui::DefiniteLength>,
    align_right: bool,
    children: Vec<AnyElement>,
}

impl TableCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<gpui::DefiniteLength>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align_right = true;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut el = div()
            .flex()
            .items_center()
            .px(rems(16. / 14.))
            .py(rems(10. / 14.))
            .text_size(gpui::rems(12. / 14.))
            .text_color(theme.foreground)
            .when(self.align_right, |this| this.justify_end());

        if let Some(w) = self.width {
            el = el.w(w).flex_shrink_0();
        } else {
            el = el.flex_1();
        }

        el.children(self.children)
    }
}

/// Table caption shown below or above the table for accessibility and context.
#[derive(IntoElement)]
pub struct TableCaption {
    text: SharedString,
}

impl TableCaption {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for TableCaption {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .mt(rems(12. / 14.))
            .text_size(gpui::rems(12. / 14.))
            .text_color(theme.muted_foreground)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_construction() {
        let table = Table::new()
            .child(
                TableHeader::new().child(
                    TableRow::new()
                        .child(TableHead::new().child("Invoice"))
                        .child(TableHead::new().child("Status")),
                ),
            )
            .child(
                TableBody::new().child(
                    TableRow::new()
                        .child(TableCell::new().child("INV-001"))
                        .child(TableCell::new().child("Paid")),
                ),
            )
            .child(TableCaption::new("Recent invoices list"));

        assert_eq!(table.children.len(), 3);
    }
}
