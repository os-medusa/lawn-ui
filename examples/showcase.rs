use std::borrow::Cow;

use gpui::prelude::*;
use gpui::{
    App, AssetSource, Bounds, ClickEvent, Context, Entity, IntoElement,
    ParentElement, Render, Result, SharedString, Styled, Window, WindowBounds,
    WindowOptions, div, px, rems,
};

use lawn_ui::*;

struct LawnAssets;

impl AssetSource for LawnAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        lawn_ui::assets::AllAssets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        lawn_ui::assets::AllAssets.list(path)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShowcaseTab {
    Buttons,
    Forms,
    Surfaces,
    Navigation,
    Feedback,
}

struct ShowcaseApp {
    current_tab: ShowcaseTab,
    is_dark: bool,
    input_state: Entity<InputState>,
    number_val: i64,
    otp_val: SharedString,
    checkbox_checked: bool,
    switch_checked: bool,
    toggle_pressed: bool,
    selected_date: Option<(u32, u32, u32)>,
    tree_selected: SharedString,
}

impl ShowcaseApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx));
        Self {
            current_tab: ShowcaseTab::Buttons,
            is_dark: true,
            input_state,
            number_val: 42,
            otp_val: "7392".into(),
            checkbox_checked: true,
            switch_checked: true,
            toggle_pressed: true,
            selected_date: Some((2026, 10, 24)),
            tree_selected: "src/lib.rs".into(),
        }
    }

    fn toggle_theme(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_dark = !self.is_dark;
        if self.is_dark {
            Theme::set_global(cx, Theme::dark());
        } else {
            Theme::set_global(cx, Theme::light());
        }
        cx.notify();
    }
}

impl Render for ShowcaseApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let current_tab = self.current_tab;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            // Top Navigation Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(rems(48. / 14.))
                    .px_6()
                    .border_b_1()
                    .border_color(theme.border)
                    .bg(theme.card)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(28.))
                                    .h(px(28.))
                                    .rounded(theme.radius_sm)
                                    .bg(theme.primary)
                                    .text_color(theme.primary_foreground)
                                    .child(Icon::new(IconName::Sparkles).size_4()),
                            )
                            .child(
                                div()
                                    .text_base()
                                    .font_semibold()
                                    .child("lawn-ui Design System"),
                            )
                            .child(
                                Badge::new()
                                    .child("37 Components")
                                    .secondary(),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                Button::new("toggle-theme")
                                    .label(if self.is_dark { "☀️ Light" } else { "🌙 Dark" })
                                    .outline()
                                    .small()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.toggle_theme(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("docs-btn")
                                    .label("GitHub")
                                    .secondary()
                                    .small(),
                            ),
                    ),
            )
            // Main workspace with Sidebar & Content
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    // Sidebar
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(240.))
                            .p_4()
                            .gap_1()
                            .border_r_1()
                            .border_color(theme.border)
                            .bg(theme.card)
                            .child(render_nav_item(
                                "nav-buttons",
                                "Buttons & Actions",
                                IconName::SquarePlay,
                                current_tab == ShowcaseTab::Buttons,
                                cx.listener(|this, _, _, cx| {
                                    this.current_tab = ShowcaseTab::Buttons;
                                    cx.notify();
                                }),
                            ))
                            .child(render_nav_item(
                                "nav-forms",
                                "Form Controls",
                                IconName::SlidersHorizontal,
                                current_tab == ShowcaseTab::Forms,
                                cx.listener(|this, _, _, cx| {
                                    this.current_tab = ShowcaseTab::Forms;
                                    cx.notify();
                                }),
                            ))
                            .child(render_nav_item(
                                "nav-surfaces",
                                "Surfaces & Overlays",
                                IconName::Layers,
                                current_tab == ShowcaseTab::Surfaces,
                                cx.listener(|this, _, _, cx| {
                                    this.current_tab = ShowcaseTab::Surfaces;
                                    cx.notify();
                                }),
                            ))
                            .child(render_nav_item(
                                "nav-navigation",
                                "Navigation & Data",
                                IconName::Compass,
                                current_tab == ShowcaseTab::Navigation,
                                cx.listener(|this, _, _, cx| {
                                    this.current_tab = ShowcaseTab::Navigation;
                                    cx.notify();
                                }),
                            ))
                            .child(render_nav_item(
                                "nav-feedback",
                                "Feedback & Indicators",
                                IconName::CheckCheck,
                                current_tab == ShowcaseTab::Feedback,
                                cx.listener(|this, _, _, cx| {
                                    this.current_tab = ShowcaseTab::Feedback;
                                    cx.notify();
                                }),
                            )),
                    )
                    // Content Canvas (Scrollable)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .p_8()
                            .gap_6()
                            .overflow_hidden()
                            .child(match current_tab {
                                ShowcaseTab::Buttons => self.render_buttons_tab(window, cx),
                                ShowcaseTab::Forms => self.render_forms_tab(window, cx),
                                ShowcaseTab::Surfaces => self.render_surfaces_tab(window, cx),
                                ShowcaseTab::Navigation => self.render_navigation_tab(window, cx),
                                ShowcaseTab::Feedback => self.render_feedback_tab(window, cx),
                            }),
                    ),
            )
    }
}

