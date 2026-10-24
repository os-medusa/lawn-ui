use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, IntoElement, ParentElement, RenderOnce, Styled, TitlebarOptions,
    Window, WindowOptions, div, point, px,
};

use crate::theme::ActiveTheme;

/// Desktop Window TitleBar element and helper for GPUI.
#[derive(IntoElement, Default)]
pub struct TitleBar {
    children: Vec<AnyElement>,
    on_close_window: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TitleBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_close_window<F>(mut self, callback: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_close_window = Some(Rc::new(callback));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn window_options() -> WindowOptions {
        WindowOptions {
            titlebar: Some(Self::title_bar_options()),
            app_owns_titlebar_drag: true,
            ..Default::default()
        }
    }

    pub fn title_bar_options() -> TitlebarOptions {
        TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .h(gpui::rems(36. / 14.))
            .w_full()
            .flex()
            .items_center()
            .bg(theme.background)
            .border_b_1()
            .border_color(theme.border)
            .children(self.children)
    }
}
