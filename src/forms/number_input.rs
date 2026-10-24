use gpui::prelude::FluentBuilder;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;

/// A controlled numeric stepper. The owner applies values requested by `on_change`.
#[derive(IntoElement)]
pub struct NumberInput {
    id: ElementId,
    value: i64,
    min: Option<i64>,
    max: Option<i64>,
    step: i64,
    disabled: bool,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
    on_change: Option<Rc<dyn Fn(i64, &mut Window, &mut App) + 'static>>,
}

impl NumberInput {
    pub fn new(id: impl Into<ElementId>, value: i64) -> Self {
        Self {
            id: id.into(),
            value,
            min: None,
            max: None,
            step: 1,
            disabled: false,
            accessibility_label: None,
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets a positive step; nonpositive values use a step of one.
    pub fn step(mut self, step: i64) -> Self {
        self.step = step.max(1);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, callback: impl Fn(i64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }

    /// Names the field and its decrease/increase commands for accessibility clients.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    fn stepped_values(&self) -> (i64, i64) {
        let min = self.min.unwrap_or(i64::MIN);
        let max = self.max.unwrap_or(i64::MAX).max(min);
        (
            self.value.saturating_sub(self.step).clamp(min, max),
            self.value.saturating_add(self.step).clamp(min, max),
        )
    }
}

impl Styled for NumberInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NumberInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let (decreased, increased) = self.stepped_values();
        let can_decrement = !self.disabled && decreased < self.value;
        let can_increment = !self.disabled && increased > self.value;
        let decrease_id = ElementId::Name(format!("{}:decrease", self.id).into());
        let increase_id = ElementId::Name(format!("{}:increase", self.id).into());
        let decrease_label = self.accessibility_label.as_ref().map_or_else(
            || SharedString::from("Уменьшить значение"),
            |label| format!("Уменьшить: {label}").into(),
        );
        let increase_label = self.accessibility_label.as_ref().map_or_else(
            || SharedString::from("Увеличить значение"),
            |label| format!("Увеличить: {label}").into(),
        );
        let focus_ring = gpui::BoxShadow::new(px(0.), px(0.), theme.ring.opacity(0.24))
            .spread_radius(window.rem_size() * (2. / 14.));

        let on_change_dec = self.on_change.clone();
        let on_change_inc = self.on_change.clone();

        div()
            .id(self.id)
            .role(Role::Group)
            .when_some(self.accessibility_label, |this, label| {
                this.aria_label(label)
            })
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(rems(36. / 14.))
            .w(rems(140. / 14.))
            .px_2()
            .bg(theme.control_fill)
            .border_1()
            .border_color(gpui::transparent_black())
            .rounded(theme.radius_3xl)
            .text_size(gpui::rems(1.))
            .cursor_default()
            .when(self.disabled, |this| this.opacity(0.5))
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            // Decrement button
            .child(
                gpui_base::Button::new(decrease_id)
                    .accessibility_label(decrease_label)
                    .disabled(!can_decrement)
                    .size(rems(24. / 14.))
                    .flex_shrink_0()
                    .rounded(theme.radius_sm)
                    .cursor_default()
                    .when(can_decrement, |this| {
                        this.hover(|s| s.bg(theme.control_hover))
                            .focus_visible(|s| s.shadow(vec![focus_ring.clone()]))
                    })
                    .when(!can_decrement, |this| this.opacity(0.4))
                    .text_color(theme.foreground)
                    .on_click(move |_, window, cx| {
                        if let Some(callback) = &on_change_dec {
                            callback(decreased, window, cx);
                        }
                    })
                    .child(Icon::new(IconName::Minus).small()),
            )
            // Value display
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_center()
                    .font_medium()
                    .text_color(theme.foreground)
                    .child(self.value.to_string()),
            )
            // Increment button
            .child(
                gpui_base::Button::new(increase_id)
                    .accessibility_label(increase_label)
                    .disabled(!can_increment)
                    .size(rems(24. / 14.))
                    .flex_shrink_0()
                    .rounded(theme.radius_sm)
                    .cursor_default()
                    .when(can_increment, |this| {
                        this.hover(|s| s.bg(theme.control_hover))
                            .focus_visible(|s| s.shadow(vec![focus_ring]))
                    })
                    .when(!can_increment, |this| this.opacity(0.4))
                    .text_color(theme.foreground)
                    .on_click(move |_, window, cx| {
                        if let Some(callback) = &on_change_inc {
                            callback(increased, window, cx);
                        }
                    })
                    .child(Icon::new(IconName::Plus).small()),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        Context, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Render, TestAppContext,
        VisualTestContext,
    };
    use gpui_base::test_support::{ElementSnapshot, find};

