use std::ops::Range;
use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Context, ElementId, Entity, FollowMode, Hsla, IntoElement, ListAlignment,
    ListState, ParentElement, Refineable, RenderOnce, Role, SharedString, StyleRefinement, Styled,
    Window, div, linear_color_stop, linear_gradient, list, px, rems,
};

use crate::{Button, IconName};

const LIST_OVERDRAW: gpui::Pixels = px(400.);

pub struct MessageScrollerState {
    list_state: ListState,
    measured_rem: Option<gpui::Pixels>,
}

impl MessageScrollerState {
    pub fn new(item_count: usize, cx: &mut Context<Self>) -> Self {
        let list_state = ListState::new(item_count, ListAlignment::Top, LIST_OVERDRAW);
        list_state.set_follow_mode(FollowMode::Tail);

        let weak_state = cx.weak_entity();
        list_state.set_scroll_handler(move |_, _, cx| {
            let weak_state = weak_state.clone();
            cx.defer(move |cx| {
                let _ = weak_state.update(cx, |_, cx| cx.notify());
            });
        });

        Self { list_state, measured_rem: None }
    }

    pub fn item_count(&self) -> usize {
        self.list_state.item_count()
    }

    pub fn is_scrolled_up(&self) -> bool {
        self.list_state.max_offset_for_scrollbar().y > px(0.)
            && !self.list_state.is_following_tail()
            && !self.list_state.is_scrolled_to_end().unwrap_or(false)
    }

    pub fn is_following_tail(&self) -> bool {
        self.list_state.is_following_tail()
    }

    pub fn reset(&mut self, item_count: usize, cx: &mut Context<Self>) {
        self.list_state.reset(item_count);
        self.list_state.set_follow_mode(FollowMode::Tail);
        cx.notify();
    }

    pub fn splice(
        &mut self,
        old_range: Range<usize>,
        count: usize,
        cx: &mut Context<Self>,
    ) -> bool {
        let item_count = self.list_state.item_count();
        if old_range.start > item_count || old_range.end > item_count {
            return false;
        }

        let neighbor = old_range.start.checked_sub(1);
        self.list_state.splice(old_range, count);

        if let Some(last) = self.list_state.item_count().checked_sub(1) {
            self.list_state.remeasure_items(last..last + 1);
            if let Some(neighbor) = neighbor.filter(|n| *n != last) {
                self.list_state.remeasure_items(neighbor..neighbor + 1);
            }
        }

        cx.notify();
        true
    }

    pub fn append(&mut self, count: usize, cx: &mut Context<Self>) -> bool {
        let item_count = self.list_state.item_count();
        self.splice(item_count..item_count, count, cx)
    }

    pub fn prepend(&mut self, count: usize, cx: &mut Context<Self>) -> bool {
        self.splice(0..0, count, cx)
    }

    pub fn remeasure(&mut self, cx: &mut Context<Self>) {
        let item_count = self.list_state.item_count();
        if item_count > 0 {
            self.list_state.remeasure_items(0..item_count);
            cx.notify();
        }
    }

    pub fn remeasure_items(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        self.list_state.remeasure_items(range);
        cx.notify();
    }

    pub fn scroll_to_bottom(&mut self, cx: &mut Context<Self>) {
        self.list_state.set_follow_mode(FollowMode::Tail);
        self.list_state.scroll_to_end();
        cx.notify();
    }

    pub fn scroll_to_end(&mut self, cx: &mut Context<Self>) {
        self.scroll_to_bottom(cx);
    }
}

pub type MessageScrollerItemRenderer =
    Rc<dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static>;

#[derive(IntoElement)]
pub struct MessageScroller {
    id: ElementId,
    state: Entity<MessageScrollerState>,
    item_renderer: MessageScrollerItemRenderer,
    jump_button: bool,
    jump_button_label: Option<SharedString>,
    bottom_fade: Option<Hsla>,
    row_style: StyleRefinement,
    style: StyleRefinement,
}

impl MessageScroller {
    pub fn new(
        id: impl Into<ElementId>,
        state: Entity<MessageScrollerState>,
        item_renderer: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            state,
            item_renderer: Rc::new(item_renderer),
            jump_button: true,
            jump_button_label: None,
            bottom_fade: None,
            row_style: StyleRefinement::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn jump_button(mut self, jump: bool) -> Self {
        self.jump_button = jump;
        self
    }

    pub fn with_jump_button_label(mut self, label: impl Into<SharedString>) -> Self {
        self.jump_button_label = Some(label.into());
        self
    }

    pub fn with_bottom_fade(mut self, color: impl Into<Hsla>) -> Self {
        self.bottom_fade = Some(color.into());
        self
    }

    /// Refine the full-width wrapper measured around every virtualized row.
    pub fn with_row_style(mut self, style: StyleRefinement) -> Self {
        self.row_style = style;
        self
    }
}

impl Styled for MessageScroller {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MessageScroller {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.clone();
        let renderer = self.item_renderer.clone();
        let row_style = self.row_style;
        let root_id = self.id.clone();
        let (list_state, scrolled_up) = state.update(cx, |state, _| {
            // GPUI remeasures for width changes; rem can change at the same width.
            // Invalidate that geometry before this frame measures any virtual rows.
            let rem = window.rem_size();
            if state.measured_rem != Some(rem) {
                state.list_state.remeasure();
                state.measured_rem = Some(rem);
            }
            (state.list_state.clone(), state.is_scrolled_up())
        });
        let viewport = div()
            .id((root_id.clone(), "viewport"))
            .role(Role::Log)
            .size_full()
            .min_w_0()
            .min_h_0()
            .child(
                list(list_state, move |index, window, cx| {
                    div()
                        .w_full()
                        .min_w_0()
                        .map(|mut row| {
                            row.style().refine(&row_style);
                            row
                        })
                        .child(renderer(index, window, cx))
                        .into_any_element()
                })
                .size_full()
                .min_h_0(),
            )
            .when_some(self.bottom_fade.filter(|_| scrolled_up), |viewport, color| {
                viewport.child(div().absolute().left_0().right_0().bottom_0().h(rems(2.)).bg(
                    linear_gradient(
                        180.,
                        linear_color_stop(color.opacity(0.), 0.),
                        linear_color_stop(color, 1.),
                    ),
                ))
            });

