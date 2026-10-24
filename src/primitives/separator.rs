use gpui::prelude::FluentBuilder;
use gpui::{App, IntoElement, Refineable, RenderOnce, StyleRefinement, Styled, Window, div, px};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Modern Shadcn/UI Separator
#[derive(IntoElement)]
pub struct Separator {
    orientation: Orientation,
    style: StyleRefinement,
}

impl Separator {
    pub fn new() -> Self {
        Self { orientation: Orientation::Horizontal, style: StyleRefinement::default() }
    }

    pub fn horizontal() -> Self {
        Self::new().with_orientation(Orientation::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new().with_orientation(Orientation::Vertical)
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let el = div().bg(theme.border).flex_shrink_0().map(|mut el| {
            el.style().refine(&self.style);
            el
        });

        match self.orientation {
            Orientation::Horizontal => el.h(px(1.)).w_full(),
            Orientation::Vertical => el.w(px(1.)).h_full(),
        }
    }
}
