use std::rc::Rc;

use gpui::{AnyElement, App, ElementId, IntoElement, Pixels, SharedString, Window, px};

use crate::icon::IconName;

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
        Self {
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
        }
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
            kind: SettingFieldKind::Switch {
                get: Rc::new(get),
                set: Rc::new(set),
            },
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
            kind: SettingFieldKind::Number {
                options,
                get: Rc::new(get),
                set: Rc::new(set),
            },
            default_value: None,
        }
    }

    pub fn input<G, S>(get: G, set: S) -> Self
    where
        G: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self {
            kind: SettingFieldKind::Input {
                get: Rc::new(get),
                set: Rc::new(set),
            },
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
        Self {
            label: label.into(),
            description: None,
            field,
            keywords: Vec::new(),
        }
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
        Self {
            title: None,
            description: None,
            items: Vec::new(),
        }
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

mod panel;

#[cfg(test)]
mod tests;