        div()
            .id(root_id.clone())
            .size_full()
            .min_w_0()
            .min_h_0()
            .relative()
            .overflow_hidden()
            .map(|mut el| {
                el.style().refine(&self.style);
                el
            })
            .child(viewport)
            .when(self.jump_button && scrolled_up, |root| {
                let label =
                    self.jump_button_label.unwrap_or_else(|| "К последним сообщениям".into());
                root.child(
                    div().absolute().left_0().right_0().bottom_3().flex().justify_center().child(
                        Button::new((root_id, "jump-to-latest"))
                            .secondary()
                            .icon(IconName::ArrowDown)
                            .pill()
                            .accessibility_label(label.clone())
                            .tooltip(label)
                            .on_click(move |_, _, cx| {
                                state.update(cx, |state, cx| state.scroll_to_bottom(cx));
                            }),
                    ),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        AppContext, Modifiers, Render, ScrollDelta, ScrollWheelEvent, Subscription, TestAppContext,
        VisualTestContext, point, size,
    };
    use gpui_base::TestSupportExt;

    struct Transcript {
        state: Entity<MessageScrollerState>,
        _subscription: Subscription,
    }

    impl Render for Transcript {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            MessageScroller::new("transcript", self.state.clone(), |index, _, _| {
                div()
                    .id(("message", index))
                    .test_support()
                    .w_full()
                    .min_w_0()
                    .text_base()
                    .child("A long message keeps its content inset while wrapping across the available conversation width. ".repeat(4))
                    .into_any_element()
            })
            .with_row_style(StyleRefinement::default().px_4().pb_2())
            .with_jump_button_label("Latest messages")
            .with_bottom_fade(gpui::white())
        }
    }

    fn transcript(cx: &mut TestAppContext) -> (Entity<Transcript>, &mut VisualTestContext) {
        cx.update(crate::init);
        let (view, cx) = cx.add_window_view(|_, cx| {
            let state = cx.new(|cx| MessageScrollerState::new(24, cx));
            let subscription = cx.observe(&state, |_, _, cx| cx.notify());
            Transcript { state, _subscription: subscription }
        });
        cx.simulate_resize(size(px(420.), px(260.)));
        cx.update(|window, cx| {
            window.set_rem_size(px(14.));
            window.draw(cx).clear(cx);
        });
        (view, cx)
    }

    fn message_bounds(cx: &mut VisualTestContext) -> gpui::Bounds<gpui::Pixels> {
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            gpui_base::test_support::find(window, &[], &("message", 23usize).into())
                .expect("last message remains rendered at the live edge")
                .bounds()
        })
    }

    #[gpui::test]
    fn row_insets_wrap_long_messages_at_zoom_and_narrow_width(cx: &mut TestAppContext) {
        let (_, cx) = transcript(cx);
        let normal = message_bounds(cx);
        assert_eq!(normal.left(), px(14.));
        assert_eq!(normal.right(), px(406.));

        cx.update(|window, cx| {
            window.set_rem_size(px(21.));
            window.draw(cx).clear(cx);
        });
        let zoomed = message_bounds(cx);
        assert_eq!(zoomed.left(), px(21.));
        assert_eq!(zoomed.right(), px(399.));
        assert!(zoomed.size.height > normal.size.height, "zoom must remeasure wrapped rows");

        cx.simulate_resize(size(px(280.), px(260.)));
        let narrow = message_bounds(cx);
        assert_eq!(narrow.left(), px(21.));
        assert_eq!(narrow.right(), px(259.));
        assert!(narrow.size.height > zoomed.size.height);
    }

    #[gpui::test]
    fn scrolling_reveals_jump_button_and_activation_restores_tail(cx: &mut TestAppContext) {
        let (view, cx) = transcript(cx);
        let jump_id: ElementId = (ElementId::from("transcript"), "jump-to-latest").into();
        cx.update(|window, _| {
            assert!(gpui_base::test_support::find(window, &[], &jump_id).is_none());
        });
        cx.simulate_event(ScrollWheelEvent {
            position: point(px(150.), px(130.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(180.))),
            ..Default::default()
        });
        let button = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(view.read(cx).state.read(cx).is_scrolled_up());
            gpui_base::test_support::find(window, &[], &jump_id)
                .expect("jump command after scrolling")
        });
        assert_eq!(button.label(), Some("Latest messages"));
        cx.simulate_click(button.bounds().center(), Modifiers::default());
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(view.read(cx).state.read(cx).is_following_tail());
            assert!(gpui_base::test_support::find(window, &[], &jump_id).is_none());
        });
    }
}
