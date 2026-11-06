mod layout;

use std::rc::Rc;

use gpui::{
    App, ElementId, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, rems, svg,
};

use crate::{ActiveTheme, Card};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyboardFormat {
    Full,
    #[default]
    Tkl,
    Compact75,
    Compact65,
    Compact60,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyboardGeometry {
    #[default]
    Ansi,
    Iso,
    Jis,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyboardLanguage {
    #[default]
    English,
    Russian,
    German,
    French,
    Japanese,
}

type SelectHandler = Rc<dyn Fn(&'static str, &mut Window, &mut App)>;

/// A physical keyboard diagram with localizable legends and controlled key selection.
#[derive(IntoElement)]
pub struct Keyboard {
    id: ElementId,
    format: KeyboardFormat,
    geometry: KeyboardGeometry,
    language: KeyboardLanguage,
    selected_key: Option<SharedString>,
    on_select: Option<SelectHandler>,
}

impl Keyboard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            format: KeyboardFormat::default(),
            geometry: KeyboardGeometry::default(),
            language: KeyboardLanguage::default(),
            selected_key: None,
            on_select: None,
        }
    }

    pub fn format(mut self, format: KeyboardFormat) -> Self {
        self.format = format;
        self
    }

    pub fn geometry(mut self, geometry: KeyboardGeometry) -> Self {
        self.geometry = geometry;
        self
    }

    pub fn language(mut self, language: KeyboardLanguage) -> Self {
        self.language = language;
        self
    }

    /// Highlights the key with this physical code, for example "KeyK" or "ShiftLeft".
    pub fn selected_key(mut self, code: impl Into<SharedString>) -> Self {
        self.selected_key = Some(code.into());
        self
    }

    /// Reports a physical key code. Pass it to selected_key on the next render.
    pub fn on_select(
        mut self,
        handler: impl Fn(&'static str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Keyboard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let unit = |value: u16| rems(f32::from(value) / 14.);
        let (width, height, rows) = layout::layout(self.format, self.geometry);
        let mut board = div()
            .id(self.id)
            .relative()
            .flex_none()
            .w(unit(width))
            .h(unit(height))
            .font_family("Inter")
            .text_size(unit(14))
            .line_height(unit(20))
            .child(Card::new().absolute().inset_0());
        for &(y, keys) in rows {
            for &(code, default, x, w, h) in keys {
                let label = layout::legend(self.language, code, default);
                let selected = self.selected_key.as_deref() == Some(code);
                let (fill, ink) = if selected {
                    (theme.primary, theme.primary_foreground)
                } else {
                    (theme.secondary, theme.secondary_foreground)
                };
                let stepped = code == "Enter" && self.geometry != KeyboardGeometry::Ansi;
                let key = div()
                    .absolute()
                    .left(unit(x))
                    .top(unit(y))
                    .w(unit(w))
                    .h(unit(h))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(ink)
                    .when(stepped, |key| {
                        key.pl(unit(13)).child(
                            svg()
                                .data(include_bytes!("enter.svg"))
                                .absolute()
                                .inset_0()
                                .size_full()
                                .text_color(fill),
                        )
                    })
                    .when(!stepped, |key| key.rounded(theme.radius_4xl).bg(fill))
                    .child(label)
                    .when_some(self.on_select.as_ref(), |key, handler| {
                        let handler = Rc::clone(handler);
                        key.child(
                            gpui_base::Button::new(code)
                                .absolute()
                                .inset_0()
                                .cursor_pointer()
                                .accessibility_label(label)
                                .rounded(theme.radius_4xl)
                                .focus_visible(|style| style.border_2().border_color(theme.ring))
                                .on_click(move |_, window, cx| handler(code, window, cx)),
                        )
                    });
                board = board.child(key);
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Context, Modifiers, Render, TestAppContext, px};

    struct Picker(Option<&'static str>);

    impl Render for Picker {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let view = cx.weak_entity();
            Keyboard::new("picker")
                .language(KeyboardLanguage::Russian)
                .when_some(self.0, |board, code| board.selected_key(code))
                .on_select(move |code, _, cx| {
                    view.update(cx, |picker, cx| {
                        picker.0 = Some(code);
                        cx.notify();
                    })
                    .unwrap();
                })
        }
    }

    #[gpui::test]
    fn localized_key_reports_its_physical_code(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (view, cx) = cx.add_window_view(|_, _| Picker(None));
        let bounds = cx.update(|window, cx| {
            window.set_rem_size(px(14.));
            window.draw(cx).clear(cx);
            let key = gpui_base::test_support::find(window, &[], &"KeyK".into()).unwrap();
            assert_eq!(key.label(), Some("Л"));
            key.bounds()
        });
        assert_eq!(bounds.size, gpui::size(px(46.), px(36.)));
        cx.simulate_click(bounds.center(), Modifiers::default());
        cx.update(|_, cx| assert_eq!(view.read(cx).0, Some("KeyK")));
    }
}
