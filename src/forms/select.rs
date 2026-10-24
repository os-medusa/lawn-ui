use gpui::prelude::*;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::overlay::menu::{Menu, PopupMenuItem};
use crate::theme::ActiveTheme;

/// An option item inside a [`Select`] control.
#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

impl SelectItem {
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

/// A dropdown select input control adhering to shadcn/ui.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    placeholder: SharedString,
    selected_value: Option<SharedString>,
    items: Vec<SelectItem>,
    disabled: bool,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: "Выберите значение...".into(),
            selected_value: None,
            items: Vec::new(),
            disabled: false,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn selected_value(mut self, val: Option<impl Into<SharedString>>) -> Self {
        self.selected_value = val.map(Into::into);
        self
    }

    pub fn item(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.items.push(SelectItem::new(value, label));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let select_id = self.id.clone();
        let disabled = self.disabled;

        let selected_label = self
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
        let on_change = self.on_change.clone();
        let items = self.items.clone();
        let style = self.style.clone();

        let render_trigger = move |is_open: bool, window: &Window, cx: &App| {
            let theme = cx.theme();
            let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
                .spread_radius(window.rem_size() * (2. / 14.));

            let mut trigger = div()
                .id(select_id.clone())
                .w_full()
                .h(rems(36. / 14.))
                .px(rems(12. / 14.))
                .rounded(theme.radius)
                .border_1()
                .border_color(if is_open { theme.ring } else { theme.border })
                .bg(theme.background)
                .flex()
                .items_center()
                .justify_between()
                .text_sm()
                .when(is_placeholder, |this| this.text_color(theme.muted_foreground))
                .when(!is_placeholder, |this| this.text_color(theme.foreground))
                .when(!disabled, |this| {
                    this.cursor_pointer()
                        .hover(|s| s.bg(theme.control_hover))
                        .focus_visible(|s| s.border_color(theme.ring).shadow(vec![focus_ring]))
                })
                .when(disabled, |this| this.opacity(0.5).cursor_not_allowed());

            trigger.style().refine(&style);

            trigger
                .child(div().truncate().child(selected_label.clone()))
                .child(
                    Icon::new(IconName::ChevronDown)
                        .size(IconSize::Xs)
                        .text_color(theme.muted_foreground),
                )
        };

        if disabled {
            return render_trigger(false, window, _cx).into_any_element();
        }

        let popover_id = ElementId::Name(format!("{}:popover", self.id).into());
        let menu_id = ElementId::Name(format!("{}:menu", self.id).into());

        gpui_base::Popover::new(popover_id)
            .anchor(gpui::Anchor::TopLeft)
            .offset(window.rem_size() * (4. / 14.))
            .trigger_with(move |is_open, window, cx| render_trigger(is_open, window, cx).into_any_element())
            .content(move |_state, window, cx| {
                let popover_entity = cx.entity().downgrade();
                let close_fn = Rc::new(move |window: &mut Window, cx: &mut App| {
                    if let Some(popover) = popover_entity.upgrade() {
                        popover.update(cx, |state, cx| {
                            state.dismiss(window, cx);
                        });
                    }
                });

                let mut menu = Menu::new().id(menu_id.clone());
                for item in &items {
                    let val = item.value.clone();
                    let handler = on_change.clone();
                    let mut menu_item = PopupMenuItem::new(item.label.clone());
                    if let Some(h) = handler {
                        menu_item = menu_item.on_click(move |_, window, cx| {
                            (h)(val.clone(), window, cx);
                        });
                    }
                    menu = menu.item(menu_item);
                }
                menu.render(close_fn, window, cx)
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_construction() {
        let select = Select::new("test-select")
            .placeholder("Select role...")
            .item("admin", "Administrator")
            .item("user", "Regular User")
            .selected_value(Some("admin"));

        assert_eq!(select.items.len(), 2);
        assert_eq!(select.selected_value.as_deref(), Some("admin"));
    }
}