fn render_nav_item(
    id: &'static str,
    title: &'static str,
    icon: IconName,
    active: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .gap_2p5()
        .h(rems(36. / 14.))
        .px_3()
        .rounded(px(8.))
        .cursor_pointer()
        .text_sm()
        .font_medium()
        .when(active, |this| {
            this.bg(gpui::hsla(0., 0., 0.5, 0.15))
        })
        .when(!active, |this| {
            this.hover(|s| s.bg(gpui::hsla(0., 0., 0.5, 0.08)))
        })
        .on_click(on_click)
        .child(Icon::new(icon).size_4())
        .child(title)
}

impl ShowcaseApp {
    fn render_buttons_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let entity = cx.entity().clone();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Button Variants"))
                            .child(CardDescription::new().child("6 standard shadcn/ui variants")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_wrap()
                                .items_center()
                                .gap_3()
                                .child(Button::new("btn-p").label("Primary").primary())
                                .child(Button::new("btn-s").label("Secondary").secondary())
                                .child(Button::new("btn-o").label("Outline").outline())
                                .child(Button::new("btn-g").label("Ghost").ghost())
                                .child(Button::new("btn-d").label("Destructive").destructive())
                                .child(Button::new("btn-l").label("Link").link()),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Button Sizing Scale"))
                            .child(CardDescription::new().child("SM (28px), MD (36px), LG (40px)")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_3()
                                .child(Button::new("btn-sm").label("Small (28px)").small())
                                .child(Button::new("btn-md").label("Default (36px)"))
                                .child(Button::new("btn-lg").label("Large (40px)").large())
                                .child(Button::new("btn-dis").label("Disabled").disabled(true)),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Toggle & ToggleGroup"))
                            .child(CardDescription::new().child("Two-state buttons and toolbar groups")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_4()
                                .child(
                                    Toggle::new("sample-toggle")
                                        .pressed(self.toggle_pressed)
                                        .on_toggle(move |pressed, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.toggle_pressed = pressed;
                                                cx.notify();
                                            });
                                        })
                                        .child("Press Toggle"),
                                )
                                .child(
                                    ToggleGroup::new("sample-togglegroup")
                                        .child(Toggle::new("tg-b").size(ToggleSize::Sm).child("B"))
                                        .child(Toggle::new("tg-i").size(ToggleSize::Sm).pressed(true).child("I"))
                                        .child(Toggle::new("tg-u").size(ToggleSize::Sm).child("U")),
                                ),
                        ),
                    ),
            )
    }

    fn render_forms_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let entity = cx.entity().clone();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Text & Numeric Inputs"))
                            .child(CardDescription::new().child("Input, NumberInput, and OtpInput")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .w(px(380.))
                                .child(Input::new(&self.input_state))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child("Quantity (Stepper):")
                                        .child(
                                            NumberInput::new("sample-num", self.number_val)
                                                .min(0)
                                                .max(100)
                                                .on_change(move |val, _, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.number_val = val;
                                                        cx.notify();
                                                    });
                                                }),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child("2FA Passcode (OtpInput):")
                                        .child(
                                            OtpInput::new("sample-otp")
                                                .length(4)
                                                .value(self.otp_val.clone()),
                                        ),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Pickers & Dropdowns"))
                            .child(CardDescription::new().child("Combobox, DatePicker, and Select")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .w(px(380.))
                                .child(
                                    Combobox::new("sample-combo")
                                        .placeholder("Search framework...")
                                        .item(ComboboxItem::new("gpui", "GPUI"))
                                        .item(ComboboxItem::new("zed", "Zed UI"))
                                        .item(ComboboxItem::new("shadcn", "shadcn/ui")),
                                )
                                .child(
                                    DatePicker::new("sample-date")
                                        .selected_date(self.selected_date),
                                )
                                .child(
                                    Select::new("sample-select")
                                        .placeholder("Choose role...")
                                        .item("admin", "Administrator")
                                        .item("member", "Member")
                                        .item("guest", "Guest"),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Selection Controls"))
                            .child(CardDescription::new().child("Checkbox, Switch, and RadioGroup")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .w(px(380.))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_3()
                                        .child(Checkbox::new("sample-chk").checked(self.checkbox_checked))
                                        .child("Enable notifications"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_3()
                                        .child(Switch::new("sample-sw").checked(self.switch_checked))
                                        .child("Airplane mode"),
                                )
                                .child(
                                    RadioGroup::new()
                                        .child(Radio::new("r-1").label("Option One"))
                                        .child(Radio::new("r-2").label("Option Two").checked(true)),
                                ),
                        ),
                    ),
            )
    }

    fn render_surfaces_tab(&self, _window: &mut Window, _cx: &mut Context<Self>) -> gpui::Div {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Card & Item Surfaces"))
                            .child(CardDescription::new().child("Structured containers")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(
                                    Item::new("Account Security")
                                        .icon(Icon::new(IconName::User))
                                        .description("Manage multi-factor authentication and recovery codes")
                                        .show_chevron(true),
                                )
                                .child(
                                    Item::new("Cloud Storage")
                                        .icon(Icon::new(IconName::HardDrive))
                                        .description("14.2 GB of 50 GB used")
                                        .show_chevron(true),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Alert Banners"))
                            .child(CardDescription::new().child("Informational and critical notices")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(
                                    Alert::new()
                                        .variant(AlertVariant::Info)
                                        .title("Update Available")
                                        .description("lawn-ui v0.1.0 is ready to install."),
                                )
                                .child(
                                    Alert::new()
                                        .variant(AlertVariant::Success)
                                        .title("Deployment Successful")
                                        .description("Your application is now live."),
                                )
                                .child(
                                    Alert::new()
                                        .variant(AlertVariant::Destructive)
                                        .title("Build Error")
                                        .description("Compilation failed with 1 error."),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Toast Notification (Sonner)"))
                            .child(CardDescription::new().child("Floating toast overlay")),
                    )
                    .child(
                        CardContent::new().child(
                            Toast::new("sample-toast", "Event Created")
                                .description("We've added your event to the calendar.")
                                .variant(ToastVariant::Success)
                                .action("Undo", |_, _| {}),
                        ),
                    ),
            )
    }

    fn render_navigation_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let entity = cx.entity().clone();

        let tree_node = TreeNode::new("root", "Project")
            .expanded(true)
            .child(
                TreeNode::new("src", "src")
                    .icon(IconName::Folder)
                    .expanded(true)
                    .child(TreeNode::new("lib.rs", "lib.rs").icon(IconName::FileCode))
                    .child(TreeNode::new("main.rs", "main.rs").icon(IconName::FileCode)),
            )
            .child(TreeNode::new("Cargo.toml", "Cargo.toml").icon(IconName::FileText));

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Tabs & Breadcrumbs"))
                            .child(CardDescription::new().child("Location and view navigation")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .child(
                                    Tabs::new().child(
                                        TabsList::new()
                                            .child(TabsTrigger::new("tab-overview", "Overview").selected(true))
                                            .child(TabsTrigger::new("tab-billing", "Billing"))
                                            .child(TabsTrigger::new("tab-settings", "Settings")),
                                    ),
                                )
                                .child(
                                    Breadcrumb::new().child(
                                        BreadcrumbList::new()
                                            .child(BreadcrumbItem::new().child(BreadcrumbLink::new("Home")))
                                            .child(BreadcrumbSeparator::new())
                                            .child(BreadcrumbItem::new().child(BreadcrumbLink::new("Components")))
                                            .child(BreadcrumbSeparator::new())
                                            .child(BreadcrumbItem::new().child(BreadcrumbPage::new("Navigation"))),
                                    ),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Tree View"))
                            .child(CardDescription::new().child("Hierarchical folder & file explorer")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .w(px(320.))
                                .child(
                                    Tree::new("sample-tree")
                                        .node(tree_node)
                                        .on_select(move |node_id, _, cx| {
                                            let id = node_id.clone();
                                            entity.update(cx, |this, cx| {
                                                this.tree_selected = id;
                                                cx.notify();
                                            });
                                        }),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Table"))
                            .child(CardDescription::new().child("Data grid with rows and headers")),
                    )
                    .child(
                        CardContent::new().child(
                            Table::new()
                                .child(
                                    TableHeader::new().child(
                                        TableRow::new()
                                            .child(TableHead::new().child("Invoice"))
                                            .child(TableHead::new().child("Status"))
                                            .child(TableHead::new().child("Amount")),
                                    ),
                                )
                                .child(
                                    TableBody::new()
                                        .child(
                                            TableRow::new()
                                                .child(TableCell::new().child("INV001"))
                                                .child(TableCell::new().child("Paid"))
                                                .child(TableCell::new().child("$250.00")),
                                        )
                                        .child(
                                            TableRow::new()
                                                .child(TableCell::new().child("INV002"))
                                                .child(TableCell::new().child("Pending"))
                                                .child(TableCell::new().child("$150.00")),
                                        ),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Calendar & Accordion"))
                            .child(CardDescription::new().child("Date selection and collapsible sections")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_start()
                                .gap_6()
                                .child(Calendar::new(2026, 10).selected_day(Some(24)))
                                .child(
                                    div()
                                        .flex_1()
                                        .child(
                                            Accordion::new()
                                                .child(
                                                    AccordionItem::new()
                                                        .child(AccordionTrigger::new("q1", "Is lawn-ui fully accessible?").open(true))
                                                        .child(AccordionContent::new().open(true).child("Yes, it implements standard keyboard navigation, focus management, and semantic roles.")),
                                                )
                                                .child(
                                                    AccordionItem::new()
                                                        .child(AccordionTrigger::new("q2", "Does it support custom themes?").open(false))
                                                        .child(AccordionContent::new().open(false).child("Yes, all colors, metrics, and radii are customizable via ThemeTokens.")),
                                                ),
                                        ),
                                ),
                        ),
                    ),
            )
    }

    fn render_feedback_tab(&self, _window: &mut Window, _cx: &mut Context<Self>) -> gpui::Div {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Avatars & Badges"))
                            .child(CardDescription::new().child("Status indicators and count chips")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_4()
                                        .child(Avatar::new().fallback("AG").size(AvatarSize::Sm))
                                        .child(Avatar::new().fallback("GP").size(AvatarSize::Md).presence(Some(PresenceStatus::Online)))
                                        .child(Avatar::new().fallback("UI").size(AvatarSize::Lg).presence(Some(PresenceStatus::Busy))),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_3()
                                        .child(Badge::new().child("Primary").default_variant())
                                        .child(Badge::new().child("Secondary").secondary())
                                        .child(Badge::new().child("Outline").outline())
                                        .child(Badge::new().child("Destructive").destructive()),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Progress & Skeletons"))
                            .child(CardDescription::new().child("Loading and progress states")),
                    )
                    .child(
                        CardContent::new().child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .w(px(380.))
                                .child(Progress::new(0.65))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_3()
                                        .child(Skeleton::new().circle().size(px(40.)))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_2()
                                                .child(Skeleton::new().w(px(160.)).h(px(16.)))
                                                .child(Skeleton::new().w(px(100.)).h(px(12.))),
                                        ),
                                ),
                        ),
                    ),
            )
            .child(
                Card::new()
                    .child(
                        CardHeader::new()
                            .child(CardTitle::new().child("Empty State"))
                            .child(CardDescription::new().child("Placeholder for empty views")),
                    )
                    .child(
                        CardContent::new().child(
                            Empty::new()
                                .child(
                                    EmptyHeader::new()
                                        .title(EmptyTitle::new().child("No projects found"))
                                        .description(EmptyDescription::new().child("Get started by creating a new project in your workspace.")),
                                )
                                .child(Button::new("create-proj").label("Create Project").primary()),
                        ),
                    ),
            )
    }
}

fn main() {
    lawn_ui::application()
        .with_assets(LawnAssets)
        .run(move |cx| {
            lawn_ui::init(cx);

            let bounds = Bounds::centered(None, gpui::size(px(1200.), px(820.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(gpui::size(px(800.), px(600.))),
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("lawn-ui Design System Showcase".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| ShowcaseApp::new(window, cx)),
            )
            .expect("open window");
        });
}
