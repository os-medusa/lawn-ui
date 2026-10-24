use gpui::prelude::FluentBuilder;
use gpui::{
    App, IntoElement, Pixels, Refineable, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::theme::ActiveTheme;

/// Modern Shadcn Skeleton loading placeholder
#[derive(IntoElement)]
pub struct Skeleton {
    width: Option<Pixels>,
    height: Option<Pixels>,
    circle: bool,
    style: StyleRefinement,
}

impl Skeleton {
    pub fn new() -> Self {
        Self { width: None, height: None, circle: false, style: StyleRefinement::default() }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    pub fn circle(mut self) -> Self {
        self.circle = true;
        self
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = div().bg(theme.muted).flex_shrink_0();

        if self.circle {
            el = el.rounded_full();
        } else {
            el = el.rounded(theme.radius_sm);
        }

        if let Some(w) = self.width {
            el = el.w(w);
        } else {
            el = el.w_full();
        }

        if let Some(h) = self.height {
            el = el.h(h);
        } else {
            el = el.h(px(16.));
        }

        el.map(|mut el| {
            el.style().refine(&self.style);
            el
        })
    }
}
