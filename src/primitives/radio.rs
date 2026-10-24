use gpui::prelude::*;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, px, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::theme::ActiveTheme;

/// A single radio button element adhering to the shadcn/ui design tokens.
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    description: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Radio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            label: None,
            description: None,
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

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let disabled = self.disabled;
        let checked = self.checked;

        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        // Outer circular control
        let mut circle = div()
            .size(rems(16. / 14.))
            .rounded_full()
            .border_1()
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .bg(theme.background)
            .when(!checked, |this| this.border_color(theme.border))
            .when(checked, |this| this.border_color(theme.primary));

        if checked {
            circle = circle.child(
                div()
                    .size(rems(8. / 14.))
                    .rounded_full()
                    .bg(theme.primary),
            );
        }

        let mut row = div()
            .id(self.id)
            .flex()
            .items_start()
            .gap(rems(8. / 14.))
            .when(!disabled, |this| this.cursor_pointer())
            .when(disabled, |this| this.opacity(0.5).cursor_not_allowed());

        if !disabled {
            row = row.focus_visible(|style| style.shadow(vec![focus_ring]));
            if let Some(on_click) = self.on_click {
                row = row.on_click(move |ev, window, cx| (on_click)(ev, window, cx));
            }
        }

        row = row.child(circle);

        if self.label.is_some() || self.description.is_some() {
            let mut text_col = div().flex().flex_col().gap_0p5();
            if let Some(label) = self.label {
                text_col = text_col.child(
                    div()
                        .text_sm()
                        .font_medium()
                        .text_color(theme.foreground)
                        .child(label),
                );
            }
            if let Some(desc) = self.description {
                text_col = text_col.child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(desc),
                );
            }
            row = row.child(text_col);
        }

        row
    }
}

/// A group container for multiple [`Radio`] items.
#[derive(IntoElement, Default)]
pub struct RadioGroup {
    children: Vec<AnyElement>,
    horizontal: bool,
}

impl RadioGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().flex().gap(rems(12. / 14.));
        if self.horizontal {
            el = el.flex_row().items_center();
        } else {
            el = el.flex_col();
        }
        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_properties_and_group() {
        let radio = Radio::new("opt-1")
            .label("Option 1")
            .description("First option description")
            .checked(true)
            .disabled(false);

        assert!(radio.checked);
        assert!(!radio.disabled);
        assert_eq!(radio.label.as_deref(), Some("Option 1"));

        let group = RadioGroup::new()
            .horizontal(true)
            .child(radio)
            .child(Radio::new("opt-2").label("Option 2"));

        assert_eq!(group.children.len(), 2);
        assert!(group.horizontal);
    }
}
