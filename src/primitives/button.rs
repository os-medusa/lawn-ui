use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    Refineable, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rems,
};
use std::rc::Rc;

use crate::icon::{Icon, IconSize};
use crate::primitives::traits::{Disableable, Selectable, Sizable, Size};
use crate::theme::ActiveTheme;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Danger,
    Warning,
    Success,
    Outline,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonSize {
    XSmall,
    Small,
    #[default]
    Default,
    Large,
    Icon,
}

pub trait ButtonVariants: Sized {
    fn with_variant(self, variant: ButtonVariant) -> Self;

    fn default_variant(self) -> Self {
        self.with_variant(ButtonVariant::Default)
    }

    fn primary(self) -> Self {
        self.with_variant(ButtonVariant::Primary)
    }

    fn secondary(self) -> Self {
        self.with_variant(ButtonVariant::Secondary)
    }

    fn destructive(self) -> Self {
        self.with_variant(ButtonVariant::Destructive)
    }

    fn danger(self) -> Self {
        self.with_variant(ButtonVariant::Danger)
    }

    fn warning(self) -> Self {
        self.with_variant(ButtonVariant::Warning)
    }

    fn success(self) -> Self {
        self.with_variant(ButtonVariant::Success)
    }

    fn outline(self) -> Self {
        self.with_variant(ButtonVariant::Outline)
    }

    fn ghost(self) -> Self {
        self.with_variant(ButtonVariant::Ghost)
    }

    fn link(self) -> Self {
        self.with_variant(ButtonVariant::Link)
    }
}

