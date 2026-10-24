use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    Refineable, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px,
};
use std::rc::Rc;

use crate::icon::{Icon, IconSize};
use crate::theme::ActiveTheme;

/// Modern Shadcn Sidebar Container
#[derive(IntoElement)]
pub struct Sidebar {
    width: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { width: None, style: StyleRefinement::default(), children: Vec::new() }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Sidebar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let width = self.width.unwrap_or(px(260.));

        div()
            .flex()
            .flex_col()
            .w(width)
            .h_full()
            .bg(theme.sidebar_background)
            .text_color(theme.sidebar_foreground)
            .border_r_1()
            .border_color(theme.sidebar_border)
            .flex_shrink_0()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Header section at top of Sidebar.
#[derive(IntoElement)]
pub struct SidebarHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarHeader {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SidebarHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_3()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Scrollable Content section inside Sidebar.
#[derive(IntoElement)]
pub struct SidebarContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarContent {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SidebarContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id("sidebar-content")
            .flex()
            .flex_col()
            .flex_1()
            .overflow_y_scroll()
            .px_2()
            .py_1()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Footer section at bottom of Sidebar.
#[derive(IntoElement)]
pub struct SidebarFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarFooter {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SidebarFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_3()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Menu group inside Sidebar.
#[derive(IntoElement)]
pub struct SidebarGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .py_2()
            .gap_1()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Group header text.
#[derive(IntoElement)]
pub struct SidebarGroupLabel {
    label: SharedString,
}

impl SidebarGroupLabel {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into() }
    }
}

impl RenderOnce for SidebarGroupLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .px_2()
            .py_1()
            .text_xs()
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(theme.muted_foreground)
            .child(self.label)
    }
}

/// Interactive button item in a Sidebar menu.
#[derive(IntoElement)]
pub struct SidebarMenuButton {
    id: ElementId,
    icon: Option<Icon>,
    label: Option<SharedString>,
    active: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl SidebarMenuButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            label: None,
            active: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
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

impl Styled for SidebarMenuButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = gpui_base::Button::new(self.id)
            .selected(self.active)
            .when_some(self.label.clone(), |this, label| this.accessibility_label(label))
            .flex()
            .items_center()
            .gap_2()
            .w_full()
            .h(gpui::rems(32. / 14.))
            .px_2()
            .rounded(theme.radius)
            .text_sm()
            .cursor_default()
            .focus_visible(|style| {
                style.shadow(vec![
                    gpui::BoxShadow::new(px(0.), px(0.), theme.sidebar_ring).spread_radius(px(2.)),
                ])
            });

        if self.active {
            el = el
                .bg(theme.sidebar_accent)
                .text_color(theme.sidebar_accent_foreground)
                .font_weight(gpui::FontWeight::MEDIUM);
        } else {
            el = el
                .text_color(theme.sidebar_foreground)
                .hover(|s| s.bg(theme.sidebar_accent).text_color(theme.sidebar_accent_foreground));
        }

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |event, window, cx| {
                (on_click)(event, window, cx);
            });
        }

        el = el.map(|mut el| {
            el.style().refine(&self.style);
            el
        });

        if let Some(mut icon) = self.icon {
            icon = icon.size(IconSize::Sm);
            el = el.child(icon);
        }

        if let Some(label) = self.label {
            el = el.child(div().flex_1().truncate().child(label));
        }

        el.children(self.children)
    }
}
