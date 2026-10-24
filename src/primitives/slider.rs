use gpui::prelude::FluentBuilder;
use gpui::{
    App, Axis, DefiniteLength, Entity, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, relative, rems,
};
pub use gpui_base::slider::{
    Slider as BaseSlider, SliderEvent, SliderScale, SliderState, SliderThumb, SliderTrack,
    SliderValue,
};

use crate::theme::ActiveTheme;

/// A slider input component matching the shadcn/ui design system.
#[derive(IntoElement)]
pub struct Slider {
    state: Entity<SliderState>,
    axis: Axis,
    style: StyleRefinement,
    disabled: bool,
    accessibility_label: Option<SharedString>,
    reverse: bool,
}

impl Slider {
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            axis: Axis::Horizontal,
            style: StyleRefinement::default(),
            disabled: false,
            accessibility_label: None,
            reverse: false,
        }
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.axis = Axis::Vertical;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let axis = self.axis;
        let is_horiz = matches!(axis, Axis::Horizontal);
        let is_vert = matches!(axis, Axis::Vertical);

        let state = self.state.read(cx);
        let is_range = state.value().is_range();
        let percentage = state.percentage();
        let (bar_start, bar_end) = if self.reverse && !is_range {
            (relative(percentage.end), relative(0.))
        } else {
            (relative(percentage.start), relative(1. - percentage.end))
        };

        let bar_color = theme.primary;
        let track_bg = theme.muted;
        let thumb_bg = theme.background;
        let thumb_border = theme.primary;

        let thumb = |position: DefiniteLength, start: bool| {
            SliderThumb::new(&self.state)
                .axis(axis)
                .start(start)
                .disabled(self.disabled)
                .when(!self.disabled, |this| {
                    this.absolute()
                        .when(is_horiz, |this| {
                            this.top(px(-5.)).left(position).ml(-px(8.))
                        })
                        .when(is_vert, |this| {
                            this.bottom(position).left(px(-5.)).mb(-px(8.))
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                })
                .child(
                    div()
                        .size(rems(16. / 14.))
                        .rounded_full()
                        .bg(thumb_bg)
                        .border_2()
                        .border_color(thumb_border)
                        .shadow(vec![
                            gpui::BoxShadow::new(px(0.), px(1.), gpui::hsla(0., 0., 0., 0.15))
                                .blur_radius(px(2.)),
                        ]),
                )
        };

        let start_thumb = is_range.then(|| thumb(relative(percentage.start), true));
        let end_thumb = thumb(relative(percentage.end), false);

        BaseSlider::new(&self.state)
            .axis(axis)
            .disabled(self.disabled)
            .when_some(self.accessibility_label, |this, label| {
                this.accessibility_label(label)
            })
            .when(is_horiz, |this| {
                this.h(rems(24. / 14.)).w_full().items_center()
            })
            .when(is_vert, |this| {
                this.w(rems(24. / 14.)).h_full().justify_center()
            })
            .flex()
            .relative()
            .when(!self.disabled, |this| this.cursor_pointer())
            .when(self.disabled, |this| this.opacity(0.5).cursor_not_allowed())
            .child(
                SliderTrack::new(&self.state)
                    .axis(axis)
                    .disabled(self.disabled)
                    .when(is_horiz, |this| this.h(rems(6. / 14.)).w_full())
                    .when(is_vert, |this| this.w(rems(6. / 14.)).h_full())
                    .relative()
                    .rounded_full()
                    .bg(track_bg)
                    .child(
                        div()
                            .absolute()
                            .rounded_full()
                            .bg(bar_color)
                            .when(is_horiz, |this| {
                                this.top_0().bottom_0().left(bar_start).right(bar_end)
                            })
                            .when(is_vert, |this| {
                                this.left_0().right_0().bottom(bar_start).top(bar_end)
                            }),
                    ),
            )
            .children(start_thumb)
            .child(end_thumb)
    }
}
