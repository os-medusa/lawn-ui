use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A node in a [`Tree`] view.
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub expanded: bool,
    pub selected: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            expanded: false,
            selected: false,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

/// An interactive hierarchical tree view.
#[derive(IntoElement)]
pub struct Tree {
    id: ElementId,
    nodes: Vec<TreeNode>,
    style: StyleRefinement,
    on_toggle: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    on_select: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
}

impl Tree {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            nodes: Vec::new(),
            style: StyleRefinement::default(),
            on_toggle: None,
            on_select: None,
        }
    }

    pub fn nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn node(mut self, node: TreeNode) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn on_toggle(
        mut self,
        callback: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(callback));
        self
    }

    pub fn on_select(
        mut self,
        callback: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(callback));
        self
    }
}

impl Styled for Tree {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Tree {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut elements: Vec<AnyElement> = Vec::new();

        for node in &self.nodes {
            collect_tree_node(
                &mut elements,
                node,
                0,
                &theme,
                self.on_toggle.as_ref(),
                self.on_select.as_ref(),
            );
        }

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_0p5()
            .w_full()
            .children(elements)
    }
}

fn collect_tree_node(
    elements: &mut Vec<AnyElement>,
    node: &TreeNode,
    depth: usize,
    theme: &crate::theme::Theme,
    on_toggle: Option<&Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    on_select: Option<&Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
) {
    let has_children = !node.children.is_empty();
    let indent = px(depth as f32 * 16.0);

    let node_id = node.id.clone();
    let on_select_cb = on_select.cloned();

    let row = div()
        .id(ElementId::Name(node.id.clone()))
        .flex()
        .flex_row()
        .items_center()
        .h(px(28.))
        .pl(indent + px(4.))
        .pr_2()
        .gap_1p5()
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .when(node.selected, |this| {
            this.bg(theme.accent)
                .text_color(theme.accent_foreground)
        })
        .when(!node.selected, |this| {
            this.hover(|s| s.bg(theme.control_hover))
                .text_color(theme.foreground)
        })
        .on_click(move |_, window, cx| {
            if let Some(cb) = &on_select_cb {
                (cb)(&node_id, window, cx);
            }
        })
        .child(
            div()
                .w(px(16.))
                .h(px(16.))
                .flex()
                .items_center()
                .justify_center()
                .when(has_children, |this| {
                    let chevron = if node.expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    };
                    this.text_color(theme.muted_foreground)
                        .child(Icon::new(chevron).size(IconSize::Custom(px(14.))))
                }),
        )
        .when_some(node.icon, |this, icon| {
            this.child(
                div()
                    .text_color(theme.muted_foreground)
                    .child(Icon::new(icon).size(IconSize::Custom(px(14.)))),
            )
        })
        .child(
            div()
                .text_sm()
                .font_medium()
                .child(node.label.clone()),
        );

    elements.push(row.into_any_element());

    if has_children && node.expanded {
        for child in &node.children {
            collect_tree_node(elements, child, depth + 1, theme, on_toggle, on_select);
        }
    }
}
