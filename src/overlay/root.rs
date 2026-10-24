use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, ClickEvent, Context, Div, ElementId, Entity, FocusHandle, IntoElement,
    ParentElement, Pixels, Point, Render, SharedString, Stateful, Styled, WeakFocusHandle, Window,
    div, px,
};
use gpui_base::{Dialog as DialogHost, RootPlugin, Sheet as SheetHost};

pub use gpui_base::Root;

use crate::overlay::alert_dialog::AlertDialog;
use crate::overlay::dialog::Dialog;
use crate::overlay::sheet::Sheet;
use crate::primitives::button::{Button, ButtonVariant, ButtonVariants};
use crate::theme::ActiveTheme;

type DialogDecision = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NotificationKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Notification {
    id: Option<ElementId>,
    title: Option<SharedString>,
    message: SharedString,
    kind: NotificationKind,
}

impl Notification {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self { id: None, title: None, message: message.into(), kind: NotificationKind::Info }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        if self.title.is_none() {
            self.title = Some(self.message);
        }
        self.message = message.into();
        self
    }

    pub fn info(mut self) -> Self {
        self.kind = NotificationKind::Info;
        self
    }

    pub fn success(mut self) -> Self {
        self.kind = NotificationKind::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.kind = NotificationKind::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.kind = NotificationKind::Error;
        self
    }
}

impl<T: Into<SharedString>> From<T> for Notification {
    fn from(s: T) -> Self {
        Notification::new(s)
    }
}

struct ActiveDialogItem {
    focus_handle: FocusHandle,
    previous_focused: Option<WeakFocusHandle>,
    builder: Rc<dyn Fn(Dialog, &mut Window, &mut App) -> Dialog + 'static>,
    on_ok: DialogDecision,
    on_cancel: DialogDecision,
}

struct ActiveSheetItem {
    focus_handle: FocusHandle,
    previous_focused: Option<WeakFocusHandle>,
    builder: Rc<dyn Fn(Sheet, &mut Window, &mut App) -> Sheet + 'static>,
}

struct ActiveContextMenuItem {
    position: Point<Pixels>,
    focus_handle: FocusHandle,
    previous_focused: Option<WeakFocusHandle>,
    builder: Rc<
        dyn Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
            + 'static,
    >,
}

pub(crate) struct GlyphRootLayer {
    active_dialogs: Vec<ActiveDialogItem>,
    active_sheet: Option<ActiveSheetItem>,
    active_context_menu: Option<ActiveContextMenuItem>,
    notifications: Vec<(Notification, std::time::Instant)>,
    next_notification_id: u64,
}

