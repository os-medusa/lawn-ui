use gpui::prelude::FluentBuilder;
use gpui::{
    App, IntoElement, ParentElement, Refineable, RenderOnce, StyleRefinement, Styled, Window, div,
    px, relative,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn/UI Luma Progress bar with fluid rounded contours.
#[derive(IntoElement)]
pub struct Progress {
    value: f32, // 0.0 to 1.0
    style: StyleRefinement,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .relative()
            .w_full()
            .h(px(8.))
            .bg(theme.secondary)
            .rounded(theme.radius_sm)
            .overflow_hidden()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .child(
                div()
                    .h_full()
                    .w(relative(self.value))
                    .bg(theme.primary)
                    .rounded(theme.radius_sm),
            )
    }
}
