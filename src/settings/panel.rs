use crate::forms::{Input, InputEvent, InputState, NumberInput, Select};
use crate::switch::Switch;
use crate::{ActiveTheme, Icon, IconName, StyledExt, button::Button, h_flex, v_flex};
use gpui::{
    AnyElement, App, AppContext, ElementId, Entity, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Subscription, Window, div, prelude::*, rems,
};
use gpui_base::TestSupportExt;

use super::{SettingField, SettingFieldKind, SettingItem, SettingPage, Settings};

struct SettingsSearch {
    input: Entity<InputState>,
    selected_page: usize,
    _subscription: Subscription,
}

struct SettingsInput {
    input: Entity<InputState>,
    _subscription: Subscription,
}

impl Settings {
    fn render_field(
        field: &SettingField,
        scope: &str,
        id: &str,
        label: SharedString,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        match &field.kind {
            SettingFieldKind::Switch { get, set } => {
                let set = set.clone();
                Switch::new(SharedString::from(id.to_owned()))
                    .accessibility_label(label)
                    .checked(get(cx))
                    .on_click(move |value, _, cx| set(*value, cx))
                    .into_any_element()
            }
            SettingFieldKind::Dropdown {
                options, get, set, ..
            } => {
                let set = set.clone();
                Select::new(SharedString::from(id.to_owned()))
                    .placeholder(label.clone())
                    .accessibility_label(label)
                    .selected_value(Some(get(cx)))
                    .items(options.iter().map(|(value, label)| {
                        crate::forms::SelectItem::new(value.clone(), label.clone())
                    }))
                    .on_change(move |value, _, cx| set(value, cx))
                    .into_any_element()
            }
            SettingFieldKind::Number { options, get, set } => {
                let value = get(cx);
                if value.fract() != 0.
                    || options.step.fract() != 0.
                    || options.min.fract() != 0.
                    || options.max.fract() != 0.
                {
                    let decrease = set.clone();
                    let increase = set.clone();
                    let options = *options;
                    return h_flex()
                        .w_full()
                        .h(rems(36. / 14.))
                        .px_2()
                        .justify_between()
                        .rounded(cx.theme().radius_3xl)
                        .bg(cx.theme().control_fill)
                        .child(
                            Button::new(SharedString::from(format!("{id}-decrease")))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Minus)
                                .accessibility_label(format!("Уменьшить: {label}"))
                                .disabled(value <= options.min)
                                .on_click(move |_, _, cx| {
                                    decrease(
                                        (value - options.step).clamp(options.min, options.max),
                                        cx,
                                    )
                                }),
                        )
                        .child(div().text_base().child(value.to_string()))
                        .child(
                            Button::new(SharedString::from(format!("{id}-increase")))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Plus)
                                .accessibility_label(format!("Увеличить: {label}"))
                                .disabled(value >= options.max)
                                .on_click(move |_, _, cx| {
                                    increase(
                                        (value + options.step).clamp(options.min, options.max),
                                        cx,
                                    )
                                }),
                        )
                        .into_any_element();
                }
                let set = set.clone();
                NumberInput::new(SharedString::from(id.to_owned()), get(cx).round() as i64)
                    .accessibility_label(label)
                    .w_full()
                    .min(options.min as i64)
                    .max(options.max as i64)
                    .step(options.step.max(1.) as i64)
                    .on_change(move |value, _, cx| set(value as f64, cx))
                    .into_any_element()
            }
            SettingFieldKind::Input { get, set } => {
                let value = get(cx);
                let initial_value = value.clone();
                let getter = get.clone();
                let setter = set.clone();
                let state = window.use_keyed_state(
                    SharedString::from(format!("{scope}:{id}-state")),
                    cx,
                    move |window, cx| {
                        let input =
                            cx.new(|cx| InputState::new(window, cx).default_value(initial_value));
                        let subscription =
                            cx.subscribe(&input, move |_, input, event: &InputEvent, cx| {
                                let next = input.read(cx).value();
                                if matches!(event, InputEvent::Change) && getter(cx) != next {
                                    setter(next, cx);
                                }
                            });
                        SettingsInput {
                            input,
                            _subscription: subscription,
                        }
                    },
                );
                let input = state.read(cx).input.clone();
                if input.read(cx).value() != value {
                    input.update(cx, |input, cx| input.set_value(value, window, cx));
                }
                Input::new(&input)
                    .accessibility_label(label)
                    .into_any_element()
            }
            SettingFieldKind::Custom(render) => render(window, cx),
        }
    }

    fn render_item(
        item: &SettingItem,
        scope: &str,
        id: &str,
        stacked: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let field = Self::render_field(&item.field, scope, id, item.label.clone(), window, cx);
        // Custom panels own the whole row, including their internal label and controls.
        if item.label.is_empty() {
            return div()
                .id(SharedString::from(format!("{id}-row")))
                .test_support()
                .w_full()
                .min_w_0()
                .py_4()
                .child(field)
                .into_any_element();
        }

        let is_switch = matches!(item.field.kind, SettingFieldKind::Switch { .. });
        let field_width = if is_switch {
            rems(44. / 14.)
        } else {
            rems(280. / 14.)
        };
        div()
            .id(SharedString::from(format!("{id}-row")))
            .test_support()
            .w_full()
            .min_w_0()
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .py_4()
            .when(stacked && !is_switch, |row| row.flex_col().items_stretch())
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_1()
                    .child(
                        div()
                            .text_base()
                            .text_color(cx.theme().foreground)
                            .child(item.label.clone()),
                    )
                    .when_some(item.description.clone(), |column, description| {
                        column.child(
                            div()
                                .text_size(rems(12. / 14.))
                                .text_color(cx.theme().muted_foreground)
                                .child(description),
                        )
                    }),
            )
            .child(
                div()
                    .min_w_0()
                    .flex_shrink_0()
                    .w(field_width)
                    .when(stacked && !is_switch, |control| control.w_full())
                    .child(field),
            )
            .into_any_element()
    }

    fn render_page(
        page: SettingPage,
        scope: &str,
        page_ix: usize,
        query: &str,
        stacked: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let page_matches = query.is_empty() || page_matches(&page, query);
        let groups = page
            .groups
            .into_iter()
            .enumerate()
            .filter_map(|(group_ix, group)| {
                let show_all = page_matches || group_matches(&group, query);
                let items: Vec<_> = group
                    .items
                    .into_iter()
                    .enumerate()
                    .filter(|(_, item)| show_all || item_matches(item, query))
                    .collect();
                if items.is_empty() {
                    return None;
                }
                let mut rows = Vec::new();
                for (item_ix, item) in items {
                    let id = format!("settings-field-{page_ix}-{group_ix}-{item_ix}");
                    rows.push(Self::render_item(&item, scope, &id, stacked, window, cx));
                }
                Some(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .when_some(group.title, |section, title| {
                            section.child(
                                div()
                                    .text_size(rems(12. / 14.))
                                    .font_semibold()
                                    .text_color(cx.theme().muted_foreground)
                                    .pb_2()
                                    .child(title),
                            )
                        })
                        .when_some(group.description, |section, description| {
                            section.child(
                                div()
                                    .text_size(rems(12. / 14.))
                                    .text_color(cx.theme().muted_foreground)
                                    .pb_2()
                                    .child(description),
                            )
                        })
                        .children(rows.into_iter().map(|row| {
                            div()
                                .w_full()
                                .border_b_1()
                                .border_color(cx.theme().card_outline)
                                .child(row)
                        }))
                        .into_any_element(),
                )
            })
            .collect::<Vec<_>>();
        if groups.is_empty() {
            return div().into_any_element();
        }
        v_flex()
            .w_full()
            .min_w_0()
            .gap_6()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_size(rems(24. / 14.))
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(page.title),
                    )
                    .when_some(page.description, |header, description| {
                        header.child(
                            div()
                                .text_base()
                                .text_color(cx.theme().muted_foreground)
                                .child(description),
                        )
                    }),
            )
            .children(groups)
            .into_any_element()
    }
}

