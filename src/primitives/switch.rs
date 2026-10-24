use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
use std::rc::Rc;

use crate::primitives::traits::Disableable;
use crate::theme::ActiveTheme;

/// A controlled switch for a setting that takes effect immediately.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            accessibility_label: None,
            style: StyleRefinement::default(),
            on_click: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_change(self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click(handler)
    }
}

impl Disableable for Switch {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        let thumb = div()
            .w(rems(24. / 14.))
            .h(rems(16. / 14.))
            .rounded(theme.radius_md)
            .bg(if self.checked { theme.card } else { gpui::white() });

        let mut el = gpui_base::Switch::new(self.id)
            .checked(self.checked)
            .disabled(self.disabled)
            .w(rems(44. / 14.))
            .h(rems(20. / 14.))
            .rounded(theme.radius_lg)
            .flex()
            .items_center()
            .px(rems(2. / 14.))
            .cursor_default()
            .flex_shrink_0()
            .styles(|styles| styles.disabled(|style| style.opacity(0.5).cursor_not_allowed()))
            .when(!self.disabled, |this| {
                this.focus_visible(|style| style.shadow(vec![focus_ring]))
                    .hover(|style| style.opacity(0.85))
            });

        if self.checked {
            el = el.bg(theme.primary).justify_end();
        } else {
            el = el.bg(theme.checkbox_fill).justify_start();
        }

        if let Some(label) = self.accessibility_label {
            el = el.accessibility_label(label);
        }
        if let Some(on_click) = self.on_click {
            el = el.on_change(move |next, _, window, cx| on_click(&next, window, cx));
        }

        el.child(thumb).map(|mut el| {
            el.style().refine(&self.style);
            el
        })
    }
}
