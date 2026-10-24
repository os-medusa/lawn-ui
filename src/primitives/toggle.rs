use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::theme::ActiveTheme;

/// Visual variants of a [`Toggle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Sizing options for a [`Toggle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// A two-state button that can be either on (pressed) or off (unpressed).
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    pressed: bool,
    disabled: bool,
    variant: ToggleVariant,
    size: ToggleSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    on_toggle: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: false,
            disabled: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Md,
            style: StyleRefinement::default(),
            children: Vec::new(),
            on_toggle: None,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_toggle(
        mut self,
        callback: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(callback));
        self
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Toggle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let height = match self.size {
            ToggleSize::Sm => rems(28. / 14.),
            ToggleSize::Md => rems(36. / 14.),
            ToggleSize::Lg => rems(40. / 14.),
        };

        let padding = match self.size {
            ToggleSize::Sm => px(8.),
            ToggleSize::Md => px(12.),
            ToggleSize::Lg => px(16.),
        };

        let (bg, text_color, border_color) = if self.pressed {
            (
                theme.accent,
                theme.accent_foreground,
                theme.border,
            )
        } else {
            match self.variant {
                ToggleVariant::Default => (
                    gpui::hsla(0., 0., 0., 0.),
                    theme.foreground,
                    gpui::hsla(0., 0., 0., 0.),
                ),
                ToggleVariant::Outline => (
                    gpui::hsla(0., 0., 0., 0.),
                    theme.foreground,
                    theme.border,
                ),
            }
        };

        let on_toggle = self.on_toggle.clone();
        let pressed = self.pressed;
        let disabled = self.disabled;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(padding)
            .rounded(theme.radius_md)
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .text_sm()
            .font_medium()
            .text_color(text_color)
            .when(disabled, |this| this.opacity(0.5).cursor_not_allowed())
            .when(!disabled, |this| {
                this.cursor_pointer()
                    .hover(|s| s.bg(theme.control_hover))
                    .on_click(move |_, window, cx| {
                        if let Some(cb) = &on_toggle {
                            (cb)(!pressed, window, cx);
                        }
                    })
            })
            .children(self.children)
    }
}

/// A group of related [`Toggle`] items.
#[derive(IntoElement)]
pub struct ToggleGroup {
    id: ElementId,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl ToggleGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl ParentElement for ToggleGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ToggleGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .p_1()
            .rounded(theme.radius_md)
            .bg(theme.muted)
            .children(self.children)
    }
}
