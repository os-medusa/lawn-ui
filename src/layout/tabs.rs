use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    Refineable, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, rems,
};
use std::rc::Rc;

use crate::icon::Icon;
use crate::primitives::traits::Selectable;
use crate::theme::ActiveTheme;

/// Modern Shadcn Tabs container
#[derive(IntoElement)]
pub struct Tabs {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Tabs {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Tabs {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Bar containing tab triggers.
#[derive(IntoElement)]
pub struct TabsList {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TabsList {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for TabsList {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TabsList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TabsList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .items_center()
            .h(rems(36. / 14.))
            .p_0p5()
            .gap_0p5()
            .rounded(theme.radius_2xl)
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

/// Single tab button inside a TabsList.
#[derive(IntoElement)]
pub struct TabsTrigger {
    id: ElementId,
    label: SharedString,
    icon: Option<Icon>,
    active: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TabsTrigger {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
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

impl Selectable for TabsTrigger {
    fn selected(mut self, selected: bool) -> Self {
        self.active = selected;
        self
    }
}

impl ParentElement for TabsTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for TabsTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabsTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut el = gpui_base::Button::new(self.id)
            .selected(self.active)
            .accessibility_label(self.label.clone())
            .flex()
            .items_center()
            .justify_center()
            .gap_1p5()
            .h(rems(28. / 14.))
            .px(rems(12. / 14.))
            .rounded(theme.radius_xl)
            .text_size(gpui::rems(1.))
            .font_weight(gpui::FontWeight::NORMAL)
            .cursor_default()
            .focus_visible(|style| {
                style.shadow(vec![
                    gpui::BoxShadow::new(gpui::px(0.), gpui::px(0.), theme.ring)
                        .spread_radius(gpui::px(2.)),
                ])
            });

        if self.active {
            el = el.bg(theme.active_tab).text_color(theme.foreground);
        } else {
            el = el.text_color(theme.muted_foreground).hover(|s| s.text_color(theme.foreground));
        }

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |event, window, cx| {
                (on_click)(event, window, cx);
            });
        }

        if let Some(icon) = self.icon {
            el = el.child(icon.small());
        }

        el.child(self.label).children(self.children).map(|mut el| {
            el.style().refine(&self.style);
            el
        })
    }
}

/// Content panel rendered when tab is active.
#[derive(IntoElement)]
pub struct TabsContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TabsContent {
    pub fn new() -> Self {
        Self { style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl Default for TabsContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TabsContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TabsContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .pt_2()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}
