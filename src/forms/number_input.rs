use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A numeric stepper input control.
#[derive(IntoElement)]
pub struct NumberInput {
    id: ElementId,
    value: i64,
    min: Option<i64>,
    max: Option<i64>,
    step: i64,
    disabled: bool,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn(i64, &mut Window, &mut App) + 'static>>,
}

impl NumberInput {
    pub fn new(id: impl Into<ElementId>, value: i64) -> Self {
        Self {
            id: id.into(),
            value,
            min: None,
            max: None,
            step: 1,
            disabled: false,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: i64) -> Self {
        self.step = step;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        callback: impl Fn(i64, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
}

impl Styled for NumberInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NumberInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let can_decrement = !self.disabled && self.min.map_or(true, |m| self.value > m);
        let can_increment = !self.disabled && self.max.map_or(true, |m| self.value < m);

        let on_change_dec = self.on_change.clone();
        let on_change_inc = self.on_change.clone();
        let current_val = self.value;
        let step = self.step;

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(rems(36. / 14.))
            .w(px(140.))
            .px_2()
            .bg(theme.background)
            .border_1()
            .border_color(theme.border)
            .rounded(theme.radius_md)
            .text_sm()
            .when(self.disabled, |this| {
                this.opacity(0.5).cursor_not_allowed()
            })
            // Decrement button
            .child(
                div()
                    .id("number-input-dec")
                    .p_1()
                    .rounded(theme.radius_sm)
                    .when(can_decrement, |this| {
                        this.cursor_pointer()
                            .hover(|s| s.bg(theme.control_hover))
                    })
                    .when(!can_decrement, |this| {
                        this.opacity(0.4).cursor_not_allowed()
                    })
                    .text_color(theme.foreground)
                    .on_click(move |_, window, cx| {
                        if can_decrement {
                            if let Some(cb) = &on_change_dec {
                                (cb)(current_val - step, window, cx);
                            }
                        }
                    })
                    .child(Icon::new(IconName::Minus).size(IconSize::Custom(px(14.)))),
            )
            // Value display
            .child(
                div()
                    .flex_1()
                    .text_center()
                    .font_medium()
                    .text_color(theme.foreground)
                    .child(self.value.to_string()),
            )
            // Increment button
            .child(
                div()
                    .id("number-input-inc")
                    .p_1()
                    .rounded(theme.radius_sm)
                    .when(can_increment, |this| {
                        this.cursor_pointer()
                            .hover(|s| s.bg(theme.control_hover))
                    })
                    .when(!can_increment, |this| {
                        this.opacity(0.4).cursor_not_allowed()
                    })
                    .text_color(theme.foreground)
                    .on_click(move |_, window, cx| {
                        if can_increment {
                            if let Some(cb) = &on_change_inc {
                                (cb)(current_val + step, window, cx);
                            }
                        }
                    })
                    .child(Icon::new(IconName::Plus).size(IconSize::Custom(px(14.)))),
            )
    }
}
