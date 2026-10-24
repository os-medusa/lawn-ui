use gpui::prelude::FluentBuilder;
use gpui::{
    Refineable, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// An option item inside a [`Combobox`].
#[derive(Clone, Debug)]
pub struct ComboboxItem {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

impl ComboboxItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// An autocomplete search & select input control (Combobox).
#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    placeholder: SharedString,
    search_placeholder: SharedString,
    search_query: Option<SharedString>,
    selected_value: Option<SharedString>,
    items: Vec<ComboboxItem>,
    disabled: bool,
    style: StyleRefinement,
    on_select: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
}

impl Combobox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: "Select option...".into(),
            search_placeholder: "Search...".into(),
            search_query: None,
            selected_value: None,
            items: Vec::new(),
            disabled: false,
            style: StyleRefinement::default(),
            on_select: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn search_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = Some(query.into());
        self
    }

    pub fn selected(mut self, value: Option<impl Into<SharedString>>) -> Self {
        self.selected_value = value.map(Into::into);
        self
    }

    pub fn items(mut self, items: Vec<ComboboxItem>) -> Self {
        self.items = items;
        self
    }

    pub fn item(mut self, item: ComboboxItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(
        mut self,
        callback: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(callback));
        self
    }
}

impl Styled for Combobox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Combobox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let display_text = self
            .selected_value
            .as_ref()
            .and_then(|val| {
                self.items
                    .iter()
                    .find(|item| &item.value == val)
                    .map(|item| item.label.clone())
            })
            .unwrap_or_else(|| self.placeholder.clone());

        let is_placeholder = self.selected_value.is_none();

        let trigger = div()
            .id("combobox-trigger")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(rems(36. / 14.))
            .px_3()
            .py_2()
            .bg(theme.control_fill)
            .border_1()
            .border_color(gpui::transparent_black())
            .rounded(theme.radius_3xl)
            .text_size(gpui::rems(1.))
            .when(is_placeholder, |this| {
                this.text_color(theme.muted_foreground)
            })
            .when(!is_placeholder, |this| {
                this.text_color(theme.foreground)
            })
            .when(self.disabled, |this| {
                this.opacity(0.5).cursor_not_allowed()
            })
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|s| s.bg(theme.control_hover))
            })
            .child(display_text)
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_color(theme.muted_foreground)
                    .child(
                        Icon::new(IconName::ChevronsUpDown)
                            .size(IconSize::Custom(px(16.))),
                    ),
            );

        div()
            .id(self.id)
            .w_full()
            .relative()
            .child(trigger)
            .map(|mut el| { el.style().refine(&self.style); el })
    }
}