impl GlyphRootLayer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            active_dialogs: Vec::new(),
            active_sheet: None,
            active_context_menu: None,
            notifications: Vec::new(),
            next_notification_id: 0,
        }
    }

    fn render_dialog_layer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let item = self.active_dialogs.last()?;
        let configured = (item.builder)(Dialog::new(), window, cx);
        let on_ok = item.on_ok.clone();
        let on_cancel = item.on_cancel.clone();
        let theme = cx.theme().clone();

        Some(
            DialogHost::new(cx)
                .focus_handle(item.focus_handle.clone())
                .backdrop(div().absolute().inset_0().bg(theme.background.opacity(0.56)).occlude())
                .popup(configured)
                .on_ok(move |event, window, cx| on_ok(event, window, cx))
                .on_cancel(move |event, window, cx| on_cancel(event, window, cx))
                .request_close(|_, window, cx| window.close_dialog(cx))
                .into_any_element(),
        )
    }

    fn render_sheet_layer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let item = self.active_sheet.as_ref()?;
        let configured = (item.builder)(Sheet::new(), window, cx);
        let side = configured.side_value();
        let theme = cx.theme().clone();
        let surface = match side {
            crate::overlay::sheet::SheetSide::Right => {
                div().size_full().flex().justify_end().child(configured)
            }
            crate::overlay::sheet::SheetSide::Left => {
                div().size_full().flex().justify_start().child(configured)
            }
            crate::overlay::sheet::SheetSide::Top => {
                div().size_full().flex().flex_col().justify_start().child(configured)
            }
            crate::overlay::sheet::SheetSide::Bottom => {
                div().size_full().flex().flex_col().justify_end().child(configured)
            }
        };

        Some(
            SheetHost::new(cx)
                .focus_handle(item.focus_handle.clone())
                .overlay(div().absolute().inset_0().bg(theme.background.opacity(0.48)).occlude())
                .surface(surface)
                .request_close(|window, cx| window.close_sheet(cx))
                .into_any_element(),
        )
    }

    fn render_notification_layer(&self, cx: &App) -> Option<AnyElement> {
        let now = std::time::Instant::now();
        let items: Vec<Notification> = self
            .notifications
            .iter()
            .filter(|(_, time)| now.duration_since(*time) < Duration::from_secs(4))
            .map(|(n, _)| n.clone())
            .collect();

        if items.is_empty() {
            return None;
        }

        let theme = cx.theme();
        Some(
            div()
                .absolute()
                .bottom_6()
                .right_6()
                .flex()
                .flex_col()
                .gap_2()
                .children(items.into_iter().map(move |n| {
                    let (icon, icon_color) = match n.kind {
                        NotificationKind::Success => {
                            (crate::icon::IconName::Check, theme.chart_bullish)
                        }
                        NotificationKind::Warning => (crate::icon::IconName::Info, theme.warning),
                        NotificationKind::Error => {
                            (crate::icon::IconName::Close, theme.destructive)
                        }
                        NotificationKind::Info => (crate::icon::IconName::Info, theme.primary),
                    };

                    div()
                        .flex()
                        .items_start()
                        .gap_3()
                        .px_4()
                        .py_3()
                        .max_w(px(360.))
                        .rounded(theme.radius)
                        .bg(theme.popover)
                        .text_color(theme.popover_foreground)
                        .border_1()
                        .border_color(theme.border)
                        .when(theme.shadow, |surface| surface.shadow_lg())
                        .child(
                            crate::icon::Icon::new(icon)
                                .size(crate::icon::IconSize::Sm)
                                .color(icon_color),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .flex_1()
                                .when_some(n.title, |this, title| {
                                    this.child(
                                        div()
                                            .text_size(gpui::rems(1.))
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(theme.foreground)
                                            .child(title),
                                    )
                                })
                                .child(
                                    div()
                                        .text_size(gpui::rems(12. / 14.))
                                        .text_color(theme.muted_foreground)
                                        .child(n.message),
                                ),
                        )
                }))
                .into_any_element(),
        )
    }
    fn render_context_menu_layer(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let active = self.active_context_menu.as_ref()?;
        let builder = active.builder.clone();
        let mut position = active.position;

        let viewport_size = window.viewport_size();
        let estimated_width = window.rem_size() * (220. / 14.);
        let estimated_height = window.rem_size() * (260. / 14.);

        if position.x + estimated_width > viewport_size.width {
            position.x = (viewport_size.width - estimated_width - px(8.)).max(px(8.));
        }
        if position.y + estimated_height > viewport_size.height {
            position.y = (viewport_size.height - estimated_height - px(8.)).max(px(8.));
        }

        let menu = builder(crate::overlay::menu::Menu::new(), window, cx);

        let close_fn = Rc::new(move |window: &mut Window, cx: &mut App| {
            window.close_context_menu(cx);
        });

        let close_backdrop = close_fn.clone();
        let close_backdrop_right = close_fn.clone();
        let close_keyboard = close_fn.clone();

        Some(
            div()
                .id("context-menu-layer")
                .track_focus(&active.focus_handle)
                .absolute()
                .inset_0()
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.key == "escape" {
                        close_keyboard(window, cx);
                        cx.stop_propagation();
                    }
                })
                .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                    close_backdrop(window, cx);
                })
                .on_mouse_down(gpui::MouseButton::Right, move |_, window, cx| {
                    close_backdrop_right(window, cx);
                })
                .child(
                    div()
                        .absolute()
                        .top(position.y)
                        .left(position.x)
                        .occlude()
                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_mouse_down(gpui::MouseButton::Right, |_, _, cx| cx.stop_propagation())
                        .child(menu.render(close_fn, window, cx)),
                )
                .into_any_element(),
        )
    }
}

impl RootPlugin for GlyphRootLayer {
    fn prepare(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.set_rem_size(cx.theme().font_size);
    }

