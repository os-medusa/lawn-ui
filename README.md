# lawn-ui

[![Crates.io](https://img.shields.io/crates/v/lawn-ui.svg)](https://crates.io/crates/lawn-ui)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A modern shadcn/ui-inspired design system and component library for [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) on top of `gpui-base` and `gpui-fast`.

## Features

- **shadcn / Luma Design Tokens**: Semantic color tokens with native Light and Dark mode palettes, precise contrast ratios, border radii, and elevation shadows.
- **Rich Component Library**:
  - **Primitives**: `Button` (6 variants, 3 sizes, focus rings, loading & dropdown split support), `Avatar`, `Badge`, `Card`, `Checkbox`, `Switch`, `Progress`, `Skeleton`, `Separator`, `MetricBox`, `Empty`.
  - **Forms**: `Input`, `Textarea`, `Field`.
  - **Layout**: `Sidebar`, `Tabs`, `TitleBar`, `ScrollArea`, `ResizablePanel`.
  - **Overlays**: `Dialog`, `AlertDialog`, `Sheet`, `Popover`, `Tooltip`, `Menu` / `DropdownMenu`.
  - **Chat & Messaging**: `Bubble`, `Message`, `MessageScroller`, `Attachment`.
  - **Settings Engine**: Type-safe settings pages, groups, and inputs.
- **Accessibility & Keyboard Navigation**: Visible focus rings (`ring` 2px), Tab-stop management, and accessibility labels.
- **Rem-baseline Scaling**: Controls scale proportionally relative to the application base rem size (14px baseline).

## Getting Started

Add `lawn-ui` to your `Cargo.toml`:

```toml
[dependencies]
lawn-ui = "0.1.0"
gpui = { package = "gpui-fast", version = "0.1.10" }
```

Initialize `lawn-ui` in your GPUI application:

```rust
use gpui::*;
use lawn-ui::{Theme, Button, ButtonVariants, v_flex};

fn main() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        lawn_ui::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|_| HelloWorld)
        }).unwrap();
    });
}

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Button::new("btn-primary")
                    .primary()
                    .label("Click me")
            )
    }
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
