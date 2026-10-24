use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rems,
};
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::primitives::traits::Disableable;
use crate::theme::ActiveTheme;

/// A controlled checkbox; the owner applies the value requested by `on_change`.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    description: Option<SharedString>,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
    on_click: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            label: None,
            description: None,
            accessibility_label: None,
            style: StyleRefinement::default(),
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl Disableable for Checkbox {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        let mut indicator = div()
            .size(rems(16. / 14.))
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radius_checkbox)
            .border_1()
            .flex_shrink_0();

        if self.checked {
            indicator = indicator.bg(theme.primary).border_color(theme.primary).child(
                Icon::new(IconName::Check).size(IconSize::Xs).color(theme.primary_foreground),
            );
        } else {
            indicator = indicator.bg(theme.background).border_color(theme.border);
        }

        let indicator = indicator.map(|mut el| {
            el.style().refine(&self.style);
            el
        });

        let mut control = gpui_base::Checkbox::new(self.id)
            .checked(self.checked)
            .disabled(self.disabled)
            .flex()
            .items_start()
            .gap(rems(8. / 14.))
            .rounded(theme.radius_sm)
            .cursor_default()
            .styles(|styles| styles.disabled(|style| style.opacity(0.5).cursor_not_allowed()))
            .when(!self.disabled, |this| {
                this.focus_visible(|style| style.shadow(vec![focus_ring]))
                    .hover(|style| style.opacity(0.85))
            })
            .child(indicator);

        if let Some(label) = self.accessibility_label.or_else(|| self.label.clone()) {
            control = control.accessibility_label(label);
        }

        if let Some(on_click) = self.on_click {
            control = control.on_change(move |next, _, window, cx| {
                on_click(&(next == gpui_base::CheckboxState::Checked), window, cx);
            });
        }

        if let Some(label) = self.label {
            let label_col = div()
                .flex()
                .flex_col()
                .gap(rems(2. / 14.))
                .child(div().text_size(rems(1.)).text_color(theme.foreground).child(label))
                .when_some(self.description, |this, desc| {
                    this.child(
                        div()
                            .text_size(rems(12. / 14.))
                            .text_color(theme.muted_foreground)
                            .child(desc),
                    )
                });
            control = control.child(label_col);
        }

        control
    }
}
