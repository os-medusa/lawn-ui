use gpui::prelude::FluentBuilder;
use gpui::{
    App, Entity, Focusable, InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, px, rems,
};
pub use gpui_base::input::TextareaState;

use crate::theme::ActiveTheme;

/// A multi-line input whose retained editing state is owned by the caller.
#[derive(IntoElement)]
pub struct Textarea {
    state: Entity<TextareaState>,
    disabled: bool,
    appearance: bool,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
}

impl Textarea {
    pub fn new(state: &Entity<TextareaState>) -> Self {
        Self {
            state: state.clone(),
            disabled: false,
            appearance: true,
            accessibility_label: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Hides the frame when composing the editor inside a larger input surface.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }
}

impl Styled for Textarea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Textarea {
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

        let mut el = gpui_base::input::InputBase::new(("glyph-textarea", self.state.entity_id()))
            .track_focus(&focus_handle)
            .focused(is_focused)
            .disabled(self.disabled)
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .min_h(rems(100. / 14.))
            .text_color(theme.foreground)
            .text_size(rems(1.))
            .when(self.appearance, |this| {
                this.p(rems(12. / 14.))
                    .rounded(theme.radius_2xl)
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .bg(theme.control_fill)
                    .when(!self.disabled, |this| {
                        this.hover(|style| style.border_color(theme.ring))
                            .focus_visible(|style| style.shadow(vec![focus_ring]))
                    })
            });

        el = el.child(gpui_base::input::Textarea::new(&self.state));

        if let Some(label) = self.accessibility_label {
            el = el.accessibility_label(label);
        }

        el.map(|mut el| {
            el.style().refine(&self.style);
            el
        })
        .styles(|styles| {
            styles
                .focused(|style| {
                    style.when(self.appearance, |style| style.border_color(theme.ring))
                })
                .disabled(|style| style.opacity(0.5).cursor_not_allowed())
        })
    }
}