    fn style(&self, surface: &mut Stateful<Div>, _window: &mut Window, cx: &mut App) {
        let theme = cx.theme();
        let mut defaults = div().bg(theme.background).text_color(theme.foreground).text_size(gpui::rems(16. / 14.));
        surface.style().refine(defaults.style());
    }
}

impl Render for GlyphRootLayer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet = self.render_sheet_layer(window, cx);
        let dialogs = self.render_dialog_layer(window, cx);
        let notifications = self.render_notification_layer(cx);
        let context_menu = self.render_context_menu_layer(window, cx);

        div()
            .size_full()
            .children(sheet)
            .children(dialogs)
            .children(notifications)
            .children(context_menu)
    }
}

pub(crate) fn init(cx: &mut App) {
    Root::register_plugin(cx, GlyphRootLayer::new);
}

fn glyph_root_layer(window: &Window, cx: &App) -> Option<Entity<GlyphRootLayer>> {
    window.root::<Root>().flatten()?.read(cx).plugin::<GlyphRootLayer>()
}

pub trait WindowExt: Sized {
    fn push_notification(&mut self, note: impl Into<Notification>, cx: &mut App);
    fn open_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Dialog, &mut Window, &mut App) -> Dialog + 'static;
    fn close_dialog(&mut self, cx: &mut App);
    fn close_all_dialogs(&mut self, cx: &mut App);
    fn has_active_dialog(&mut self, cx: &mut App) -> bool;

    fn open_alert_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(AlertDialog, &mut Window, &mut App) -> AlertDialog + 'static;

    fn open_sheet<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Sheet, &mut Window, &mut App) -> Sheet + 'static;
    fn close_sheet(&mut self, cx: &mut App);
    fn has_active_sheet(&mut self, cx: &mut App) -> bool;

    fn open_context_menu<F>(&mut self, cx: &mut App, position: Point<Pixels>, build: F)
    where
        F: Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
            + 'static;
    fn close_context_menu(&mut self, cx: &mut App);
    fn has_active_context_menu(&mut self, cx: &mut App) -> bool;

    fn has_focused_input(&mut self, cx: &mut App) -> bool;
    fn remove_notification(&mut self, id: impl Into<ElementId>, cx: &mut App);
    fn clear_notifications(&mut self, cx: &mut App);
}

fn push_dialog<F>(
    window: &mut Window,
    cx: &mut App,
    build: F,
    on_ok: DialogDecision,
    on_cancel: DialogDecision,
) where
    F: Fn(Dialog, &mut Window, &mut App) -> Dialog + 'static,
{
    if let Some(root) = glyph_root_layer(window, cx) {
        let previous = window.focused(cx).map(|handle| handle.downgrade());
        root.update(cx, |this, cx| {
            let focus_handle = cx.focus_handle();
            this.active_dialogs.push(ActiveDialogItem {
                focus_handle: focus_handle.clone(),
                previous_focused: previous,
                builder: Rc::new(build),
                on_ok,
                on_cancel,
            });
            cx.notify();
            cx.on_next_frame(window, move |layer, window, cx| {
                if layer.active_context_menu.is_none()
                    && layer
                        .active_dialogs
                        .last()
                        .is_some_and(|item| item.focus_handle == focus_handle)
                {
                    focus_handle.focus(window, cx);
                }
            });
        });
    }
}

fn alert_decision(
    callback: Option<crate::overlay::alert_dialog::AlertDialogHandler>,
    alert: Rc<RefCell<AlertDialog>>,
) -> DialogDecision {
    let Some(callback) = callback else {
        return Rc::new(|_, _, _| true);
    };

    Rc::new(move |_, window, cx| {
        let Some(root) = window.root::<Root>().flatten() else {
            return true;
        };
        root.update(cx, |_, root_cx| {
            let mut alert = alert.borrow_mut();
            callback(&mut alert, window, root_cx)
        })
    })
}

