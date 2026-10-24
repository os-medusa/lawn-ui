use gpui::prelude::*;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

/// An inline banner message communicating prominent status or updates to the user.
#[derive(IntoElement)]
pub struct Alert {
    id: Option<ElementId>,
    variant: AlertVariant,
    title: Option<SharedString>,
    description: Option<SharedString>,
    icon: Option<Icon>,
    dismissible: bool,
    on_close: Option<Rc<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Alert {
    pub fn new() -> Self {
        Self {
            id: None,
            variant: AlertVariant::Default,
            title: None,
            description: None,
            icon: None,
            dismissible: false,
            on_close: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn info(self) -> Self {
        self.variant(AlertVariant::Info)
    }

    pub fn success(self) -> Self {
        self.variant(AlertVariant::Success)
    }

    pub fn warning(self) -> Self {
        self.variant(AlertVariant::Warning)
    }

    pub fn destructive(self) -> Self {
        self.variant(AlertVariant::Destructive)
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn on_close(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dismissible = true;
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Alert {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let (border_color, bg_color, text_color, default_icon) = match self.variant {
            AlertVariant::Default => (
                theme.border,
                theme.card,
                theme.foreground,
                IconName::Info,
            ),
            AlertVariant::Info => (
                theme.ring.opacity(0.4),
                theme.card,
                theme.foreground,
                IconName::Info,
            ),
            AlertVariant::Success => (
                theme.success.opacity(0.4),
                theme.card,
                theme.foreground,
                IconName::Check,
            ),
            AlertVariant::Warning => (
                theme.warning.opacity(0.4),
                theme.card,
                theme.foreground,
                IconName::TriangleAlert,
            ),
            AlertVariant::Destructive => (
                theme.destructive.opacity(0.4),
                theme.card,
                theme.destructive,
                IconName::CircleAlert,
            ),
        };

        let icon = self.icon.unwrap_or_else(|| {
            Icon::new(default_icon)
                .size(IconSize::Sm)
                .text_color(text_color)
        });

        let id = self.id.unwrap_or_else(|| ElementId::Integer(0));

        let mut row = div()
            .id(id)
            .w_full()
            .flex()
            .items_start()
            .gap(rems(12. / 14.))
            .p(rems(16. / 14.))
            .rounded(theme.radius)
            .border_1()
            .border_color(border_color)
            .bg(bg_color);

        row.style().refine(&self.style);

        row = row.child(icon);

        let mut content = div().flex().flex_col().gap_1().flex_1();
        if let Some(title) = self.title {
            content = content.child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(text_color)
                    .child(title),
            );
        }
        if let Some(desc) = self.description {
            content = content.child(
                div()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child(desc),
            );
        }
        content = content.children(self.children);
        row = row.child(content);

        if self.dismissible {
            let mut close_btn = div()
                .id("alert-dismiss-btn")
                .size(rems(20. / 14.))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .cursor_pointer()
                .hover(|s| s.bg(theme.control_hover))
                .child(
                    Icon::new(IconName::Close)
                        .size(IconSize::Xs)
                        .text_color(theme.muted_foreground),
                );

            if let Some(on_close) = self.on_close {
                close_btn = close_btn.on_click(move |ev, window, cx| (on_close)(ev, window, cx));
            }

            row = row.child(close_btn);
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_variants_and_title() {
        let alert = Alert::new()
            .warning()
            .title("Heads up!")
            .description("Your preferences have been updated.")
            .dismissible(true);

        assert_eq!(alert.variant, AlertVariant::Warning);
        assert_eq!(alert.title.as_deref(), Some("Heads up!"));
        assert!(alert.dismissible);
    }
}
