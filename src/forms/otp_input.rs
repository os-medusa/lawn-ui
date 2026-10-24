use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::theme::ActiveTheme;

/// A segmented one-time password / PIN code input control.
#[derive(IntoElement)]
pub struct OtpInput {
    id: ElementId,
    length: usize,
    value: SharedString,
    disabled: bool,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
}

impl OtpInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            length: 4,
            value: "".into(),
            disabled: false,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        callback: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
}

impl Styled for OtpInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for OtpInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let chars: Vec<char> = self.value.chars().collect();

        let mut row = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap_2();

        for i in 0..self.length {
            let char_opt = chars.get(i).copied();
            let is_active = chars.len() == i;

            let char_str = char_opt.map(|c| c.to_string()).unwrap_or_default();

            let slot = div()
                .id(ElementId::Integer(i as u64))
                .flex()
                .items_center()
                .justify_center()
                .w(px(40.))
                .h(px(40.))
                .rounded(theme.radius_md)
                .bg(theme.background)
                .border_1()
                .border_color(if is_active {
                    theme.primary
                } else {
                    theme.border
                })
                .text_lg()
                .font_medium()
                .text_color(theme.foreground)
                .when(self.disabled, |this| this.opacity(0.5))
                .child(char_str);

            row = row.child(slot);
        }

        row
    }
}
