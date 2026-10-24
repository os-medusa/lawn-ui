use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, AppContext, ElementId, Entity, IntoElement, ParentElement, Pixels, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Subscription, Window, div, px,
};

use crate::forms::{Input, InputEvent, InputState};
use crate::icon::{Icon, IconName};
use crate::primitives::button::Button;
use crate::primitives::separator::Separator;
use crate::primitives::switch::Switch;
use crate::theme::ActiveTheme;

struct SettingInputState {
    input: Entity<InputState>,
    _subscription: Subscription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectIndex {
    pub page_ix: usize,
    pub group_ix: Option<usize>,
}

pub struct RenderOptions;

#[derive(Clone, Copy, Debug)]
pub struct NumberFieldOptions {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl Default for NumberFieldOptions {
    fn default() -> Self {
        Self { min: f64::MIN, max: f64::MAX, step: 1.0 }
    }
}

pub enum SettingFieldKind {
    Switch {
        get: Rc<dyn Fn(&App) -> bool>,
        set: Rc<dyn Fn(bool, &mut App)>,
    },
    Dropdown {
        options: Vec<(SharedString, SharedString)>,
        get: Rc<dyn Fn(&App) -> SharedString>,
        set: Rc<dyn Fn(SharedString, &mut App)>,
        scrollable: bool,
    },
    Number {
        options: NumberFieldOptions,
        get: Rc<dyn Fn(&App) -> f64>,
        set: Rc<dyn Fn(f64, &mut App)>,
    },
    Input {
        get: Rc<dyn Fn(&App) -> SharedString>,
        set: Rc<dyn Fn(SharedString, &mut App)>,
    },
    Custom(Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>),
}

pub struct SettingField {
    kind: SettingFieldKind,
    default_value: Option<SharedString>,
}

impl SettingField {
    pub fn switch<G, S>(get: G, set: S) -> Self
    where
        G: Fn(&App) -> bool + 'static,
        S: Fn(bool, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Switch { get: Rc::new(get), set: Rc::new(set) },
            default_value: None,
        }
    }

    pub fn dropdown<G, S>(options: Vec<(SharedString, SharedString)>, get: G, set: S) -> Self
    where
        G: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Dropdown {
                options,
                get: Rc::new(get),
                set: Rc::new(set),
                scrollable: false,
            },
            default_value: None,
        }
    }

    pub fn scrollable_dropdown<G, S>(
        options: Vec<(SharedString, SharedString)>,
        get: G,
        set: S,
    ) -> Self
    where
        G: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Dropdown {
                options,
                get: Rc::new(get),
                set: Rc::new(set),
                scrollable: true,
            },
            default_value: None,
        }
    }

    pub fn number_input<G, S>(options: NumberFieldOptions, get: G, set: S) -> Self
    where
        G: Fn(&App) -> f64 + 'static,
        S: Fn(f64, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Number { options, get: Rc::new(get), set: Rc::new(set) },
            default_value: None,
        }
    }

    pub fn input<G, S>(get: G, set: S) -> Self
    where
        G: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Input { get: Rc::new(get), set: Rc::new(set) },
            default_value: None,
        }
    }

    pub fn render<F, E>(f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&RenderOptions, &mut Window, &mut App) -> E + 'static,
    {
        Self {
            kind: SettingFieldKind::Custom(Rc::new(move |w, cx| {
                f(&RenderOptions, w, cx).into_any_element()
            })),
            default_value: None,
        }
    }

    pub fn default_value(mut self, val: impl ToString) -> Self {
        self.default_value = Some(val.to_string().into());
        self
    }
}

pub struct SettingItem {
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub field: SettingField,
    pub keywords: Vec<SharedString>,
}

impl SettingItem {
    pub fn new(label: impl Into<SharedString>, field: SettingField) -> Self {
        Self { label: label.into(), description: None, field, keywords: Vec::new() }
    }

