use gpui::prelude::FluentBuilder;
use gpui::{
    App, IntoElement, ParentElement, Refineable, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div,
};

use crate::primitives::badge::{Badge, BadgeVariant};
use crate::theme::ActiveTheme;

/// Modern Shadcn Luma Metric / Stat Box with pill badge and fluid contours.
#[derive(IntoElement)]
pub struct MetricBox {
    label: SharedString,
    value: SharedString,
    badge: Option<(SharedString, BadgeVariant)>,
    style: StyleRefinement,
}

impl MetricBox {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            badge: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn with_badge(mut self, text: impl Into<SharedString>, variant: BadgeVariant) -> Self {
        self.badge = Some((text.into(), variant));
        self
    }
}

impl Styled for MetricBox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MetricBox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_4()
            .bg(theme.secondary)
            .border_1()
            .border_color(theme.border)
            .rounded(theme.radius_3xl)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(theme.muted_foreground)
                            .child(self.label),
                    )
                    .children(self.badge.map(|(text, variant)| {
                        Badge::new().variant(variant).child(text)
                    })),
            )
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child(self.value),
            )
    }
}