impl WindowExt for Window {
    fn push_notification(&mut self, note: impl Into<Notification>, cx: &mut App) {
        let mut note = note.into();
        if let Some(root) = glyph_root_layer(self, cx) {
            root.update(cx, |this, cx| {
                if note.id.is_none() {
                    note.id =
                        Some(format!("glyph-notification-{}", this.next_notification_id).into());
                    this.next_notification_id = this.next_notification_id.wrapping_add(1);
                }
                let id = note.id.clone().expect("notification id was assigned");
                let created_at = std::time::Instant::now();
                this.notifications.retain(|(existing, _)| existing.id.as_ref() != Some(&id));
                this.notifications.push((note, created_at));
                cx.notify();

                cx.spawn(async move |root, cx| {
                    cx.background_executor().timer(Duration::from_secs(4)).await;
                    if let Some(root) = root.upgrade() {
                        root.update(cx, |this, cx| {
                            let count = this.notifications.len();
                            this.notifications.retain(|(notification, timestamp)| {
                                notification.id.as_ref() != Some(&id) || *timestamp != created_at
                            });
                            if this.notifications.len() != count {
                                cx.notify();
                            }
                        });
                    }
                })
                .detach();
            });
        }
    }

    fn open_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Dialog, &mut Window, &mut App) -> Dialog + 'static,
    {
        push_dialog(self, cx, build, Rc::new(|_, _, _| true), Rc::new(|_, _, _| true));
    }

    fn close_dialog(&mut self, cx: &mut App) {
        if let Some(root) = glyph_root_layer(self, cx) {
            let focus_to_restore = root.update(cx, |this, cx| {
                let previous = this
                    .active_dialogs
                    .pop()
                    .and_then(|item| item.previous_focused.and_then(|handle| handle.upgrade()));
                let focus = previous
                    .or_else(|| this.active_dialogs.last().map(|item| item.focus_handle.clone()));
                cx.notify();
                focus
            });
            if let Some(focus) = focus_to_restore {
                focus.focus(self, cx);
            }
        }
    }

    fn close_all_dialogs(&mut self, cx: &mut App) {
        if let Some(root) = glyph_root_layer(self, cx) {
            let focus_to_restore = root.update(cx, |this, cx| {
                let previous = this
                    .active_dialogs
                    .first()
                    .and_then(|item| item.previous_focused.clone())
                    .and_then(|handle| handle.upgrade());
                this.active_dialogs.clear();
                cx.notify();
                previous
            });
            if let Some(focus) = focus_to_restore {
                focus.focus(self, cx);
            }
        }
    }

    fn has_active_dialog(&mut self, cx: &mut App) -> bool {
        glyph_root_layer(self, cx).map(|r| !r.read(cx).active_dialogs.is_empty()).unwrap_or(false)
    }

    fn open_alert_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(AlertDialog, &mut Window, &mut App) -> AlertDialog + 'static,
    {
        let alert = build(AlertDialog::new(), self, cx);
        let title = alert.title.clone().unwrap_or_else(|| "Confirm".into());
        let desc = alert.description.clone();
        let ok_text = alert.button_props.ok_text.clone().unwrap_or_else(|| "OK".into());
        let cancel_text = alert.button_props.cancel_text.clone().unwrap_or_else(|| "Cancel".into());
        let show_cancel = alert.button_props.show_cancel;
        let ok_variant = alert.button_props.ok_variant.unwrap_or(ButtonVariant::Default);
        let alert = Rc::new(RefCell::new(alert));
        let on_ok_callback = alert.borrow().on_ok.clone();
        let on_cancel_callback = alert.borrow().on_cancel.clone();
        let on_ok = alert_decision(on_ok_callback, alert.clone());
        let on_cancel = alert_decision(on_cancel_callback, alert);

        let button_on_ok = on_ok.clone();
        let button_on_cancel = on_cancel.clone();
        push_dialog(
            self,
            cx,
            move |dialog, _, _| {
                let d_title = title.clone();
                let d_desc = desc.clone();
                let d_ok_text = ok_text.clone();
                let d_cancel_text = cancel_text.clone();
                let button_on_ok = button_on_ok.clone();
                let button_on_cancel = button_on_cancel.clone();

                dialog.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(
                            div().text_size(gpui::rems(18. / 14.)).font_weight(gpui::FontWeight::SEMIBOLD).child(d_title),
                        )
                        .when_some(d_desc, |this, d| this.child(div().text_size(gpui::rems(1.)).child(d)))
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .gap_2()
                                .when(show_cancel, |this| {
                                    this.child(
                                        Button::new("alert-cancel")
                                            .with_variant(ButtonVariant::Outline)
                                            .label(d_cancel_text)
                                            .on_click(move |event, window, cx| {
                                                if button_on_cancel(event, window, cx) {
                                                    window.close_dialog(cx);
                                                }
                                            }),
                                    )
                                })
                                .child(
                                    Button::new("alert-ok")
                                        .with_variant(ok_variant)
                                        .label(d_ok_text)
                                        .on_click(move |event, window, cx| {
                                            if button_on_ok(event, window, cx) {
                                                window.close_dialog(cx);
                                            }
                                        }),
                                ),
                        ),
                )
            },
            on_ok,
            on_cancel,
        );
    }

    fn open_sheet<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Sheet, &mut Window, &mut App) -> Sheet + 'static,
    {
        if let Some(root) = glyph_root_layer(self, cx) {
            let previous = self.focused(cx).map(|h| h.downgrade());
            root.update(cx, |this, cx| {
                let focus_handle = cx.focus_handle();
                this.active_sheet = Some(ActiveSheetItem {
                    focus_handle: focus_handle.clone(),
                    previous_focused: previous,
                    builder: Rc::new(build),
                });
                cx.notify();
                cx.on_next_frame(self, move |layer, window, cx| {
                    if layer.active_dialogs.is_empty()
                        && layer.active_context_menu.is_none()
                        && layer
                            .active_sheet
                            .as_ref()
                            .is_some_and(|item| item.focus_handle == focus_handle)
                    {
                        focus_handle.focus(window, cx);
                    }
                });
            });
        }
    }

    fn close_sheet(&mut self, cx: &mut App) {
        if let Some(root) = glyph_root_layer(self, cx) {
            let focus_to_restore = root.update(cx, |this, cx| {
                let previous = this
                    .active_sheet
                    .take()
                    .and_then(|item| item.previous_focused.and_then(|handle| handle.upgrade()));
                cx.notify();
                previous
            });
            if let Some(focus) = focus_to_restore {
                focus.focus(self, cx);
            }
        }
    }

    fn has_active_sheet(&mut self, cx: &mut App) -> bool {
        glyph_root_layer(self, cx)
            .map(|layer| layer.read(cx).active_sheet.is_some())
            .unwrap_or(false)
    }

    fn open_context_menu<F>(&mut self, cx: &mut App, position: Point<Pixels>, build: F)
    where
        F: Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
            + 'static,
    {
        if let Some(root) = glyph_root_layer(self, cx) {
            let previous = root
                .read(cx)
                .active_context_menu
                .as_ref()
                .map(|menu| menu.previous_focused.clone())
                .unwrap_or_else(|| self.focused(cx).map(|focus| focus.downgrade()));
            root.update(cx, |this, cx| {
                this.active_context_menu = Some(ActiveContextMenuItem {
                    position,
                    focus_handle: cx.focus_handle(),
                    previous_focused: previous,
                    builder: Rc::new(build),
                });
                cx.notify();
            });
        }
    }

    fn close_context_menu(&mut self, cx: &mut App) {
        if let Some(root) = glyph_root_layer(self, cx) {
            let previous = root.update(cx, |this, cx| {
                let previous =
                    this.active_context_menu.take().and_then(|item| item.previous_focused);
                cx.notify();
                previous
            });
            if let Some(focus) = previous.and_then(|handle| handle.upgrade()) {
                focus.focus(self, cx);
            }
        }
    }

    fn has_active_context_menu(&mut self, cx: &mut App) -> bool {
        glyph_root_layer(self, cx)
            .map(|r| r.read(cx).active_context_menu.is_some())
            .unwrap_or(false)
    }

    fn has_focused_input(&mut self, _cx: &mut App) -> bool {
        false
    }

    fn remove_notification(&mut self, id: impl Into<ElementId>, cx: &mut App) {
        let id = id.into();
        if let Some(root) = glyph_root_layer(self, cx) {
            root.update(cx, |this, cx| {
                let count = this.notifications.len();
                this.notifications
                    .retain(|(notification, _)| notification.id.as_ref() != Some(&id));
                if this.notifications.len() != count {
                    cx.notify();
                }
            });
        }
    }

    fn clear_notifications(&mut self, cx: &mut App) {
        if let Some(root) = glyph_root_layer(self, cx) {
            root.update(cx, |this, cx| {
                this.notifications.clear();
                cx.notify();
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlay::menu::PopupMenuItem;
    use gpui::{
        AppContext, Entity, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, TestAppContext,
        VisualTestContext, point,
    };
    use std::cell::Cell;

    struct ContextMenuHarness {
        activations: Rc<Cell<usize>>,
        opens_dialog: bool,
    }

    impl Render for ContextMenuHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let activations = self.activations.clone();
            let opens_dialog = self.opens_dialog;
            div().size_full().child(Button::new("context-trigger").label("Open menu").on_click(
                move |_, window, cx| {
                    let activations = activations.clone();
                    window.open_context_menu(cx, point(px(20.), px(64.)), move |menu, _, _| {
                        let activations = activations.clone();
                        menu.item(PopupMenuItem::new("Run command").id("context-command").on_click(
                            move |_, window, cx| {
                                activations.set(activations.get() + 1);
                                if opens_dialog {
                                    window.open_dialog(cx, |dialog, _, _| {
                                        dialog.title("Command details").child(
                                            Button::new("dialog-close").label("Close").on_click(
                                                |_, window, cx| {
                                                    window.close_dialog(cx);
                                                },
                                            ),
                                        )
                                    });
                                }
                            },
                        ))
                    });
                },
            ))
        }
    }

    fn harness(
        cx: &mut TestAppContext,
        opens_dialog: bool,
    ) -> (Entity<Root>, Rc<Cell<usize>>, &mut VisualTestContext) {
        cx.update(crate::init);
        let activations = Rc::new(Cell::new(0));
        let (root, cx) = cx.add_window_view({
            let activations = activations.clone();
            move |window, cx| {
                let view = cx.new(|_| ContextMenuHarness { activations, opens_dialog });
                Root::new(view, window, cx)
            }
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        (root, activations, cx)
    }

    fn snapshot(cx: &mut VisualTestContext, id: &str) -> gpui_base::test_support::ElementSnapshot {
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.simulate_next_frame(cx);
            window.draw(cx).clear(cx);
            gpui_base::test_support::find(window, &[], &ElementId::Name(id.to_owned().into()))
                .expect("rendered context menu control")
        })
    }

    fn click(cx: &mut VisualTestContext, id: &str) {
        let bounds = snapshot(cx, id).bounds();
        cx.simulate_click(bounds.center(), Modifiers::default());
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
    fn context_menu_click_activates_once_and_closes(cx: &mut TestAppContext) {
        let (root, activations, cx) = harness(cx, false);
        click(cx, "context-trigger");
        assert_eq!(snapshot(cx, "context-command").focused(), Some(true));
        click(cx, "context-command");
        assert_eq!(snapshot(cx, "context-trigger").focused(), Some(true));
        assert_eq!(activations.get(), 1);
        cx.update(|_, cx| {
            assert!(
                root.read(cx)
                    .plugin::<GlyphRootLayer>()
                    .unwrap()
                    .read(cx)
                    .active_context_menu
                    .is_none()
            )
        });
    }

    #[gpui::test]
    fn context_menu_escape_closes_and_restores_trigger_focus(cx: &mut TestAppContext) {
        let (root, activations, cx) = harness(cx, false);
        click(cx, "context-trigger");
        assert_eq!(snapshot(cx, "context-command").focused(), Some(true));
        press(cx, "escape");
        assert_eq!(snapshot(cx, "context-trigger").focused(), Some(true));
        assert_eq!(activations.get(), 0);
        cx.update(|_, cx| {
            assert!(
                root.read(cx)
                    .plugin::<GlyphRootLayer>()
                    .unwrap()
                    .read(cx)
                    .active_context_menu
                    .is_none()
            )
        });
    }

    #[gpui::test]
    fn context_menu_dialog_restores_focus_to_the_original_trigger(cx: &mut TestAppContext) {
        let (root, activations, cx) = harness(cx, true);
        click(cx, "context-trigger");
        click(cx, "context-command");
        snapshot(cx, "dialog-close");
        cx.update(|window, cx| {
            let layer = root.read(cx).plugin::<GlyphRootLayer>().unwrap();
            let layer = layer.read(cx);
            assert!(layer.active_context_menu.is_none());
            let dialog = layer.active_dialogs.last().expect("opened dialog");
            assert!(dialog.focus_handle.contains_focused(window, cx));
        });
        click(cx, "dialog-close");
        assert_eq!(snapshot(cx, "context-trigger").focused(), Some(true));
        assert_eq!(activations.get(), 1);
        cx.update(|_, cx| {
            assert!(
                root.read(cx)
                    .plugin::<GlyphRootLayer>()
                    .unwrap()
                    .read(cx)
                    .active_dialogs
                    .is_empty()
            )
        });
    }

    struct KeyboardHarness;

    impl Render for KeyboardHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .tab_group()
                .child(Button::new("first-command").label("First").on_click(|_, _, _| {}))
                .child(Button::new("second-command").label("Second").on_click(|_, _, _| {}))
        }
    }

    #[gpui::test]
    fn base_root_owns_tab_navigation_and_modal_focus_trapping(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|_| KeyboardHarness);
            Root::new(view, window, cx)
        });
        cx.update(|window, cx| {
            window.activate_window();
            assert!(root.read(cx).view().clone().downcast::<KeyboardHarness>().is_ok());
            assert!(root.read(cx).plugin::<GlyphRootLayer>().is_some());
        });
        cx.run_until_parked();
        click(cx, "first-command");
        press(cx, "tab");
        assert_eq!(snapshot(cx, "second-command").focused(), Some(true));
        press(cx, "shift-tab");
        assert_eq!(snapshot(cx, "first-command").focused(), Some(true));

        cx.update(|window, cx| {
            window.open_dialog(cx, |dialog, _, _| {
                dialog.title("Details").child(
                    div()
                        .tab_group()
                        .child(
                            Button::new("dialog-first-command")
                                .label("First")
                                .on_click(|_, _, _| {}),
                        )
                        .child(
                            Button::new("dialog-second-command")
                                .label("Second")
                                .on_click(|_, _, _| {}),
                        ),
                )
            });
        });
        snapshot(cx, "dialog-first-command");
        press(cx, "tab");
        assert_eq!(snapshot(cx, "dialog-first-command").focused(), Some(true));
        press(cx, "shift-tab");
        assert_eq!(snapshot(cx, "dialog-second-command").focused(), Some(true));
        press(cx, "tab");
        assert_eq!(snapshot(cx, "dialog-first-command").focused(), Some(true));
        cx.update(|window, cx| window.close_dialog(cx));
        assert_eq!(snapshot(cx, "first-command").focused(), Some(true));
    }

    #[gpui::test]
    fn closed_or_covered_overlays_cannot_steal_deferred_focus(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|_| KeyboardHarness);
            Root::new(view, window, cx)
        });
        click(cx, "first-command");

        cx.update(|window, cx| {
            window.open_dialog(cx, |dialog, _, _| dialog.title("Transient dialog"));
            window.close_dialog(cx);
        });
        assert_eq!(snapshot(cx, "first-command").focused(), Some(true));

        cx.update(|window, cx| {
            window.open_sheet(cx, |sheet, _, _| sheet.child("Transient sheet"));
            window.close_sheet(cx);
        });
        assert_eq!(snapshot(cx, "first-command").focused(), Some(true));

        cx.update(|window, cx| {
            window.open_dialog(cx, |dialog, _, _| {
                dialog
                    .title("Topmost dialog")
                    .child(Button::new("top-dialog-command").label("Done").on_click(|_, _, _| {}))
            });
            window.open_sheet(cx, |sheet, _, _| sheet.child("Covered sheet"));
        });
        snapshot(cx, "top-dialog-command");
        cx.update(|window, cx| {
            let layer = root.read(cx).plugin::<GlyphRootLayer>().unwrap();
            let layer = layer.read(cx);
            let dialog = layer.active_dialogs.last().unwrap();
            assert!(dialog.focus_handle.contains_focused(window, cx));
            assert!(
                !layer.active_sheet.as_ref().unwrap().focus_handle.contains_focused(window, cx)
            );
        });
    }
}
