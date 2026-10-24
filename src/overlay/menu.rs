use std::{collections::HashMap, rc::Rc};

use gpui::prelude::*;
use gpui::{
    Anchor, AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, Pixels, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px, rems,
};

use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;

#[derive(Clone)]
pub enum PopupMenuItemKind {
    Item {
        label: SharedString,
        icon: Option<IconName>,
        checked: Option<bool>,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    },
    Separator,
    Custom(Rc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>),
}

#[derive(Clone)]
pub struct PopupMenuItem {
    id: Option<ElementId>,
    pub(crate) kind: PopupMenuItemKind,
}

impl PopupMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            kind: PopupMenuItemKind::Item {
                label: label.into(),
                icon: None,
                checked: None,
                on_click: None,
            },
        }
    }

    /// Supplies stable identity when a command's label can change.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        if let PopupMenuItemKind::Item { icon: ref mut i, .. } = self.kind {
            *i = Some(icon);
        }
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        if let PopupMenuItemKind::Item { checked: ref mut c, .. } = self.kind {
            *c = Some(checked);
        }
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        if let PopupMenuItemKind::Item { on_click: ref mut cb, .. } = self.kind {
            *cb = Some(Rc::new(callback));
        }
        self
    }

    pub fn separator() -> Self {
        Self { id: None, kind: PopupMenuItemKind::Separator }
    }

    pub fn element<F, E>(f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        Self {
            id: None,
            kind: PopupMenuItemKind::Custom(Rc::new(move |w, cx| f(w, cx).into_any_element())),
        }
    }
}

pub struct Menu {
    id: ElementId,
    items: Vec<PopupMenuItem>,
    min_w: Option<Pixels>,
    scrollable: bool,
}

struct MenuFocusState {
    container: FocusHandle,
    commands: HashMap<ElementId, FocusHandle>,
}

