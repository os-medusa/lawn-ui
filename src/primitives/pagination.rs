use gpui::prelude::*;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A pagination navigation component adhering to shadcn/ui.
#[derive(IntoElement)]
pub struct Pagination {
    id: Option<ElementId>,
    current_page: usize,
    total_pages: usize,
    disabled: bool,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            id: None,
            current_page: 1,
            total_pages: 1,
            disabled: false,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    pub fn total_pages(mut self, total: usize) -> Self {
        self.total_pages = total.max(1);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let current = self.current_page.min(self.total_pages);
        let total = self.total_pages;
        let disabled = self.disabled;
        let on_change = self.on_change.clone();

        let id = self.id.unwrap_or_else(|| ElementId::Integer(0));

        let mut row = div()
            .id(id)
            .flex()
            .items_center()
            .gap(rems(6. / 14.))
            .text_sm()
            .text_color(theme.foreground);

        row.style().refine(&self.style);

        // Previous button
        let can_prev = current > 1 && !disabled;
        let mut prev_btn = div()
            .id("pagination-prev")
            .h(rems(36. / 14.))
            .px(rems(12. / 14.))
            .rounded(theme.radius)
            .flex()
            .items_center()
            .gap(rems(4. / 14.))
            .when(can_prev, |this| {
                this.cursor_pointer().hover(|s| s.bg(theme.control_hover))
            })
            .when(!can_prev, |this| this.opacity(0.4).cursor_not_allowed())
            .child(Icon::new(IconName::ChevronLeft).size(IconSize::Xs))
            .child("Назад");

        if can_prev {
            if let Some(handler) = on_change.clone() {
                let target = current - 1;
                prev_btn = prev_btn.on_click(move |_, window, cx| (handler)(target, window, cx));
            }
        }
        row = row.child(prev_btn);

        // Visible page numbers calculation
        let start_page = if current <= 3 {
            1
        } else if current + 2 >= total {
            total.saturating_sub(4).max(1)
        } else {
            current - 2
        };
        let end_page = (start_page + 4).min(total);

        for p in start_page..=end_page {
            let is_active = p == current;
            let mut page_btn = div()
                .id(("pagination-page", p))
                .size(rems(36. / 14.))
                .rounded(theme.radius)
                .flex()
                .items_center()
                .justify_center()
                .text_sm()
                .font_medium()
                .when(is_active, |this| {
                    this.border_1()
                        .border_color(theme.border)
                        .bg(theme.background)
                        .text_color(theme.foreground)
                })
                .when(!is_active && !disabled, |this| {
                    this.cursor_pointer().hover(|s| s.bg(theme.control_hover))
                })
                .when(disabled, |this| this.opacity(0.4).cursor_not_allowed())
                .child(p.to_string());

            if !is_active && !disabled {
                if let Some(handler) = on_change.clone() {
                    page_btn = page_btn.on_click(move |_, window, cx| (handler)(p, window, cx));
                }
            }

            row = row.child(page_btn);
        }

        // Next button
        let can_next = current < total && !disabled;
        let mut next_btn = div()
            .id("pagination-next")
            .h(rems(36. / 14.))
            .px(rems(12. / 14.))
            .rounded(theme.radius)
            .flex()
            .items_center()
            .gap(rems(4. / 14.))
            .when(can_next, |this| {
                this.cursor_pointer().hover(|s| s.bg(theme.control_hover))
            })
            .when(!can_next, |this| this.opacity(0.4).cursor_not_allowed())
            .child("Вперед")
            .child(Icon::new(IconName::ChevronRight).size(IconSize::Xs));

        if can_next {
            if let Some(handler) = on_change.clone() {
                let target = current + 1;
                next_btn = next_btn.on_click(move |_, window, cx| (handler)(target, window, cx));
            }
        }
        row = row.child(next_btn);

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_properties() {
        let pag = Pagination::new()
            .current_page(2)
            .total_pages(10);

        assert_eq!(pag.current_page, 2);
        assert_eq!(pag.total_pages, 10);
    }
}