    #[test]
    fn steps_clamp_at_bounds_and_saturate_at_integer_limits() {
        assert_eq!(
            NumberInput::new("stepper", 1)
                .min(0)
                .max(10)
                .step(4)
                .stepped_values(),
            (0, 5)
        );
        assert_eq!(
            NumberInput::new("stepper", 9)
                .min(0)
                .max(10)
                .step(4)
                .stepped_values(),
            (5, 10)
        );
        assert_eq!(
            NumberInput::new("stepper", i64::MIN).stepped_values().0,
            i64::MIN
        );
        assert_eq!(
            NumberInput::new("stepper", i64::MAX).stepped_values().1,
            i64::MAX
        );
    }

    struct Stepper {
        value: i64,
        changes: Vec<i64>,
        disabled: bool,
    }

    impl Render for Stepper {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let change = cx.listener(|this, value: &i64, _, cx| {
                this.value = *value;
                this.changes.push(*value);
                cx.notify();
            });
            div().p_4().child(
                NumberInput::new("volume", self.value)
                    .min(0)
                    .max(10)
                    .step(3)
                    .disabled(self.disabled)
                    .accessibility_label("Громкость")
                    .on_change(move |value, window, cx| change(&value, window, cx)),
            )
        }
    }

    fn snapshot(cx: &mut VisualTestContext, id: &str) -> ElementSnapshot {
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            find(window, &[], &ElementId::Name(id.to_owned().into())).expect("numeric command")
        })
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
    fn native_commands_support_keyboard_clamping_and_disabled_bounds(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (view, cx) = cx.add_window_view(|_, _| Stepper {
            value: 8,
            changes: Vec::new(),
            disabled: false,
        });
        cx.update(|window, _| window.set_rem_size(px(14.)));
        let increase = snapshot(cx, "volume:increase");
        assert_eq!(increase.role(), Some(Role::Button));
        assert_eq!(increase.label(), Some("Увеличить: Громкость"));
        cx.simulate_click(increase.bounds().center(), Modifiers::default());
        assert_eq!(snapshot(cx, "volume:increase").disabled(), Some(true));
        press(cx, "enter");
        cx.update(|_, cx| assert_eq!(view.read(cx).changes, vec![10]));

        let decrease = snapshot(cx, "volume:decrease");
        assert_eq!(decrease.label(), Some("Уменьшить: Громкость"));
        cx.simulate_click(decrease.bounds().center(), Modifiers::default());
        snapshot(cx, "volume:decrease");
        press(cx, "enter");
        snapshot(cx, "volume:decrease");
        press(cx, "space");
        snapshot(cx, "volume:decrease");
        press(cx, "space");
        assert_eq!(snapshot(cx, "volume:decrease").disabled(), Some(true));
        press(cx, "enter");
        cx.update(|_, cx| assert_eq!(view.read(cx).changes, vec![10, 7, 4, 1, 0]));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.disabled = true;
                cx.notify();
            });
        });
        let increase = snapshot(cx, "volume:increase");
        assert_eq!(increase.disabled(), Some(true));
        cx.simulate_click(increase.bounds().center(), Modifiers::default());
        press(cx, "space");
        cx.update(|_, cx| assert_eq!(view.read(cx).changes, vec![10, 7, 4, 1, 0]));
    }
}
