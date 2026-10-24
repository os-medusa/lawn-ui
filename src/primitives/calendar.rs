use gpui::prelude::*;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, rems,
};
use gpui_base::StyledExt;
use std::rc::Rc;

use crate::icon::{Icon, IconName, IconSize};
use crate::theme::ActiveTheme;

/// A monthly calendar grid component adhering to shadcn/ui.
#[derive(IntoElement)]
pub struct Calendar {
    id: Option<ElementId>,
    year: i32,
    month: u32,
    selected_day: Option<u32>,
    style: StyleRefinement,
    on_select_date: Option<Rc<dyn Fn(i32, u32, u32, &mut Window, &mut App) + 'static>>,
    on_prev_month: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_next_month: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Calendar {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            id: None,
            year,
            month: month.clamp(1, 12),
            selected_day: None,
            style: StyleRefinement::default(),
            on_select_date: None,
            on_prev_month: None,
            on_next_month: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn selected_day(mut self, day: Option<u32>) -> Self {
        self.selected_day = day;
        self
    }

    pub fn on_select_date(
        mut self,
        handler: impl Fn(i32, u32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select_date = Some(Rc::new(handler));
        self
    }

    pub fn on_prev_month(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_prev_month = Some(Rc::new(handler));
        self
    }

    pub fn on_next_month(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_next_month = Some(Rc::new(handler));
        self
    }
}

impl Styled for Calendar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Январь",
        2 => "Февраль",
        3 => "Март",
        4 => "Апрель",
        5 => "Май",
        6 => "Июнь",
        7 => "Июль",
        8 => "Август",
        9 => "Сентябрь",
        10 => "Октябрь",
        11 => "Ноябрь",
        12 => "Декабрь",
        _ => "",
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn first_weekday_of_month(year: i32, month: u32) -> u32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    let dow = (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + 1) % 7;
    ((dow + 6) % 7) as u32
}

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let year = self.year;
        let month = self.month;
        let selected_day = self.selected_day;
        let on_select = self.on_select_date.clone();

        let id = self.id.unwrap_or_else(|| ElementId::Integer(0));

        let mut container = div()
            .id(id)
            .w(rems(320. / 14.))
            .p(rems(16. / 14.))
            .rounded(theme.radius_2xl)
            .border_1()
            .border_color(theme.border)
            .bg(theme.card)
            .flex()
            .flex_col()
            .gap(rems(16. / 14.));

        container.style().refine(&self.style);

        // Header: Month Year + navigation buttons
        let title = format!("{} {}", month_name(month), year);
        let on_prev = self.on_prev_month.clone();
        let on_next = self.on_next_month.clone();

        let mut prev_btn = div()
            .id("calendar-prev-btn")
            .size(rems(28. / 14.))
            .rounded(theme.radius_sm)
            .border_1()
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(theme.control_hover))
            .child(Icon::new(IconName::ChevronLeft).size(IconSize::Xs));

        if let Some(on_prev) = on_prev {
            prev_btn = prev_btn.on_click(move |_, window, cx| (on_prev)(window, cx));
        }

        let mut next_btn = div()
            .id("calendar-next-btn")
            .size(rems(28. / 14.))
            .rounded(theme.radius_sm)
            .border_1()
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(theme.control_hover))
            .child(Icon::new(IconName::ChevronRight).size(IconSize::Xs));

        if let Some(on_next) = on_next {
            next_btn = next_btn.on_click(move |_, window, cx| (on_next)(window, cx));
        }

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_size(gpui::rems(1.))
                    .font_semibold()
                    .text_color(theme.foreground)
                    .child(title),
            )
            .child(div().flex().items_center().gap_1().child(prev_btn).child(next_btn));

        container = container.child(header);

        // Day of week headers
        let week_headers = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"];
        let mut days_header = div().flex().w_full().justify_between();
        for day_name in week_headers {
            days_header = days_header.child(
                div()
                    .size(rems(36. / 14.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(gpui::rems(12. / 14.))
                    .font_medium()
                    .text_color(theme.muted_foreground)
                    .child(day_name),
            );
        }
        container = container.child(days_header);

        // Days grid
        let num_days = days_in_month(year, month);
        let first_weekday = first_weekday_of_month(year, month);

        let mut weeks_col = div().flex().flex_col().gap_1();
        let mut current_week = div().flex().w_full().justify_between();
        let mut day_counter = 1;

        for slot in 0..42 {
            let col = slot % 7;
            if slot < first_weekday || day_counter > num_days {
                current_week = current_week.child(div().size(rems(36. / 14.)));
            } else {
                let day = day_counter;
                let is_selected = selected_day == Some(day);

                let mut day_cell = div()
                    .id(("calendar-day", day))
                    .size(rems(36. / 14.))
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(gpui::rems(1.))
                    .cursor_pointer()
                    .when(is_selected, |this| {
                        this.bg(theme.primary).text_color(theme.primary_foreground)
                    })
                    .when(!is_selected, |this| {
                        this.text_color(theme.foreground)
                            .hover(|s| s.bg(theme.control_hover))
                    })
                    .child(day.to_string());

                if let Some(on_select) = on_select.clone() {
                    day_cell = day_cell.on_click(move |_, window, cx| {
                        (on_select)(year, month, day, window, cx);
                    });
                }

                current_week = current_week.child(day_cell);
                day_counter += 1;
            }

            if col == 6 {
                weeks_col = weeks_col.child(current_week);
                current_week = div().flex().w_full().justify_between();
                if day_counter > num_days {
                    break;
                }
            }
        }

        container.child(weeks_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_days_and_leap_year() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2026, 10), 31);
    }
}
