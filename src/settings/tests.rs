use super::{SelectIndex, SettingField, SettingGroup, SettingItem, SettingPage, Settings};
use crate::{Button, Root, Theme};
use gpui::{
    AppContext, Context, ElementId, Entity, Global, InteractiveElement, IntoElement, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, ParentElement, Render, SharedString, Styled, TestAppContext,
    VisualTestContext, Window, div, px, size,
};
use gpui_base::{
    TestSupportExt,
    test_support::{ElementSnapshot, find, snapshots},
};

struct Values {
    workspace_title: SharedString,
    display_name: SharedString,
    read_receipts: bool,
    name_changes: usize,
}

impl Global for Values {}

struct SettingsFixture {
    selected: SelectIndex,
    selections: Vec<usize>,
}

impl Render for SettingsFixture {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        div().size_full().tab_group().child(
            Settings::new("settings-fixture")
                .default_selected_index(self.selected)
                .on_select_page(move |page_ix, _, cx| {
                    owner
                        .update(cx, |this, cx| {
                            this.selected.page_ix = page_ix;
                            this.selections.push(page_ix);
                            cx.notify();
                        })
                        .unwrap();
                })
                .page(
                    SettingPage::new("Appearance").group(
                        SettingGroup::new()
                            .title("Workspace")
                            .item(SettingItem::render(|_, _, _| {
                                div()
                                    .id("custom-panel")
                                    .test_support()
                                    .w_full()
                                    .min_w_0()
                                    .child(
                                        Button::new("choose-background")
                                            .outline()
                                            .w_full()
                                            .label("Choose background"),
                                    )
                            }))
                            .item(SettingItem::new(
                                "Workspace title",
                                SettingField::input(
                                    |cx| cx.global::<Values>().workspace_title.clone(),
                                    |value, cx| {
                                        cx.global_mut::<Values>().workspace_title = value;
                                        cx.refresh_windows();
                                    },
                                ),
                            ))
                            .item(
                                SettingItem::new(
                                    "Send read receipts",
                                    SettingField::switch(
                                        |cx| cx.global::<Values>().read_receipts,
                                        |value, cx| {
                                            cx.global_mut::<Values>().read_receipts = value;
                                            cx.refresh_windows();
                                        },
                                    ),
                                )
                                .description(
                                    "Let participants see when their messages have been read.",
                                ),
                            ),
                    ),
                )
                .page(
                    SettingPage::new("Profile").group(
                        SettingGroup::new().item(
                            SettingItem::new(
                                "Display name",
                                SettingField::input(
                                    |cx| cx.global::<Values>().display_name.clone(),
                                    |value, cx| {
                                        let values = cx.global_mut::<Values>();
                                        values.display_name = value;
                                        values.name_changes += 1;
                                        cx.refresh_windows();
                                    },
                                ),
                            )
                            .keywords(["screenreader"]),
                        ),
                    ),
                ),
        )
    }
}

fn fixture(cx: &mut TestAppContext) -> (Entity<SettingsFixture>, &mut VisualTestContext) {
    cx.update(|cx| {
        crate::init(cx);
        cx.set_global(Values {
            workspace_title: "Roadmap".into(),
            display_name: "".into(),
            read_receipts: false,
            name_changes: 0,
        });
    });
    let mut fixture = None;
    let (_, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|_| SettingsFixture {
            selected: SelectIndex::default(),
            selections: Vec::new(),
        });
        fixture = Some(view.clone());
        Root::new(view, window, cx)
    });
    cx.simulate_resize(size(px(1100.), px(720.)));
    render_frame(cx);
    (fixture.unwrap(), cx)
}

