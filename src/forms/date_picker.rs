use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A date picker input control with an interactive calendar popover.
#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    placeholder: SharedString,
    selected_date: Option<(u32, u32, u32)>, // (year, month, day)
    disabled: bool,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn((u32, u32, u32), &mut Window, &mut App) + 'static>>,
}

impl DatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: "Pick a date...".into(),
            selected_date: None,
            disabled: false,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn selected_date(mut self, date: Option<(u32, u32, u32)>) -> Self {
        self.selected_date = date;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        callback: impl Fn((u32, u32, u32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
}

impl Styled for DatePicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DatePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let display_text = if let Some((year, month, day)) = self.selected_date {
            format!("{:04}-{:02}-{:02}", year, month, day).into()
        } else {
            self.placeholder.clone()
        };

        let is_placeholder = self.selected_date.is_none();

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(rems(36. / 14.))
            .px_3()
            .py_2()
            .gap_2()
            .bg(theme.background)
            .border_1()
            .border_color(theme.border)
            .rounded(theme.radius_md)
            .text_sm()
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
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child(Icon::new(IconName::Calendar).size(IconSize::Custom(px(16.)))),
            )
            .child(display_text)
    }
}
