use gpui::Pixels;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    Custom(Pixels),
}

impl From<Pixels> for Size {
    fn from(px: Pixels) -> Self {
        Size::Custom(px)
    }
}

pub trait Sizable: Sized {
    fn with_size(self, size: impl Into<Size>) -> Self;

    fn xsmall(self) -> Self {
        self.with_size(Size::XSmall)
    }

    fn small(self) -> Self {
        self.with_size(Size::Small)
    }

    fn medium(self) -> Self {
        self.with_size(Size::Medium)
    }

    fn large(self) -> Self {
        self.with_size(Size::Large)
    }
}

pub trait Selectable: Sized {
    fn selected(self, selected: bool) -> Self;
}

pub trait Disableable: Sized {
    fn disabled(self, disabled: bool) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl Side {
    pub fn is_left(self) -> bool {
        matches!(self, Side::Left)
    }

    pub fn is_right(self) -> bool {
        matches!(self, Side::Right)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    #[default]
    Dark,
}

pub trait SearchableListItem {
    type Value: Clone + PartialEq;
    fn title(&self) -> gpui::SharedString;
    fn display_title(&self) -> Option<gpui::AnyElement> {
        None
    }
    fn render(&self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement;
    fn value(&self) -> &Self::Value;
    fn matches(&self, query: &str) -> bool {
        self.title().to_lowercase().contains(&query.to_lowercase())
    }
}
