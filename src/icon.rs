use gpui::prelude::FluentBuilder;
use gpui::{
    App, Hsla, IntoElement, Pixels, Refineable, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, px, rems, svg,
};
pub use gpui_kit_assets::{IconName, IconNamed};

use crate::primitives::traits::Sizable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
    Xs, // 12px
    Sm, // 14px
    Md, // 16px (default)
    Lg, // 20px
    Xl, // 24px
    Custom(Pixels),
}

impl IconSize {
    pub fn to_pixels(self) -> Pixels {
        match self {
            Self::Xs => px(12.),
            Self::Sm => px(14.),
            Self::Md => px(16.),
            Self::Lg => px(20.),
            Self::Xl => px(24.),
            Self::Custom(p) => p,
        }
    }
}

impl From<Pixels> for IconSize {
    fn from(px: Pixels) -> Self {
        Self::Custom(px)
    }
}

/// Modern Lucide Icon renderer for GPUI.
#[derive(Clone, IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Option<Hsla>,
    size: IconSize,
    style: StyleRefinement,
}

impl Icon {
    pub fn new(name: impl Into<IconSource>) -> Self {
        Self {
            path: name.into().into_path(),
            color: None,
            size: IconSize::Md,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: impl Into<IconSize>) -> Self {
        self.size = size.into();
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn xs(self) -> Self {
        self.size(IconSize::Xs)
    }

    pub fn sm(self) -> Self {
        self.size(IconSize::Sm)
    }

    pub fn md(self) -> Self {
        self.size(IconSize::Md)
    }

    pub fn lg(self) -> Self {
        self.size(IconSize::Lg)
    }

    pub fn xl(self) -> Self {
        self.size(IconSize::Xl)
    }

    pub fn xsmall(self) -> Self {
        self.size(IconSize::Xs)
    }

    pub fn small(self) -> Self {
        self.size(IconSize::Sm)
    }

    pub fn medium(self) -> Self {
        self.size(IconSize::Md)
    }

    pub fn large(self) -> Self {
        self.size(IconSize::Lg)
    }
}

pub enum IconSource {
    Name(IconName),
    Path(SharedString),
}

impl IconSource {
    pub fn into_path(self) -> SharedString {
        match self {
            Self::Name(name) => name.path(),
            Self::Path(path) => path,
        }
    }
}

impl From<IconName> for IconSource {
    fn from(name: IconName) -> Self {
        Self::Name(name)
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Icon::new(name)
    }
}

impl From<&str> for IconSource {
    fn from(path: &str) -> Self {
        Self::Path(path.to_string().into())
    }
}

impl From<SharedString> for IconSource {
    fn from(path: SharedString) -> Self {
        Self::Path(path)
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Sizable for Icon {
    fn with_size(mut self, size: impl Into<crate::primitives::traits::Size>) -> Self {
        self.size = match size.into() {
            crate::primitives::traits::Size::XSmall => IconSize::Xs,
            crate::primitives::traits::Size::Small => IconSize::Sm,
            crate::primitives::traits::Size::Medium => IconSize::Md,
            crate::primitives::traits::Size::Large => IconSize::Lg,
            crate::primitives::traits::Size::Custom(p) => IconSize::Custom(p),
        };
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size: gpui::DefiniteLength = match self.size {
            IconSize::Xs => rems(12. / 14.).into(),
            IconSize::Sm => rems(1.).into(),
            IconSize::Md => rems(16. / 14.).into(),
            IconSize::Lg => rems(20. / 14.).into(),
            IconSize::Xl => rems(24. / 14.).into(),
            // Explicit pixel sizes remain available for raster/platform alignment.
            IconSize::Custom(size) => size.into(),
        };
        let color = self.color.unwrap_or(window.text_style().color);

        svg()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .path(self.path)
            .size(size)
            .text_color(color)
            .flex_shrink_0()
    }
}