fn navigate_menu(handles: &[FocusHandle], event: &KeyDownEvent, window: &mut Window, cx: &mut App) {
    if handles.is_empty() || event.keystroke.modifiers.modified() {
        return;
    }
    let current = handles.iter().position(|handle| handle.is_focused(window));
    let next = match event.keystroke.key.as_str() {
        "down" => current.map_or(0, |ix| (ix + 1) % handles.len()),
        "up" => current.map_or(handles.len() - 1, |ix| (ix + handles.len() - 1) % handles.len()),
        "home" => 0,
        "end" => handles.len() - 1,
        _ => return,
    };
    handles[next].focus(window, cx);
    cx.stop_propagation();
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Menu {
    pub fn new() -> Self {
        Self {
            id: "menu-dropdown-content".into(),
            items: Vec::new(),
            min_w: None,
            scrollable: false,
        }
    }

    /// Namespaces retained menu focus when several menus share a view.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn min_w(mut self, min_w: Pixels) -> Self {
        self.min_w = Some(min_w);
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    pub fn item(mut self, item: PopupMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(PopupMenuItem::separator());
        self
    }

    pub fn render(
        self,
        close_fn: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let id = self.id.clone();
        let min_w = self.min_w;
        let scrollable = self.scrollable;

        let (focus_handle, items) = self.render_items_internal(Some(close_fn), window, cx);
        let theme = cx.theme();

        let mut el = div()
            .id(id)
            .role(Role::Menu)
            .track_focus(&focus_handle)
            .tab_group()
            .flex()
            .flex_col()
            .p_1()
            .gap_0p5()
            .min_w(rems(180. / 14.))
            .when_some(min_w, |this, width| this.min_w(width))
            .rounded(theme.radius)
            .border_1()
            .border_color(theme.border)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .when(theme.shadow, |surface| surface.shadow_md());

        if scrollable {
            el = el.max_h(rems(280. / 14.)).overflow_y_scroll();
        }

        el.children(items).into_any_element()
    }

    pub fn render_items(self, window: &mut Window, cx: &mut App) -> Vec<AnyElement> {
        self.render_items_internal(None, window, cx).1
    }

    fn render_items_internal(
        self,
        close_fn: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
        window: &mut Window,
        cx: &mut App,
    ) -> (FocusHandle, Vec<AnyElement>) {
        let row_ids: Vec<_> = self
            .items
            .iter()
            .filter_map(|item| match &item.kind {
                PopupMenuItemKind::Item { label, .. } => {
                    Some(item.id.clone().unwrap_or_else(|| label.clone().into()))
                }
                _ => None,
            })
            .collect();
        let initial_ids = row_ids.clone();
        let focus_state = window.use_keyed_state(self.id, cx, move |window, cx| {
            let container = cx.focus_handle().tab_stop(false);
            let commands: HashMap<_, _> =
                initial_ids.into_iter().map(|id| (id, cx.focus_handle().tab_stop(true))).collect();
            // Initial focus belongs to the opening lifecycle, never to redraws.
            let first =
                row_ids.first().and_then(|id| commands.get(id)).unwrap_or(&container).clone();
            window.defer(cx, move |window, cx| first.focus(window, cx));
            MenuFocusState { container, commands }
        });
        let (container, handles) = focus_state.update(cx, |state, cx| {
            let handles = self
                .items
                .iter()
                .filter_map(|item| match &item.kind {
                    PopupMenuItemKind::Item { label, .. } => {
                        let id = item.id.clone().unwrap_or_else(|| label.clone().into());
                        Some(
                            state
                                .commands
                                .entry(id)
                                .or_insert_with(|| cx.focus_handle().tab_stop(true))
                                .clone(),
                        )
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            (state.container.clone(), Rc::new(handles))
        });
        let has_checks = self
            .items
            .iter()
            .any(|item| matches!(item.kind, PopupMenuItemKind::Item { checked: Some(_), .. }));
        let has_icons = self
            .items
            .iter()
            .any(|item| matches!(item.kind, PopupMenuItemKind::Item { icon: Some(_), .. }));
        let (border, popover_foreground, accent, accent_foreground, muted_foreground, radius) = {
            let t = cx.theme();
            (
                t.border,
                t.popover_foreground,
                t.accent,
                t.accent_foreground,
                t.muted_foreground,
                t.radius_sm,
            )
        };
        let mut command_ix = 0;
        let items = self
            .items
            .into_iter()
            .map(|item| match item.kind {
                PopupMenuItemKind::Separator => {
                    div().h(px(1.)).my_1().bg(border).into_any_element()
                }
                PopupMenuItemKind::Custom(f) => f(window, cx),
                PopupMenuItemKind::Item { label, icon, checked, on_click } => {
                    let id = item.id.unwrap_or_else(|| label.clone().into());
                    let focus_handle = &handles[command_ix];
                    command_ix += 1;
                    let navigation = handles.clone();
                    let mut el = gpui_base::Button::new(id)
                        .role(if checked.is_some() {
                            Role::MenuItemCheckBox
                        } else {
                            Role::MenuItem
                        })
                        .accessibility_label(label.clone())
                        .track_focus(focus_handle)
                        .flex()
                        .justify_start()
                        .items_center()
                        .gap_2()
                        .min_h(rems(32. / 14.))
                        .px_2()
                        .py_1p5()
                        .rounded(radius)
                        .cursor_default()
                        .text_size(rems(1.))
                        .text_color(popover_foreground)
                        .hover(move |s| s.bg(accent).text_color(accent_foreground))
                        .focus(move |s| s.bg(accent).text_color(accent_foreground))
                        .on_key_down(move |event, window, cx| {
                            navigate_menu(&navigation, event, window, cx);
                        })
                        .when_some(checked, |this, checked| {
                            this.aria_toggled(if checked {
                                gpui::accesskit::Toggled::True
                            } else {
                                gpui::accesskit::Toggled::False
                            })
                        });

                    if has_checks {
                        el = el.child(
                            div()
                                .size(rems(16. / 14.))
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .when(checked == Some(true), |this| {
                                    this.child(Icon::new(IconName::Check).xsmall())
                                }),
                        );
                    }

                    if has_icons {
                        el = el.child(
                            div()
                                .size(rems(16. / 14.))
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .when_some(icon, |this, icon| {
                                    this.child(
                                        Icon::new(icon).xsmall().text_color(muted_foreground),
                                    )
                                }),
                        );
                    }

                    el = el.child(div().flex_1().min_w_0().child(label));

                    let close = close_fn.clone();
                    if let Some(cb) = on_click {
                        el = el.on_click(move |ev, window, cx| {
                            if let Some(close) = close.as_ref() {
                                close(window, cx);
                            }
                            cb(ev, window, cx);
                        });
                    } else if let Some(close) = close {
                        el = el.on_click(move |_ev, window, cx| {
                            close(window, cx);
                        });
                    }

                    el.into_any_element()
                }
            })
            .collect();
        (container, items)
    }
}

#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    anchor: Anchor,
    trigger: Option<AnyElement>,
    items: Vec<PopupMenuItem>,
    min_w: Option<Pixels>,
    scrollable: bool,
    builder: Option<Rc<dyn Fn(Menu, &mut Window, &mut App) -> Menu + 'static>>,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            anchor: Anchor::TopLeft,
            trigger: None,
            items: Vec::new(),
            min_w: None,
            scrollable: false,
            builder: None,
        }
    }

    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn min_w(mut self, min_w: Pixels) -> Self {
        self.min_w = Some(min_w);
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    pub fn item(mut self, item: PopupMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(PopupMenuItem::separator());
        self
    }

    pub fn menu<F>(mut self, builder: F) -> Self
    where
        F: Fn(Menu, &mut Window, &mut App) -> Menu + 'static,
    {
        self.builder = Some(Rc::new(builder));
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let menu_id = ElementId::Name(format!("{}:menu", self.id).into());
        let trigger = self.trigger.unwrap_or_else(|| div().into_any_element());
        let items = self.items;
        let min_w = self.min_w;
        let scrollable = self.scrollable;
        let builder = self.builder;

        gpui_base::Popover::new(self.id)
            .anchor(self.anchor)
            .offset(window.rem_size() * (4. / 14.))
            .trigger_with(move |_is_open, _window, _cx| trigger)
            .content(move |_state, window, cx| {
                let popover_entity = cx.entity().downgrade();
                let close_fn = Rc::new(move |window: &mut Window, cx: &mut App| {
                    if let Some(popover) = popover_entity.upgrade() {
                        popover.update(cx, |state, cx| {
                            state.dismiss(window, cx);
                        });
                    }
                });

                let mut menu = Menu::new().id(menu_id);
                if let Some(w) = min_w {
                    menu = menu.min_w(w);
                }
                if scrollable {
                    menu = menu.scrollable(true);
                }
                for item in items {
                    menu = menu.item(item);
                }
                if let Some(b) = builder {
                    menu = b(menu, window, cx);
                }
                menu.render(close_fn, window, cx)
            })
    }
}

pub trait ContextMenuExt: Sized {
    fn context_menu<F>(self, builder: F) -> Self
    where
        F: Fn(Menu, &mut Window, &mut App) -> Menu + 'static;
}

impl<T: gpui::InteractiveElement + Sized> ContextMenuExt for T {
    fn context_menu<F>(self, builder: F) -> Self
    where
        F: Fn(Menu, &mut Window, &mut App) -> Menu + 'static,
    {
        let builder = Rc::new(builder);
        self.on_mouse_down(gpui::MouseButton::Right, move |event, window, cx| {
            cx.stop_propagation();
            use crate::overlay::root::WindowExt;
            window.open_context_menu(cx, event.position, {
                let builder = builder.clone();
                move |m, w, cx| builder(m, w, cx)
            });
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        Context, Entity, KeyUpEvent, Keystroke, Modifiers, Render, TestAppContext,
        VisualTestContext,
    };

    struct MenuHarness {
        open: bool,
        checked: bool,
        activations: usize,
        closes: usize,
    }

    impl Render for MenuHarness {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let host = cx.entity().downgrade();
            let close = Rc::new(move |_: &mut Window, cx: &mut App| {
                let _ = host.update(cx, |this, cx| {
                    this.closes += 1;
                    cx.notify();
                });
            });
            let menu = Menu::new()
                .id("commands")
                .item(PopupMenuItem::new("First command").id("first").on_click(cx.listener(
                    |this, _, _, cx| {
                        this.activations += 1;
                        cx.notify();
                    },
                )))
                .separator()
                .item(PopupMenuItem::element(|_, _| div().child("Section")))
                .item(
                    PopupMenuItem::new("Show previews")
                        .id("checked")
                        .checked(self.checked)
                        .icon(IconName::Image)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.checked = !this.checked;
                            this.activations += 1;
                            cx.notify();
                        })),
                )
                .item(PopupMenuItem::new("Last command").id("last").on_click(cx.listener(
                    |this, _, _, cx| {
                        this.activations += 1;
                        cx.notify();
                    },
                )));

            div()
                .size_full()
                .tab_group()
                .flex()
                .flex_col()
                .items_start()
                .child(gpui_base::Button::new("before").h_8().child("Before"))
                .when(self.open, |this| this.child(menu.render(close, window, cx)))
                .child(gpui_base::Button::new("after").h_8().child("After"))
        }
    }

    fn harness(cx: &mut TestAppContext) -> (Entity<MenuHarness>, &mut VisualTestContext) {
        cx.update(crate::init);
        let (view, cx) = cx.add_window_view(|_, _| MenuHarness {
            open: true,
            checked: true,
            activations: 0,
            closes: 0,
        });
        cx.update(|window, cx| {
            window.set_rem_size(px(14.));
            window.draw(cx).clear(cx);
        });
        (view, cx)
    }

    fn snapshot(cx: &mut VisualTestContext, id: &str) -> gpui_base::test_support::ElementSnapshot {
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            gpui_base::test_support::find(window, &[], &ElementId::Name(id.to_owned().into()))
                .expect("rendered menu command")
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
    }

    #[gpui::test]
    fn menu_keyboard_navigation_stays_among_commands(cx: &mut TestAppContext) {
        let (view, cx) = harness(cx);
        assert_eq!(snapshot(cx, "first").focused(), Some(true));
        for (key, expected) in [
            ("down", "checked"),
            ("down", "last"),
            ("down", "first"),
            ("up", "last"),
            ("home", "first"),
            ("end", "last"),
        ] {
            press(cx, key);
            assert_eq!(snapshot(cx, expected).focused(), Some(true));
        }
        press(cx, "enter");
        press(cx, "space");
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).activations, 2);
            assert_eq!(view.read(cx).closes, 2);
        });
        assert_eq!(snapshot(cx, "last").focused(), Some(true));
    }

    #[gpui::test]
    fn checked_menu_command_exposes_state_and_uses_native_activation(cx: &mut TestAppContext) {
        let (view, cx) = harness(cx);
        let checked = snapshot(cx, "checked");
        assert_eq!(checked.role(), Some(Role::MenuItemCheckBox));
        assert_eq!(checked.label(), Some("Show previews"));
        assert_eq!(checked.checked(), Some(true));
        cx.simulate_click(checked.bounds().center(), Modifiers::default());
        assert_eq!(snapshot(cx, "checked").checked(), Some(false));
        press(cx, "space");
        assert_eq!(snapshot(cx, "checked").checked(), Some(true));
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).activations, 2);
            assert_eq!(view.read(cx).closes, 2);
        });
    }

    #[gpui::test]
    fn menu_command_geometry_scales_with_rem(cx: &mut TestAppContext) {
        let (_, cx) = harness(cx);
        assert_eq!(snapshot(cx, "first").bounds().size.height, px(32.));
        cx.update(|window, _| window.set_rem_size(px(21.)));
        assert_eq!(snapshot(cx, "first").bounds().size.height, px(48.));
    }

    #[gpui::test]
    fn reopened_menu_focuses_its_first_command(cx: &mut TestAppContext) {
        let (view, cx) = harness(cx);
        press(cx, "end");
        assert_eq!(snapshot(cx, "last").focused(), Some(true));
        cx.update(|_, cx| {
            view.update(cx, |this, cx| {
                this.open = false;
                cx.notify();
            });
        });
        let outside = snapshot(cx, "before");
        cx.simulate_click(outside.bounds().center(), Modifiers::default());
        assert_eq!(snapshot(cx, "before").focused(), Some(true));
        cx.update(|_, cx| {
            view.update(cx, |this, cx| {
                this.open = true;
                cx.notify();
            });
        });
        assert_eq!(snapshot(cx, "first").focused(), Some(true));
    }
}
