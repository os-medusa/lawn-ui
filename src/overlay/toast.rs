use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_base::StyledExt;

use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;

/// Visual variants for toast notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Warning,
    Destructive,
}

/// A modern Shadcn / Sonner-style floating toast notification.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    action: Option<(SharedString, Box<dyn Fn(&mut Window, &mut App) + 'static>)>,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Toast {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            action: None,
            on_close: None,
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some((label.into(), Box::new(on_click)));
        self
    }

    pub fn on_close(mut self, on_close: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(on_close));
        self
    }
}

impl RenderOnce for Toast {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let (border_color, icon_color, icon_name) = match self.variant {
            ToastVariant::Default => (theme.border, theme.foreground, None),
            ToastVariant::Success => (
                theme.success,
                theme.success,
                Some(IconName::Check),
            ),
            ToastVariant::Warning => (
                theme.warning,
                theme.warning,
                Some(IconName::TriangleAlert),
            ),
            ToastVariant::Destructive => (
                theme.destructive,
                theme.destructive,
                Some(IconName::CircleAlert),
            ),
        };

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w(px(356.))
            .p_4()
            .gap_3()
            .bg(theme.card)
            .border_1()
            .border_color(border_color)
            .rounded(theme.radius_md)
            .shadow_md()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .flex_1()
                    .when_some(icon_name, |this, name| {
                        this.child(
                            div()
                                .pt(px(2.))
                                .text_color(icon_color)
                                .child(Icon::new(name).size_4()),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(gpui::rems(1.))
                                    .font_medium()
                                    .text_color(theme.card_foreground)
                                    .child(self.title),
                            )
                            .when_some(self.description, |this, desc| {
                                this.child(
                                    div()
                                        .text_size(gpui::rems(12. / 14.))
                                        .text_color(theme.muted_foreground)
                                        .child(desc),
                                )
                            }),
                    ),
            )
            .when_some(self.action, |this, (label, on_click)| {
                this.child(
                    div()
                        .id("toast-action")
                        .px_2()
                        .py_1()
                        .rounded(theme.radius_sm)
                        .bg(theme.secondary)
                        .hover(|s| s.bg(theme.control_hover))
                        .text_size(gpui::rems(12. / 14.))
                        .font_medium()
                        .text_color(theme.secondary_foreground)
                        .cursor_pointer()
                        .on_click(move |_, window, cx| (on_click)(window, cx))
                        .child(label),
                )
            })
            .when_some(self.on_close, |this, on_close| {
                this.child(
                    div()
                        .id("toast-close")
                        .p_1()
                        .rounded(theme.radius_sm)
                        .hover(|s| s.bg(theme.control_hover))
                        .text_color(theme.muted_foreground)
                        .cursor_pointer()
                        .on_click(move |_, window, cx| (on_close)(window, cx))
                        .child(Icon::new(IconName::X).size_3p5()),
                )
            })
    }
}
