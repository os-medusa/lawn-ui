use gpui::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, RenderOnce, Styled, div, rems};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Tinted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageAlignment {
    #[default]
    Start,
    End,
}

#[derive(IntoElement, Default)]
pub struct Bubble {
    alignment: MessageAlignment,
    variant: BubbleVariant,
    children: Vec<AnyElement>,
}

impl Bubble {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alignment(mut self, alignment: MessageAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for Bubble {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        let is_end = matches!(self.alignment, MessageAlignment::End);

        let (bg_color, foreground) = match self.variant {
            BubbleVariant::Tinted => (theme.primary, theme.primary_foreground),
            BubbleVariant::Secondary => (theme.muted, theme.foreground),
            BubbleVariant::Default => (theme.card, theme.card_foreground),
        };

        div()
            .flex()
            .w_full()
            .min_w_0()
            .when(is_end, |this| this.justify_end())
            .when(!is_end, |this| this.justify_start())
            .child(
                div()
                    .max_w(rems(540. / 14.))
                    .min_w_0()
                    .px_4()
                    .py_3()
                    .rounded(theme.radius_xl)
                    .bg(bg_color)
                    .text_base()
                    .text_color(foreground)
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .children(self.children),
            )
    }
}
