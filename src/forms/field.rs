use gpui::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div};

use crate::theme::ActiveTheme;

#[derive(IntoElement, Default)]
pub struct Field {
    label: Option<SharedString>,
    description: Option<SharedString>,
    error: Option<SharedString>,
    child: Option<AnyElement>,
}

impl Field {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for Field {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap_1p5()
            .w_full()
            .when_some(self.label, |this, lbl| {
                this.child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(theme.foreground)
                        .child(lbl),
                )
            })
            .when_some(self.description, |this, desc| {
                this.child(div().text_xs().text_color(theme.muted_foreground).child(desc))
            })
            .children(self.child)
            .when_some(self.error, |this, err| {
                this.child(
                    div()
                        .text_xs()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(theme.destructive)
                        .child(err),
                )
            })
    }
}