impl RenderOnce for Settings {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default_selected_page = self.selected_index.page_ix;
        let search_state = window.use_keyed_state(
            SharedString::from(format!("{}:search", self.id)),
            cx,
            |window, cx| {
                let input =
                    cx.new(|cx| InputState::new(window, cx).placeholder("Найти настройку…"));
                let subscription = cx.subscribe(&input, |_, _, event: &InputEvent, cx| {
                    if matches!(event, InputEvent::Change) {
                        cx.notify();
                    }
                });
                SettingsSearch {
                    input,
                    selected_page: default_selected_page,
                    _subscription: subscription,
                }
            },
        );
        let search = search_state.read(cx).input.clone();
        let query = search.read(cx).value().trim().to_lowercase();
        let compact = window.viewport_size().width < window.rem_size() * (900. / 14.);
        let selected_page = if self.on_select_page.is_some() {
            self.selected_index.page_ix
        } else {
            search_state.read(cx).selected_page
        };
        let selected_page = selected_page.min(self.pages.len().saturating_sub(1));
        let nav = self
            .pages
            .iter()
            .enumerate()
            .map(|(ix, page)| {
                let select = self.on_select_page.clone();
                let state = search_state.clone();
                let search = search.clone();
                Button::new(SharedString::from(format!("settings-nav-{ix}")))
                    .ghost()
                    .w_full()
                    .justify_start()
                    .selected(ix == selected_page)
                    .label(page.title.clone())
                    .when_some(page.icon, |button, icon| button.icon(icon))
                    .on_click(move |_, window, cx| {
                        search.update(cx, |input, cx| input.set_value("", window, cx));
                        state.update(cx, |state, cx| {
                            state.selected_page = ix;
                            cx.notify();
                        });
                        if let Some(select) = &select {
                            select(ix, window, cx);
                        }
                    })
            })
            .collect::<Vec<_>>();
        let mut sidebar = v_flex()
            .gap_6()
            .p_4()
            .w(self.sidebar_width)
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar_background)
            .border_r_1()
            .border_color(cx.theme().card_outline)
            .child(
                Input::new(&search)
                    .prefix(Icon::new(IconName::Search).small())
                    .cleanable(true)
                    .accessibility_label("Поиск настроек"),
            );
        if compact {
            let select = self.on_select_page.clone();
            let state = search_state.clone();
            let search = search.clone();
            sidebar = sidebar
                .w_full()
                .h_auto()
                .p_4()
                .gap_3()
                .border_r_0()
                .border_b_1()
                .child(
                    Select::new("settings-page-select")
                        .placeholder("Раздел настроек")
                        .accessibility_label("Раздел настроек")
                        .selected_value(Some(selected_page.to_string()))
                        .items(self.pages.iter().enumerate().map(|(ix, page)| {
                            crate::forms::SelectItem::new(ix.to_string(), page.title.clone())
                        }))
                        .on_change(move |value, window, cx| {
                            if let Ok(ix) = value.parse() {
                                search.update(cx, |input, cx| input.set_value("", window, cx));
                                state.update(cx, |state, cx| {
                                    state.selected_page = ix;
                                    cx.notify();
                                });
                                if let Some(select) = &select {
                                    select(ix, window, cx);
                                }
                            }
                        }),
                );
        } else {
            sidebar = sidebar.child(
                div()
                    .id("settings-navigation-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p_1()
                    .child(v_flex().gap_1().children(nav)),
            );
        }

        let has_matches = self.pages.iter().any(|page| {
            page_matches(page, &query)
                || page.groups.iter().any(|group| {
                    group_matches(group, &query)
                        || group.items.iter().any(|item| item_matches(item, &query))
                })
        });
        let scope = self.id.to_string();
        let pages = self
            .pages
            .into_iter()
            .enumerate()
            .filter(|(ix, page)| {
                if query.is_empty() {
                    *ix == selected_page
                } else {
                    page_has_matches(page, &query)
                }
            })
            .map(|(ix, page)| Self::render_page(page, &scope, ix, &query, compact, window, cx))
            .collect::<Vec<_>>();
        let content = v_flex()
            .w_full()
            .min_w_0()
            .gap_8()
            .p_6()
            .children(pages)
            .when(!has_matches, |content| {
                content.child(
                    v_flex()
                        .gap_2()
                        .py_6()
                        .child(div().text_base().child("Настройки не найдены"))
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child("Попробуйте другое слово или очистите поиск."),
                        ),
                )
            });
        div()
            .id(self.id)
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .items_stretch()
            .when(compact, |panel| panel.flex_col())
            .child(sidebar)
            .child(
                div()
                    .id(ElementId::Name(
                        format!("settings-scroll-{selected_page}").into(),
                    ))
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(content),
            )
    }
}

fn item_matches(item: &SettingItem, query: &str) -> bool {
    item.label.to_lowercase().contains(query)
        || item
            .description
            .as_ref()
            .is_some_and(|text| text.to_lowercase().contains(query))
        || item
            .keywords
            .iter()
            .any(|word| word.to_lowercase().contains(query))
}
fn group_matches(group: &super::SettingGroup, query: &str) -> bool {
    group
        .title
        .as_ref()
        .is_some_and(|text| text.to_lowercase().contains(query))
        || group
            .description
            .as_ref()
            .is_some_and(|text| text.to_lowercase().contains(query))
}
fn page_matches(page: &SettingPage, query: &str) -> bool {
    page.title.to_lowercase().contains(query)
        || page
            .description
            .as_ref()
            .is_some_and(|text| text.to_lowercase().contains(query))
}

fn page_has_matches(page: &SettingPage, query: &str) -> bool {
    page_matches(page, query)
        || page.groups.iter().any(|group| {
            group_matches(group, query) || group.items.iter().any(|item| item_matches(item, query))
        })
}