/// A themed command button with native keyboard and focus behavior.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    variant: ButtonVariant,
    size: ButtonSize,
    label: Option<SharedString>,
    icon: Option<Icon>,
    disabled: bool,
    selected: bool,
    loading: bool,
    pill: bool,
    dropdown_caret: bool,
    tooltip: Option<SharedString>,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    dropdown_menu: Option<(
        gpui::Anchor,
        Rc<
            dyn Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
                + 'static,
        >,
    )>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            label: None,
            icon: None,
            disabled: false,
            selected: false,
            loading: false,
            pill: false,
            dropdown_caret: false,
            tooltip: None,
            accessibility_label: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
            on_click: None,
            dropdown_menu: None,
        }
    }

    pub fn pill(mut self) -> Self {
        self.pill = true;
        self
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.variant = ButtonVariant::Destructive;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }

    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    pub fn link(mut self) -> Self {
        self.variant = ButtonVariant::Link;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn tooltip(mut self, text: impl Into<SharedString>) -> Self {
        self.tooltip = Some(text.into());
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    pub fn dropdown_caret(mut self, caret: bool) -> Self {
        self.dropdown_caret = caret;
        self
    }

    pub fn dropdown_menu<F>(self, builder: F) -> Self
    where
        F: Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
            + 'static,
    {
        self.dropdown_menu_with_anchor(gpui::Anchor::TopLeft, builder)
    }

    pub fn dropdown_menu_with_anchor<F>(mut self, anchor: gpui::Anchor, builder: F) -> Self
    where
        F: Fn(crate::overlay::menu::Menu, &mut Window, &mut App) -> crate::overlay::menu::Menu
            + 'static,
    {
        self.dropdown_menu = Some((anchor, Rc::new(builder)));
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn xsmall(self) -> Self {
        self.size(ButtonSize::XSmall)
    }

    pub fn small(self) -> Self {
        self.size(ButtonSize::Small)
    }

    pub fn medium(self) -> Self {
        self.size(ButtonSize::Default)
    }

    pub fn large(self) -> Self {
        self.size(ButtonSize::Large)
    }

    pub fn icon_size(self) -> Self {
        self.size(ButtonSize::Icon)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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

impl ButtonVariants for Button {
    fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Sizable for Button {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = match size.into() {
            Size::XSmall => ButtonSize::XSmall,
            Size::Small => ButtonSize::Small,
            Size::Medium => ButtonSize::Default,
            Size::Large => ButtonSize::Large,
            Size::Custom(_) => ButtonSize::Default,
        };
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Button {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled || self.loading;
        if let Some((anchor, builder)) = self.dropdown_menu.take().filter(|_| !disabled) {
            let button_id = self.id.clone();
            let popover_id = ElementId::Name(format!("{}:popover", button_id).into());
            let menu_id = ElementId::Name(format!("{}:menu", button_id).into());

            return gpui_base::Popover::new(popover_id)
                .anchor(anchor)
                .offset(window.rem_size() * (4. / 14.))
                .trigger_with(move |is_open, _window, _cx| {
                    let mut btn = self;
                    if is_open {
                        btn.selected = true;
                    }
                    btn.into_any_element()
                })
                .content(move |_state, window, cx| {
                    let popover_entity = cx.entity().downgrade();
                    let close_fn = Rc::new(move |window: &mut Window, cx: &mut App| {
                        if let Some(popover) = popover_entity.upgrade() {
                            popover.update(cx, |state, cx| {
                                state.dismiss(window, cx);
                            });
                        }
                    });

                    let menu = builder(crate::overlay::menu::Menu::new().id(menu_id), window, cx);
                    menu.render(close_fn, window, cx)
                })
                .into_any_element();
        }

        let theme = cx.theme();

        let is_icon_only = self.icon.is_some() && self.label.is_none() && self.children.is_empty();
        let is_small = matches!(self.size, ButtonSize::XSmall | ButtonSize::Small);
        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        let mut el = gpui_base::Button::new(self.id)
            .disabled(disabled)
            .selected(self.selected)
            .flex()
            .items_center()
            .justify_center()
            .gap(rems(6. / 14.))
            .rounded(if self.pill { px(9999.) } else { theme.radius_4xl })
            .font_weight(gpui::FontWeight::NORMAL)
            .cursor_default()
            .flex_shrink_0();

        // Luma control scale directly matching Figcraft (h: 28px/36px/40px, px: 12px, font: 14px)
        el = match self.size {
            ButtonSize::XSmall => {
                if is_icon_only {
                    el.size(rems(24. / 14.))
                } else {
                    el.h(rems(24. / 14.)).px(rems(8. / 14.)).text_size(rems(12. / 14.))
                }
            }
            ButtonSize::Small => {
                if is_icon_only {
                    el.size(rems(28. / 14.))
                } else {
                    el.h(rems(28. / 14.)).px(rems(12. / 14.)).text_size(rems(1.))
                }
            }
            ButtonSize::Large => {
                if is_icon_only {
                    el.size(rems(40. / 14.))
                } else {
                    el.h(rems(40. / 14.)).px(rems(12. / 14.)).text_size(rems(1.))
                }
            }
            ButtonSize::Icon => el.size(rems(36. / 14.)),
            ButtonSize::Default => {
                if is_icon_only {
                    el.size(rems(36. / 14.))
                } else {
                    el.h(rems(36. / 14.)).px(rems(12. / 14.)).text_size(rems(1.))
                }
            }
        };

        el = match self.variant {
            ButtonVariant::Default | ButtonVariant::Primary => {
                el.bg(theme.primary).text_color(theme.primary_foreground)
            }
            ButtonVariant::Secondary => {
                el.bg(theme.secondary).text_color(theme.secondary_foreground)
            }
            ButtonVariant::Outline => {
                el.border_1().border_color(theme.border).text_color(theme.foreground)
            }
            ButtonVariant::Destructive | ButtonVariant::Danger => {
                el.bg(theme.destructive).text_color(theme.destructive_foreground)
            }
            ButtonVariant::Warning => el.bg(theme.warning).text_color(theme.warning_foreground),
            ButtonVariant::Success => el.bg(theme.success).text_color(theme.success_foreground),
            ButtonVariant::Ghost => el.text_color(theme.foreground),
            ButtonVariant::Link => el.text_color(theme.primary).underline(),
        };

        el = el
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .styles(|styles| {
                styles
                    .selected(|style| style.bg(theme.accent).text_color(theme.accent_foreground))
                    .disabled(|style| style.opacity(0.5).cursor_not_allowed())
            });

        if !disabled {
            el = el.focus_visible(|style| style.border_color(theme.ring).shadow(vec![focus_ring]));
            el = match self.variant {
                ButtonVariant::Outline | ButtonVariant::Ghost => el
                    .hover(|style| style.bg(theme.accent))
                    .active(|style| style.bg(theme.accent)),
                ButtonVariant::Secondary => el
                    .hover(|style| style.bg(theme.secondary_hover))
                    .active(|style| style.opacity(0.8)),
                ButtonVariant::Link => el.hover(|style| style.underline()),
                ButtonVariant::Default | ButtonVariant::Primary => el
                    .hover(|style| style.bg(theme.primary.opacity(0.9)))
                    .active(|style| style.opacity(0.8)),
                ButtonVariant::Destructive | ButtonVariant::Danger => el
                    .hover(|style| style.bg(theme.destructive.opacity(0.9)))
                    .active(|style| style.opacity(0.8)),
                _ => el.hover(|style| style.opacity(0.9)).active(|style| style.opacity(0.8)),
            };
            if let Some(on_click) = self.on_click {
                el = el.on_click(move |event, window, cx| (on_click)(event, window, cx));
            }
        }

        if let Some(label) =
            self.accessibility_label.or_else(|| self.label.clone()).or_else(|| self.tooltip.clone())
        {
            el = el.accessibility_label(label);
        }

        if let Some(mut icon) = self.icon {
            let icon_sz = if is_small { IconSize::Sm } else { IconSize::Md };
            icon = icon.size(icon_sz);
            el = el.child(icon);
        }

        if let Some(label) = self.label {
            el = el.child(label);
        }

        if self.dropdown_caret {
            let icon_sz = if is_small { IconSize::Sm } else { IconSize::Md };
            el = el.child(
                Icon::new(crate::icon::IconName::ChevronDown)
                    .size(icon_sz)
                    .text_color(theme.muted_foreground),
            );
        }

        if let Some(tooltip) = self.tooltip {
            el = el.tooltip(move |window, cx| {
                crate::overlay::tooltip::Tooltip::new(tooltip.clone()).build(window, cx)
            });
        }

        el.children(self.children).into_any_element()
    }
}

/// Group of adjacent buttons sharing a single visual container.
#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), style: StyleRefinement::default(), children: Vec::new() }
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .id(self.id)
            .flex()
            .items_center()
            .rounded(theme.radius)
            .border_1()
            .border_color(theme.border)
            .bg(theme.muted.opacity(0.35))
            .p_0p5()
            .gap_1()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        forms::{
            input::{Input, InputState},
            textarea::{Textarea, TextareaState},
        },
        primitives::{checkbox::Checkbox, switch::Switch},
    };
    use gpui::{
        AppContext, Context, Entity, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Render,
        TestAppContext, VisualTestContext, point,
    };

    struct Controls {
        disabled: bool,
        clicks: usize,
        checked: bool,
        checkbox_changes: usize,
        switched: bool,
        switch_changes: usize,
        input: Entity<InputState>,
        textarea: Entity<TextareaState>,
    }

    impl Render for Controls {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("controls")
                .tab_group()
                .flex()
                .flex_col()
                .items_start()
                .gap_2()
                .child(Button::new("command").label("Open").disabled(self.disabled).on_click(
                    cx.listener(|this, _, _, cx| {
                        this.clicks += 1;
                        cx.notify();
                    }),
                ))
                .child(
                    Checkbox::new("checkbox")
                        .label("Include archived conversations")
                        .checked(self.checked)
                        .disabled(self.disabled)
                        .on_change(cx.listener(|this, value, _, cx| {
                            this.checked = *value;
                            this.checkbox_changes += 1;
                            cx.notify();
                        })),
                )
                .child(
                    Switch::new("switch")
                        .accessibility_label("Show previews")
                        .checked(self.switched)
                        .disabled(self.disabled)
                        .on_change(cx.listener(|this, value, _, cx| {
                            this.switched = *value;
                            this.switch_changes += 1;
                            cx.notify();
                        })),
                )
                .child(Input::new(&self.input).disabled(self.disabled).accessibility_label("Name"))
                .child(
                    Textarea::new(&self.textarea)
                        .appearance(false)
                        .disabled(self.disabled)
                        .accessibility_label("Message"),
                )
        }
    }

    fn controls(cx: &mut TestAppContext) -> (Entity<Controls>, &mut VisualTestContext) {
        cx.update(crate::init);
        let (view, cx) = cx.add_window_view(|window, cx| Controls {
            disabled: false,
            clicks: 0,
            checked: false,
            checkbox_changes: 0,
            switched: false,
            switch_changes: 0,
            input: cx.new(|cx| InputState::new(window, cx)),
            textarea: cx.new(|cx| TextareaState::new(window, cx)),
        });
        cx.update(|window, cx| {
            window.set_rem_size(px(14.));
            window.draw(cx).clear(cx);
        });
        (view, cx)
    }

    fn snapshot(
        cx: &mut VisualTestContext,
        id: impl Into<ElementId>,
    ) -> gpui_base::test_support::ElementSnapshot {
        let id = id.into();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            gpui_base::test_support::find(window, &[], &id).expect("rendered control")
        })
    }

    fn click(cx: &mut VisualTestContext, id: impl Into<ElementId>) {
        let bounds = snapshot(cx, id).bounds();
        cx.simulate_click(bounds.center(), Modifiers::default());
    }

    fn press(cx: &mut VisualTestContext, key: &str) {
        let keystroke = Keystroke::parse(key).unwrap();
        cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        cx.simulate_event(KeyUpEvent { keystroke });
    }

    #[gpui::test]
    fn controls_activate_from_labels_and_native_keyboard(cx: &mut TestAppContext) {
        let (view, cx) = controls(cx);
        click(cx, "command");
        press(cx, "enter");
        press(cx, "space");
        cx.update(|_, cx| assert_eq!(view.read(cx).clicks, 3));

        let checkbox = snapshot(cx, "checkbox");
        assert_eq!(checkbox.role(), Some(gpui::Role::CheckBox));
        assert_eq!(checkbox.label(), Some("Include archived conversations"));
        let bounds = checkbox.bounds();
        // Exercise the label's trailing edge, outside the indicator hit target.
        cx.simulate_click(point(bounds.right() - px(4.), bounds.center().y), Modifiers::default());
        assert_eq!(snapshot(cx, "checkbox").checked(), Some(true));
        press(cx, "space");
        assert_eq!(snapshot(cx, "checkbox").checked(), Some(false));
        press(cx, "enter");
        assert_eq!(snapshot(cx, "checkbox").checked(), Some(true));
        cx.update(|_, cx| assert_eq!(view.read(cx).checkbox_changes, 3));

        click(cx, "switch");
        assert_eq!(snapshot(cx, "switch").checked(), Some(true));
        press(cx, "space");
        assert_eq!(snapshot(cx, "switch").checked(), Some(false));
        cx.update(|_, cx| assert_eq!(view.read(cx).switch_changes, 2));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.disabled = true;
                cx.notify();
            })
        });
        for id in ["command", "checkbox", "switch"] {
            click(cx, id);
            press(cx, "enter");
            press(cx, "space");
        }
        cx.update(|_, cx| {
            let state = view.read(cx);
            assert_eq!((state.clicks, state.checkbox_changes, state.switch_changes), (3, 3, 2));
            assert!(state.checked);
            assert!(!state.switched);
        });
    }

    #[gpui::test]
    fn disabled_editors_keep_their_values(cx: &mut TestAppContext) {
        let (view, cx) = controls(cx);
        let (input, textarea) =
            cx.update(|_, cx| (view.read(cx).input.clone(), view.read(cx).textarea.clone()));
        let input_id = ("glyph-input", input.entity_id());
        let textarea_id = ("glyph-textarea", textarea.entity_id());
        click(cx, input_id);
        cx.simulate_input("Ada");
        click(cx, textarea_id);
        cx.simulate_input("Draft");
        cx.update(|_, cx| {
            assert_eq!(input.read(cx).value().as_ref(), "Ada");
            assert_eq!(textarea.read(cx).value().as_ref(), "Draft");
            view.update(cx, |view, cx| {
                view.disabled = true;
                cx.notify();
            });
        });

        click(cx, input_id);
        cx.simulate_input("discard");
        click(cx, textarea_id);
        cx.simulate_input("discard");
        cx.update(|_, cx| {
            assert_eq!(input.read(cx).value().as_ref(), "Ada");
            assert_eq!(textarea.read(cx).value().as_ref(), "Draft");
        });
    }

    #[gpui::test]
    fn controls_scale_with_the_window_rem(cx: &mut TestAppContext) {
        let (view, cx) = controls(cx);
        let input_id = cx.update(|_, cx| ("glyph-input", view.read(cx).input.entity_id()));
        assert_eq!(snapshot(cx, "command").bounds().size.height, px(36.));
        assert_eq!(snapshot(cx, "switch").bounds().size.height, px(20.));
        assert_eq!(snapshot(cx, input_id).bounds().size.height, px(36.));
        cx.update(|window, cx| {
            window.set_rem_size(px(21.));
            window.draw(cx).clear(cx);
        });
        assert_eq!(snapshot(cx, "command").bounds().size.height, px(54.));
        assert_eq!(snapshot(cx, "switch").bounds().size.height, px(30.));
        assert_eq!(snapshot(cx, input_id).bounds().size.height, px(54.));
    }
}
