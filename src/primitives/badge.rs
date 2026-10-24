use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Refineable, RenderOnce,
    StyleRefinement, Styled, Window, div, rems,
};

use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Warning,
    Success,
    Outline,
}

/// Modern Shadcn/UI Badge
#[derive(IntoElement)]
pub struct Badge {
    id: Option<ElementId>,
    variant: BadgeVariant,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            id: None,
            variant: BadgeVariant::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn default_variant(self) -> Self {
        self.variant(BadgeVariant::Default)
    }

    pub fn secondary(self) -> Self {
        self.variant(BadgeVariant::Secondary)
    }

    pub fn destructive(self) -> Self {
        self.variant(BadgeVariant::Destructive)
    }

    pub fn warning(self) -> Self {
        self.variant(BadgeVariant::Warning)
    }

    pub fn success(self) -> Self {
        self.variant(BadgeVariant::Success)
    }

    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }

    pub fn count(self, count: usize) -> Self {
        let label = if count > 99 { "99+".to_string() } else { count.to_string() };
        self.child(label)
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Badge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = div()
            .flex()
            .items_center()
            .justify_center()
            .h(rems(22. / 14.))
            .rounded(theme.radius_3xl)
            .px(rems(8. / 14.))
            .text_size(gpui::rems(12. / 14.))
            .font_weight(gpui::FontWeight::NORMAL)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            });

        el = match self.variant {
            BadgeVariant::Default => el.bg(theme.primary).text_color(theme.primary_foreground),
            BadgeVariant::Secondary => {
                el.bg(theme.secondary).text_color(theme.secondary_foreground)
            }
            BadgeVariant::Destructive => {
                el.bg(theme.destructive).text_color(theme.destructive_foreground)
            }
            BadgeVariant::Warning => el.bg(theme.warning).text_color(theme.warning_foreground),
            BadgeVariant::Success => el.bg(theme.success).text_color(theme.success_foreground),
            BadgeVariant::Outline => {
                el.border_1().border_color(theme.border).text_color(theme.foreground)
            }
        };

        el.children(self.children)
    }
}