    pub fn render<F, E>(f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&RenderOptions, &mut Window, &mut App) -> E + 'static,
    {
        Self {
            label: "".into(),
            description: None,
            field: SettingField::render(f),
            keywords: Vec::new(),
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn keywords<I, S>(mut self, keywords: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SharedString>,
    {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }
}

pub struct SettingGroup {
    pub title: Option<SharedString>,
    pub description: Option<SharedString>,
    pub items: Vec<SettingItem>,
}

impl Default for SettingGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingGroup {
    pub fn new() -> Self {
        Self { title: None, description: None, items: Vec::new() }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn item(mut self, item: SettingItem) -> Self {
        self.items.push(item);
        self
    }
}

pub struct SettingPage {
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub description: Option<SharedString>,
    pub default_open: bool,
    pub groups: Vec<SettingGroup>,
}

impl SettingPage {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            description: None,
            default_open: true,
            groups: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn group(mut self, group: SettingGroup) -> Self {
        self.groups.push(group);
        self
    }
}

#[derive(IntoElement)]
pub struct Settings {
    id: ElementId,
    sidebar_width: Pixels,
    selected_index: SelectIndex,
    pages: Vec<SettingPage>,
    on_select_page: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Settings {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            sidebar_width: px(220.),
            selected_index: SelectIndex::default(),
            pages: Vec::new(),
            on_select_page: None,
        }
    }

    pub fn on_select_page<F>(mut self, on_select: F) -> Self
    where
        F: Fn(usize, &mut Window, &mut App) + 'static,
    {
        self.on_select_page = Some(Rc::new(on_select));
        self
    }

    pub fn sidebar_width(mut self, width: Pixels) -> Self {
        self.sidebar_width = width;
        self
    }

    pub fn default_selected_index(mut self, index: SelectIndex) -> Self {
        self.selected_index = index;
        self
    }

    pub fn page(mut self, page: SettingPage) -> Self {
        self.pages.push(page);
        self
    }
}

impl RenderOnce for Settings {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme().clone();
        let selected_page_ix = self.selected_index.page_ix.min(self.pages.len().saturating_sub(1));

        // Sidebar navigation items
        let on_select_page = self.on_select_page.clone();
        let nav_items = self.pages.iter().enumerate().map(|(ix, p)| {
            let is_selected = ix == selected_page_ix;
            let title = p.title.clone();
            let icon = p.icon;
            let on_select = on_select_page.clone();

            gpui_base::Button::new(format!("settings-nav-{}", ix))
                .selected(is_selected)
                .accessibility_label(title.clone())
                .flex()
                .items_center()
                .gap_2p5()
                .px_3()
                .py_2()
                .rounded(theme.radius)
                .cursor_default()
                .focus_visible(|style| {
                    style.shadow(vec![
                        gpui::BoxShadow::new(px(0.), px(0.), theme.sidebar_ring)
                            .spread_radius(px(2.)),
                    ])
                })
                .text_base()
                .when(is_selected, |this| {
                    this.bg(theme.accent)
                        .text_color(theme.accent_foreground)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                })
                .when(!is_selected, |this| {
                    this.text_color(theme.muted_foreground)
                        .hover(|s| s.bg(theme.accent.opacity(0.5)).text_color(theme.foreground))
                })
                .when_some(on_select, |this, cb| {
                    this.on_click(move |_, window, cx| {
                        (cb)(ix, window, cx);
                    })
                })
                .when_some(icon, |this, ic| {
                    this.child(Icon::new(ic).small().text_color(if is_selected {
                        theme.accent_foreground
                    } else {
                        theme.muted_foreground
                    }))
                })
                .child(div().flex_1().child(title))
        });

        // Content of selected page
        let active_page = self.pages.get(selected_page_ix);
        let content = if let Some(page) = active_page {
            div()
                .flex()
                .flex_col()
                .gap_6()
                .p_6()
                .w_full()
                // Page header
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_size(gpui::rems(20. / 14.))
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(theme.foreground)
                                .child(page.title.clone()),
                        )
                        .when_some(page.description.clone(), |this, desc| {
                            this.child(
                                div().text_base().text_color(theme.muted_foreground).child(desc),
                            )
                        }),
                )
                // Groups
                .children(page.groups.iter().enumerate().map(|(g_ix, group)| {
                    div()
                        .id(format!("settings-group-{}", g_ix))
                        .flex()
                        .flex_col()
                        .gap_3()
                        .when_some(group.title.clone(), |this, title| {
                            this.child(
                                div()
                                    .text_size(gpui::rems(12. / 14.))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(theme.muted_foreground)
                                    .child(title),
                            )
                        })
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .rounded_xl()
                                .border_1()
                                .border_color(theme.border)
                                .bg(theme.card)
                                .overflow_hidden()
                                .children(group.items.iter().enumerate().map(|(i_ix, item)| {
                                    let is_last = i_ix + 1 == group.items.len();
                                    let label = item.label.clone();
                                    let accessibility_label = label.clone();
                                    let desc = item.description.clone();

                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .p_4()
                                                .gap_4()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .gap_0p5()
                                                        .flex_1()
                                                        .child(
                                                            div()
                                                                .text_base()
                                                                .font_weight(
                                                                    gpui::FontWeight::MEDIUM,
                                                                )
                                                                .text_color(theme.foreground)
                                                                .child(label),
                                                        )
                                                        .when_some(desc, |this, d| {
                                                            this.child(
                                                                div()
                                                                    .text_size(gpui::rems(12. / 14.))
                                                                    .text_color(
                                                                        theme.muted_foreground,
                                                                    )
                                                                    .child(d),
                                                            )
                                                        }),
                                                )
                                                .child(match &item.field.kind {
                                                    SettingFieldKind::Switch { get, set } => {
                                                        let val = get(cx);
                                                        let set = set.clone();
                                                        Switch::new(format!(
                                                            "switch-{}-{}",
                                                            g_ix, i_ix
                                                        ))
                                                        .accessibility_label(accessibility_label)
                                                        .checked(val)
                                                        .on_click(move |checked, _window, cx| {
                                                            set(*checked, cx);
                                                        })
                                                        .into_any_element()
                                                    }
                                                    SettingFieldKind::Dropdown {
                                                        options,
                                                        get,
                                                        set,
                                                        scrollable,
                                                    } => {
                                                        let val = get(cx);
                                                        let set = set.clone();
                                                        let options = options.clone();
                                                        let scrollable = *scrollable;

                                                        if !scrollable && options.len() <= 4 {
                                                            let set = set.clone();
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .p_0p5()
                                                                .rounded(theme.radius)
                                                                .bg(theme.muted)
                                                                .children(options.into_iter().enumerate().map(|(o_ix, (k, l))| {
                                                                    let is_active = k == val;
                                                                    let set = set.clone();
                                                                    let k_clone = k.clone();
                                                                    gpui_base::Button::new(format!("seg-opt-{}-{}-{}", g_ix, i_ix, o_ix))
                                                                        .selected(is_active)
                                                                        .accessibility_label(l.clone())
                                                                        .px_2p5()
                                                                        .py_1()
                                                                        .rounded(theme.radius_sm)
                                                                        .cursor_default()
                                                                        .focus_visible(|style| style.shadow(vec![
                                                                            gpui::BoxShadow::new(px(0.), px(0.), theme.ring).spread_radius(px(2.)),
                                                                        ]))
                                                                        .text_size(gpui::rems(12. / 14.))
                                                                        .font_weight(if is_active {
                                                                            gpui::FontWeight::SEMIBOLD
                                                                        } else {
                                                                            gpui::FontWeight::NORMAL
                                                                        })
                                                                        .when(is_active, |this| {
                                                                            this.bg(theme.active_tab)
                                                                                .text_color(theme.foreground)
                                                                                .when(theme.shadow, |segment| segment.shadow_sm())
                                                                        })
                                                                        .when(!is_active, |this| {
                                                                            this.text_color(theme.muted_foreground)
                                                                                .hover(|s| s.text_color(theme.foreground))
                                                                        })
                                                                        .on_click(move |_, _window, cx| {
                                                                            set(k_clone.clone(), cx);
                                                                        })
                                                                        .child(l)
                                                                }))
                                                                .into_any_element()
                                                        } else {
                                                            let label = options
                                                                .iter()
                                                                .find(|(k, _)| k == &val)
                                                                .map(|(_, l)| l.clone())
                                                                .unwrap_or_else(|| val.clone());

                                                            let options_for_menu = options.clone();
                                                            let val_for_menu = val.clone();
                                                            let set_for_menu = set.clone();

                                                            Button::new(format!("dropdown-{}-{}", g_ix, i_ix))
                                                                .outline()
                                                                .label(label)
                                                                .dropdown_caret(true)
                                                                .dropdown_menu_with_anchor(gpui::Anchor::TopRight, move |menu, _, _| {
                                                                    let mut menu = menu.scrollable(scrollable).min_w(px(200.));
                                                                    for (k, l) in options_for_menu.iter() {
                                                                        let is_checked = k == &val_for_menu;
                                                                        let set = set_for_menu.clone();
                                                                        let k_clone = k.clone();
                                                                        menu = menu.item(
                                                                            crate::overlay::menu::PopupMenuItem::new(l.clone())
                                                                                .checked(is_checked)
                                                                                .on_click(move |_, _, cx| {
                                                                                    set(k_clone.clone(), cx);
                                                                                }),
                                                                        );
                                                                    }
                                                                    menu
                                                                })
                                                                .into_any_element()
                                                        }
                                                    }
                                                    SettingFieldKind::Number {
                                                        options,
                                                        get,
                                                        set,
                                                    } => {
                                                        let val = get(cx);
                                                        let min = options.min;
                                                        let max = options.max;
                                                        let step = options.step;

                                                        let set_dec = set.clone();
                                                        let set_inc = set.clone();

                                                        let dec_val = (val - step).max(min);
                                                        let inc_val = (val + step).min(max);

                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_1p5()
                                                            .child(
                                                                Button::new(format!(
                                                                    "num-dec-{g_ix}-{i_ix}"
                                                                ))
                                                                .ghost()
                                                                .xsmall()
                                                                .icon(IconName::Minus)
                                                                .accessibility_label(format!("Уменьшить: {accessibility_label}"))
                                                                .disabled(val <= min)
                                                                .on_click(move |_, _window, cx| {
                                                                    set_dec(dec_val, cx);
                                                                }),
                                                            )
                                                            .child(
                                                                div()
                                                                    .min_w(px(36.))
                                                                    .text_center()
                                                                    .font_weight(gpui::FontWeight::MEDIUM)
                                                                    .text_base()
                                                                    .text_color(theme.foreground)
                                                                    .child(format!("{val}")),
                                                            )
                                                            .child(
                                                                Button::new(format!(
                                                                    "num-inc-{g_ix}-{i_ix}"
                                                                ))
                                                                .ghost()
                                                                .xsmall()
                                                                .icon(IconName::Plus)
                                                                .accessibility_label(format!("Увеличить: {accessibility_label}"))
                                                                .disabled(val >= max)
                                                                .on_click(move |_, _window, cx| {
                                                                    set_inc(inc_val, cx);
                                                                }),
                                                            )
                                                            .into_any_element()
                                                    }
                                                    SettingFieldKind::Input { get, set } => {
                                                        let val = get(cx);
                                                        let setter = set.clone();
                                                        let input_state = window.use_keyed_state(
                                                            format!(
                                                                "settings-input-{}-{}-{}",
                                                                selected_page_ix, g_ix, i_ix
                                                            ),
                                                            cx,
                                                            {
                                                                let initial_value = val.clone();
                                                                move |window, cx| {
                                                                    let input = cx.new(|cx| {
                                                                        InputState::new(window, cx)
                                                                            .default_value(initial_value)
                                                                    });
                                                                    let setter = setter.clone();
                                                                    let subscription = cx.subscribe(
                                                                        &input,
                                                                        move |_, input, event: &InputEvent, cx| {
                                                                            if matches!(event, InputEvent::Change) {
                                                                                setter(input.read(cx).value(), cx);
                                                                            }
                                                                        },
                                                                    );
                                                                    SettingInputState {
                                                                        input,
                                                                        _subscription: subscription,
                                                                    }
                                                                }
                                                            },
                                                        );

                                                        let input = input_state.read(cx).input.clone();
                                                        input_state.update(cx, |state, cx| {
                                                            if state.input.read(cx).value() != val {
                                                                state.input.update(cx, |input, cx| {
                                                                    input.set_value(val.clone(), window, cx);
                                                                });
                                                            }
                                                        });

                                                        Input::new(&input)
                                                            .accessibility_label(accessibility_label)
                                                            .into_any_element()
                                                    }
                                                    SettingFieldKind::Custom(f) => f(window, cx),
                                                }),
                                        )
                                        .when(!is_last, |this| this.child(Separator::horizontal()))
                                })),
                        )
                }))
        } else {
            div()
        };

        div()
            .id(self.id)
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_row()
            .child(
                div()
                    .w(self.sidebar_width)
                    .h_full()
                    .border_r_1()
                    .border_color(theme.border)
                    .bg(theme.sidebar_background)
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(nav_items),
            )
            .child(
                div()
                    .id("settings-scroll-content")
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .h_full()
                    .overflow_y_scroll()
                    .child(content),
            )
    }
}
