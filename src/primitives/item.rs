use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A structured list item or card row adhering to shadcn/ui.
#[derive(IntoElement)]
pub struct Item {
    id: Option<ElementId>,
    icon: Option<Icon>,
    title: SharedString,
    description: Option<SharedString>,
    trailing: Option<AnyElement>,
    show_chevron: bool,
    clickable: bool,
    selected: bool,
    style: StyleRefinement,
    on_click: Option<Rc<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Item {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            icon: None,
            title: title.into(),
            description: None,
            trailing: None,
            show_chevron: false,
            clickable: false,
            selected: false,
            style: StyleRefinement::default(),
            on_click: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn trailing(mut self, trailing: impl IntoElement) -> Self {
        self.trailing = Some(trailing.into_any_element());
        self
    }

    pub fn show_chevron(mut self, show: bool) -> Self {
        self.show_chevron = show;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.clickable = true;
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for Item {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Item {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let bg_color = if self.selected {
            theme.accent
        } else {
            theme.card
        };

        let id = self.id.unwrap_or_else(|| ElementId::Integer(0));

        let mut row = div()
            .id(id)
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .p(rems(16. / 14.))
            .rounded(theme.radius)
            .border_1()
            .border_color(theme.border)
            .bg(bg_color)
            .gap(rems(16. / 14.));

        row.style().refine(&self.style);

        if self.clickable || self.on_click.is_some() {
            row = row
                .cursor_pointer()
                .hover(|s| s.bg(theme.control_hover));

            if let Some(on_click) = self.on_click {
                row = row.on_click(move |ev, window, cx| (on_click)(ev, window, cx));
            }
        }

        // Left section (icon + text)
        let mut left = div().flex().items_center().gap(rems(12. / 14.)).min_w_0().flex_1();
        if let Some(icon) = self.icon {
            left = left.child(
                div()
                    .size(rems(36. / 14.))
                    .rounded(theme.radius)
                    .bg(theme.muted)
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .child(icon),
            );
        }

        let mut text_col = div().flex().flex_col().gap_0p5().min_w_0().flex_1();
        text_col = text_col.child(
            div()
                .text_sm()
                .font_medium()
                .text_color(theme.foreground)
                .truncate()
                .child(self.title),
        );
        if let Some(desc) = self.description {
            text_col = text_col.child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .truncate()
                    .child(desc),
            );
        }
        left = left.child(text_col);
        row = row.child(left);

        // Right section (trailing + chevron)
        if self.trailing.is_some() || self.show_chevron {
            let mut right = div().flex().items_center().gap(rems(8. / 14.)).flex_shrink_0();
            if let Some(trailing) = self.trailing {
                right = right.child(trailing);
            }
            if self.show_chevron {
                right = right.child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::Sm)
                        .text_color(theme.muted_foreground),
                );
            }
            row = row.child(right);
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_properties() {
        let item = Item::new("Scheduled transfers")
            .description("Set up a transfer for a later date.")
            .show_chevron(true);

        assert_eq!(item.title.as_ref(), "Scheduled transfers");
        assert!(item.show_chevron);
    }
}
