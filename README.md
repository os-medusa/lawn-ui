# lawn-ui

[![Crates.io](https://img.shields.io/crates/v/lawn-ui.svg)](https://crates.io/crates/lawn-ui)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A modern, comprehensive shadcn/ui-inspired design system and component library for [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) on top of `gpui-base` and `gpui-fast`.

Directly mapped and 100% synchronized with the **FigCraft Luma Neutral Design System** specification.

---

## 🎨 Design System Components (100% FigCraft Parity)

`lawn-ui` implements all **28 core components** defined in FigCraft (`02 · shadcn / Components`):

| # | Component | Module | Features & Variants |
|---|---|---|---|
| 1 | **Button** | `lawn_ui::Button` | 6 variants (`Primary`, `Secondary`, `Outline`, `Ghost`, `Destructive`, `Link`), 3 sizes (`sm: 28px`, `md: 36px`, `lg: 40px`), hover/active/disabled states, loading spinner, dropdown split |
| 2 | **Input** | `lawn_ui::Input` | Form input with focus rings, clear buttons, prefix/suffix icons, error states |
| 3 | **Select** | `lawn_ui::Select` | Popover dropdown selector with items list and keyboard focus |
| 4 | **Textarea** | `lawn_ui::Textarea` | Multi-line text editor with auto-scroll and rem-baseline sizing |
| 5 | **Badge** | `lawn_ui::Badge` | Count badges and tags with semantic theme variants |
| 6 | **Switch** | `lawn_ui::Switch` | Toggle switch with spring animation and focus rings |
| 7 | **Checkbox** | `lawn_ui::Checkbox` | Checked, unchecked, and indeterminate states with `radius_checkbox` |
| 8 | **Radio Group** | `lawn_ui::Radio`, `RadioGroup` | Single selection groups with horizontal or vertical layout |
| 9 | **Slider** | `lawn_ui::Slider` | Range and single value track/thumb sliders with drag & touch support |
| 10 | **Tabs** | `lawn_ui::Tabs` | Tab bar and triggers with active tab styling and focus |
| 11 | **Breadcrumb** | `lawn_ui::Breadcrumb` | Trail navigation with chevrons, links, and current page |
| 12 | **Pagination** | `lawn_ui::Pagination` | Numbered page buttons, previous/next controls, and disabled bounds |
| 13 | **Dropdown Menu** | `lawn_ui::Menu`, `DropdownMenu` | Floating context and dropdown menus with keyboard navigation |
| 14 | **Accordion** | `lawn_ui::Accordion` | Collapsible content sections with chevron rotation |
| 15 | **Tooltip** | `lawn_ui::Tooltip` | Hover tooltips with show delay and anchor positioners |
| 16 | **Popover** | `lawn_ui::Popover` | Anchored overlay panels with focus trapping and dismiss |
| 17 | **Alert** | `lawn_ui::Alert` | Status banners (`Default`, `Info`, `Success`, `Warning`, `Destructive`) with dismiss button |
| 18 | **Progress** | `lawn_ui::Progress` | Determinate progress bars with semantic status fills |
| 19 | **Avatar** | `lawn_ui::Avatar` | Image avatars, initials fallback, and status dot indicators |
| 20 | **Skeleton** | `lawn_ui::Skeleton` | Loading shimmer placeholders for text, cards, and avatars |
| 21 | **Card** | `lawn_ui::Card` | Container cards with Header, Title, Description, Content, Footer |
| 22 | **Dialog** | `lawn_ui::Dialog`, `AlertDialog` | Modal dialogs with backdrop blur, focus trap, and action footers |
| 23 | **Table** | `lawn_ui::Table` | Full table system (`TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`, `TableCaption`) |
| 24 | **Sidebar** | `lawn_ui::Sidebar` | Collapsible sidebar layout with navigation groups and buttons |
| 25 | **Calendar** | `lawn_ui::Calendar` | Monthly grid view, date selection, month/year navigation |
| 26 | **Item** | `lawn_ui::Item` | Structured list item card with leading icon, title, description, and chevron |
| 27 | **Empty State** | `lawn_ui::Empty` | Informative placeholders with illustration, title, description, and CTA button |
| 28 | **Lucide Icons** | `lawn_ui::Icon`, `IconName` | Full Lucide icon set embedded via `gpui-kit-assets` |

---

## ⚙️ Cargo Features

- `chat` *(default)*: Includes chat & messaging primitives (`Bubble`, `Message`, `MessageScroller`, `Attachment`). To build `lawn-ui` as a lean, general-purpose desktop UI toolkit without messaging components, use:
  ```toml
  [dependencies]
  lawn-ui = { version = "0.1.0", default-features = false }
  ```

---

## 🚀 Getting Started

Add `lawn-ui` to your `Cargo.toml`:

```toml
[dependencies]
lawn-ui = "0.1.0"
gpui = { package = "gpui-fast", version = "0.1.10" }
```

Initialize and render:

```rust
use gpui::*;
use lawn_ui::{Theme, Button, ButtonVariants, Table, TableHeader, TableRow, TableHead, TableBody, TableCell, v_flex};

fn main() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        lawn_ui::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|_| AppView)
        }).unwrap();
    });
}

struct AppView;

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_8()
            .gap_4()
            .child(
                Button::new("btn-primary")
                    .primary()
                    .label("Get Started")
            )
    }
}
```

---

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
