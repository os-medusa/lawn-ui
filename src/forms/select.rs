use gpui::prelude::*;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rems,
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
    accessibility_label: Option<SharedString>,
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
            accessibility_label: None,
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

    /// Names the field independently of the currently selected option.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
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
        let accessibility_label = self.accessibility_label.clone();
        let selected_value = self.selected_value.clone();
        let option_namespace = self.id.clone();

        let render_trigger = move |is_open: bool, window: &Window, cx: &App| {
            let theme = cx.theme();
            let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
                .spread_radius(window.rem_size() * (2. / 14.));

            let mut trigger = gpui_base::Button::new(select_id.clone())
                .role(gpui::Role::ComboBox)
                .accessibility_label(
                    accessibility_label
                        .clone()
                        .unwrap_or_else(|| selected_label.clone()),
                )
                .aria_value(selected_label.clone())
                .aria_expanded(is_open)
                .disabled(disabled)
                .w_full()
                .min_w_0()
                .h(rems(36. / 14.))
                .px(rems(12. / 14.))
                .rounded(theme.radius_3xl)
                .border_1()
                .border_color(if is_open {
                    theme.ring
                } else {
                    gpui::transparent_black()
                })
                .bg(theme.control_fill)
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .cursor_default()
                .text_size(gpui::rems(1.))
                .when(is_placeholder, |this| {
                    this.text_color(theme.muted_foreground)
                })
                .when(!is_placeholder, |this| this.text_color(theme.foreground))
                .when(!disabled, |this| {
                    this.hover(|s| s.bg(theme.control_hover))
                        .focus_visible(|s| s.border_color(theme.ring).shadow(vec![focus_ring]))
                })
                .when(disabled, |this| this.opacity(0.5));

            trigger.style().refine(&style);

            trigger
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .child(selected_label.clone()),
                )
                .child(
                    Icon::new(IconName::ChevronDown)
                        .size(IconSize::Md)
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
            .trigger_with(move |is_open, window, cx| {
                render_trigger(is_open, window, cx).into_any_element()
            })
            .content(move |_state, window, cx| {
                let popover_entity = cx.entity().downgrade();
                let close_fn = Rc::new(move |window: &mut Window, cx: &mut App| {
                    if let Some(popover) = popover_entity.upgrade() {
                        popover.update(cx, |state, cx| {
                            state.dismiss(window, cx);
                        });
                    }
                });

                let mut menu = Menu::new().id(menu_id.clone()).scrollable(true);
                for item in &items {
                    let val = item.value.clone();
                    let handler = on_change.clone();
                    let mut menu_item = PopupMenuItem::new(item.label.clone())
                        .id(format!("{option_namespace}:option:{}", item.value))
                        .checked(selected_value.as_ref() == Some(&item.value));
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

    use crate::{Button, Root};
    use gpui::{
        AppContext, Context, Entity, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Render,
        TestAppContext, VisualTestContext, size,
    };
    use gpui_base::test_support::{ElementSnapshot, find};

    struct SelectFixture {
        selected: SharedString,
        changes: Vec<SharedString>,
    }

    impl Render for SelectFixture {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let owner = cx.entity().downgrade();
            div()
                .size_full()
                .tab_group()
                .p_4()
                .flex()
                .flex_col()
                .gap_2()
                .child(Button::new("before-select").label("Before"))
                .child(
                    Select::new("color-scheme")
                        .accessibility_label("Color scheme")
                        .selected_value(Some(self.selected.clone()))
                        .item("red", "Red")
                        .item("blue", "Blue")
                        .item("green", "Green")
                        .on_change(move |value, _, cx| {
                            owner
                                .update(cx, |this, cx| {
                                    this.selected = value.clone();
                                    this.changes.push(value);
                                    cx.notify();
                                })
                                .unwrap();
                        }),
                )
        }
    }

    fn render_frame(cx: &mut VisualTestContext) {
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.simulate_next_frame(cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn snapshot(cx: &mut VisualTestContext, id: &str) -> ElementSnapshot {
        render_frame(cx);
        cx.update(|window, _| {
            find(window, &[], &ElementId::Name(id.to_owned().into()))
                .unwrap_or_else(|| panic!("rendered Select control {id:?}"))
        })
    }

    fn press(cx: &mut VisualTestContext, key: &str) {
        let keystroke = Keystroke::parse(key).unwrap();
        cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        cx.simulate_event(KeyUpEvent { keystroke });
        render_frame(cx);
    }

    #[gpui::test]
    fn keyboard_selection_and_escape_preserve_controlled_value_and_trigger_focus(
        cx: &mut TestAppContext,
    ) {
        cx.update(crate::init);
        let mut fixture = None;
        let (_, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|_| SelectFixture {
                selected: "blue".into(),
                changes: Vec::new(),
            });
            fixture = Some(view.clone());
            Root::new(view, window, cx)
        });
        let fixture: Entity<SelectFixture> = fixture.unwrap();
        cx.simulate_resize(size(px(640.), px(480.)));
        let before = snapshot(cx, "before-select");
        cx.simulate_click(before.bounds().center(), Modifiers::default());
        press(cx, "tab");
        let trigger = snapshot(cx, "color-scheme");
        assert_eq!(trigger.role(), Some(gpui::Role::ComboBox));
        assert_eq!(trigger.label(), Some("Color scheme"));
        assert_eq!(trigger.value(), Some("Blue"));
        assert_eq!(trigger.focused(), Some(true));
        assert_eq!(trigger.expanded(), Some(false));

        press(cx, "enter");
        assert_eq!(snapshot(cx, "color-scheme").expanded(), Some(true));
        assert_eq!(
            snapshot(cx, "color-scheme:option:blue").checked(),
            Some(true)
        );
        assert_eq!(
            snapshot(cx, "color-scheme:option:red").checked(),
            Some(false)
        );
        assert_eq!(
            snapshot(cx, "color-scheme:option:red").focused(),
            Some(true)
        );
        press(cx, "down");
        assert_eq!(
            snapshot(cx, "color-scheme:option:blue").focused(),
            Some(true)
        );
        press(cx, "down");
        assert_eq!(
            snapshot(cx, "color-scheme:option:green").focused(),
            Some(true)
        );
        press(cx, "enter");
        let trigger = snapshot(cx, "color-scheme");
        assert_eq!(trigger.value(), Some("Green"));
        assert_eq!(trigger.expanded(), Some(false));
        assert_eq!(trigger.focused(), Some(true));
        cx.update(|_, cx| {
            let fixture = fixture.read(cx);
            assert_eq!(fixture.selected.as_ref(), "green");
            assert_eq!(fixture.changes.as_slice(), &[SharedString::from("green")]);
        });

        press(cx, "space");
        assert_eq!(snapshot(cx, "color-scheme").expanded(), Some(true));
        assert_eq!(
            snapshot(cx, "color-scheme:option:green").checked(),
            Some(true)
        );
        press(cx, "down");
        press(cx, "escape");
        let trigger = snapshot(cx, "color-scheme");
        assert_eq!(trigger.expanded(), Some(false));
        assert_eq!(trigger.focused(), Some(true));
        assert_eq!(trigger.value(), Some("Green"));
        cx.update(|_, cx| {
            let fixture = fixture.read(cx);
            assert_eq!(fixture.selected.as_ref(), "green");
            assert_eq!(
                fixture.changes.len(),
                1,
                "Escape must not commit the focused option"
            );
        });
    }
}