fn render_frame(cx: &mut VisualTestContext) {
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.simulate_next_frame(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

fn control(cx: &mut VisualTestContext, id: impl Into<ElementId>) -> ElementSnapshot {
    let id = id.into();
    render_frame(cx);
    cx.update(|window, _| {
        find(window, &[], &id).unwrap_or_else(|| panic!("rendered settings control {id:?}"))
    })
}

fn labelled(cx: &mut VisualTestContext, label: &str) -> ElementSnapshot {
    render_frame(cx);
    cx.update(|window, _| {
        let mut matches = snapshots(window)
            .into_iter()
            .filter(|element| element.visible() && element.label() == Some(label));
        let target = matches
            .next()
            .unwrap_or_else(|| panic!("visible control labelled {label:?}"));
        assert!(
            matches.next().is_none(),
            "settings control label must be unique: {label:?}"
        );
        target
    })
}

fn click(cx: &mut VisualTestContext, id: impl Into<ElementId>) {
    let target = control(cx, id);
    cx.simulate_click(target.bounds().center(), Modifiers::default());
    render_frame(cx);
}

fn click_labelled(cx: &mut VisualTestContext, label: &str) {
    let target = labelled(cx, label);
    cx.simulate_click(target.bounds().center(), Modifiers::default());
    render_frame(cx);
}

fn press(cx: &mut VisualTestContext, key: &str) {
    let keystroke = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
    render_frame(cx);
}

#[gpui::test]
fn page_navigation_reports_pointer_and_keyboard_selection(cx: &mut TestAppContext) {
    let (fixture, cx) = fixture(cx);
    cx.update(|_, cx| assert_eq!(fixture.read(cx).selected.page_ix, 0));
    assert!(control(cx, "choose-background").visible());

    click(cx, "settings-nav-1");
    cx.update(|_, cx| assert_eq!(fixture.read(cx).selected.page_ix, 1));
    assert!(labelled(cx, "Display name").visible());

    click(cx, "settings-nav-0");
    press(cx, "tab");
    assert_eq!(control(cx, "settings-nav-1").focused(), Some(true));
    press(cx, "enter");
    cx.update(|_, cx| assert_eq!(fixture.read(cx).selected.page_ix, 1));
    cx.update(|_, cx| {
        let fixture = fixture.read(cx);
        assert_eq!(fixture.selected.page_ix, 1);
        assert_eq!(
            fixture.selections,
            [1, 0, 1],
            "each activation reports one selection"
        );
    });
}

#[gpui::test]
fn keyword_search_reaches_other_pages_and_preserves_edited_values(cx: &mut TestAppContext) {
    let (_, cx) = fixture(cx);
    click_labelled(cx, "Поиск настроек");
    cx.simulate_input("SCREENREADER");
    render_frame(cx);
    assert!(
        labelled(cx, "Display name").visible(),
        "keywords must search inactive pages"
    );

    click_labelled(cx, "Display name");
    cx.simulate_input("Ada 中文");
    render_frame(cx);
    cx.update(|_, cx| {
        let values = cx.global::<Values>();
        assert_eq!(values.display_name.as_ref(), "Ada 中文");
        assert!(
            values.name_changes > 0,
            "editing must update the owner's value"
        );
    });

    click(cx, "settings-nav-0");
    assert!(
        control(cx, "choose-background").visible(),
        "navigation clears the query and shows its destination"
    );
    click(cx, "settings-nav-1");
    click_labelled(cx, "Display name");
    press(cx, "backspace");
    cx.update(|_, cx| assert_eq!(cx.global::<Values>().display_name.as_ref(), "Ada 中"));
}

#[gpui::test]
fn custom_items_fill_their_rows_and_controls_fit_narrow_zoomed_windows(cx: &mut TestAppContext) {
    let (_, cx) = fixture(cx);
    let custom = control(cx, "custom-panel").bounds();
    let row = control(cx, "settings-field-0-0-0-row").bounds();
    let action = control(cx, "choose-background").bounds();
    assert_eq!(
        custom.left(),
        row.left(),
        "custom items must not reserve an empty label column"
    );
    assert_eq!(custom.right(), row.right());
    assert_eq!(action.left(), custom.left());
    assert_eq!(action.right(), custom.right());

    cx.update(|_, cx| {
        let mut theme = Theme::dark();
        theme.set_scale(px(21.), px(8.));
        Theme::set_global(cx, theme);
        cx.refresh_windows();
    });
    cx.simulate_resize(size(px(420.), px(720.)));
    render_frame(cx);
    for id in [
        "custom-panel",
        "choose-background",
        "settings-field-0-0-1-row",
        "settings-field-0-0-2",
    ] {
        let target = control(cx, id);
        assert!(
            target.visible(),
            "{id} must remain reachable in a narrow window"
        );
        let bounds = target.bounds();
        assert!(
            bounds.left() >= px(0.) && bounds.right() <= px(420.),
            "{id} overflows the viewport: {bounds:?}"
        );
    }
    let custom = control(cx, "custom-panel").bounds();
    let input = labelled(cx, "Workspace title").bounds();
    assert_eq!(
        input.left(),
        custom.left(),
        "narrow form controls share the content spine"
    );
    assert_eq!(input.right(), custom.right());
    assert!(
        input.top() > custom.bottom(),
        "stacked controls must not overlap preceding rows"
    );
    cx.update(|_, cx| assert_eq!(cx.global::<Values>().workspace_title.as_ref(), "Roadmap"));
}
