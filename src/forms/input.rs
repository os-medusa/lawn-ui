use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, Entity, Focusable, InteractiveElement, IntoElement, ParentElement, Refineable,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
pub use gpui_base::input::{InputEvent, InputState};

use crate::primitives::traits::Disableable;
use crate::theme::ActiveTheme;

/// A single-line input whose retained editing state is owned by the caller.
#[derive(IntoElement)]
pub struct Input {
    state: Entity<InputState>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    disabled: bool,
    cleanable: bool,
    is_small: bool,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
}

impl Input {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            prefix: None,
            suffix: None,
            disabled: false,
            cleanable: false,
            is_small: false,
            accessibility_label: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    pub fn small(mut self) -> Self {
        self.is_small = true;
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }
}

impl Disableable for Input {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            if state.presentation().is_disabled() != self.disabled {
                state.set_disabled(self.disabled, cx);
            }
        });
        let theme = cx.theme();
        let focus_handle = self.state.read(cx).focus_handle(cx).tab_stop(!self.disabled);
        let is_focused = focus_handle.is_focused(window);
        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        let (h, px_padding, text_size) = if self.is_small {
            (rems(28. / 14.), rems(8. / 14.), rems(12. / 14.))
        } else {
            (rems(36. / 14.), rems(12. / 14.), rems(1.))
        };

        let mut el = gpui_base::input::InputBase::new(("glyph-input", self.state.entity_id()))
            .track_focus(&focus_handle)
            .focused(is_focused)
            .disabled(self.disabled)
            .flex()
            .items_center()
            .w_full()
            .min_w_0()
            .h(h)
            .px(px_padding)
            .gap(rems(8. / 14.))
            .rounded(theme.radius_3xl)
            .border_1()
            .border_color(gpui::transparent_black())
            .bg(theme.control_fill)
            .text_color(theme.foreground)
            .text_size(text_size)
            .when(!self.disabled, |this| {
                this.hover(|style| style.border_color(theme.ring))
                    .focus_visible(|style| style.shadow(vec![focus_ring]))
            });

        if let Some(prefix) = self.prefix {
            el = el.child(prefix);
        }

        el = el.child(
            div()
                .flex_1()
                .min_w_0()
                .h_full()
                .flex()
                .items_center()
                .child(gpui_base::input::Input::new(&self.state)),
        );

        let is_empty = self.state.read(cx).value().is_empty();
        if self.cleanable && !is_empty && !self.disabled {
            let state = self.state.clone();
            el = el.child(
                gpui_base::Button::new(("glyph-input-clear", self.state.entity_id()))
                    .accessibility_label("Clear input")
                    .size(rems(20. / 14.))
                    .rounded(theme.radius_sm)
                    .cursor_default()
                    .text_color(theme.muted_foreground)
                    .hover(|s| s.text_color(theme.foreground))
                    .focus_visible(|s| s.bg(theme.accent).text_color(theme.foreground))
                    .on_click(move |_, window, cx| {
                        state.update(cx, |s, cx| {
                            s.set_value("", window, cx);
                            s.focus_handle(cx).focus(window, cx);
                        });
                    })
                    .child(
                        crate::icon::Icon::new(crate::icon::IconName::Close)
                            .size(crate::icon::IconSize::Xs),
                    ),
            );
        }

        if let Some(suffix) = self.suffix {
            el = el.child(suffix);
        }

        el = el
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .styles(|styles| {
                styles
                    .focused(|style| style.border_color(theme.ring))
                    .disabled(|style| style.opacity(0.5).cursor_not_allowed())
            });

        if let Some(label) = self.accessibility_label {
            el = el.accessibility_label(label);
        }
        el
    }
}
