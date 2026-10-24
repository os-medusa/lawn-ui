# FigCraft style reference

Source: `glyph-ui_design_system.figcraft`, page `02 · shadcn / Components`, Luma Neutral Light and Dark. Component nodes are authoritative; the generic shadcn radius scale alone does not describe their appearance.

## Typography and application setup

Use Inter and a 14px window rem baseline. Install or register Inter in the host application; selecting its family name alone does not bundle the font.

```rust
window.set_rem_size(cx.theme().font_size);
div()
    .font_family("Inter")
    .text_size(cx.theme().font_size)
```

Component type sizes use explicit `rems(px / 14.)` values. GPUI's `text_sm()` assumes a 16px rem and renders 12.25px at Lawn's 14px baseline. Body labels are 14px; badges, table cells and hints are 12px; card titles are 16px; dialog and empty-state titles are 18px. Text in the saved component nodes is Regular. The showcase sets the baseline and font family explicitly.

## Component geometry and surfaces

Dimensions refer to the default 14px baseline.

| Component | Reference styling |
|---|---|
| Button | 28 / 36 / 40px high; 26px radius; 14px labels; 12px horizontal padding |
| Input, Select | 36px high; 22px radius; control-fill; no visible resting border |
| Textarea | Minimum 100px high; 18px radius; 12px padding; control-fill |
| Badge | 22px high; 22px radius; 12px label |
| Checkbox | 16px indicator; 5px radius; checkbox-fill when unchecked |
| Radio | 16px circle; checkbox-fill off; primary on with 6px inverse dot |
| Switch | 44 × 20px track; 24 × 16px thumb; checkbox-fill off; primary on |
| Slider | 8px track in checkbox-fill; primary range; white 24 × 16px pill thumb |
| Tabs | 36px list with 18px radius; active 28px pill with 14px radius |
| Pagination | 36px controls; selected page uses secondary and 18px radius |
| Dropdown menu, Popover | 18px radius; popover with 1px card-outline |
| Tooltip | 32px high; 14px radius; primary fill and inverse 12px text |
| Card, Dialog, Empty | 26px radius; 1px card-outline; card/popover surface |
| Alert | 18px radius; transparent surface; border or destructive outline |
| Table | 48px minimum rows; 12px labels; straight row separators |
| Item | 18px radius; transparent or item-fill surface |
| Calendar | 18px surface radius; circular selected date |

Combobox, DatePicker and NumberInput use the same 36px control surface and 22px radius as Input. OTP slots use the same fill and radius. Toggles use the button radius. These desktop extensions have no dedicated reference board.

## Palette and states

`design/figcraft-tokens.json` stores source color and metric values for both modes. Do not substitute control-fill for checkbox-fill: their alpha values differ.

| Token | Light | Dark |
|---|---|---|
| control-fill | #e5e5e580 | #ffffff13 |
| checkbox-fill | #e5e5e5e6 | #ffffff22 |
| card-outline | #0a0a0a0d | #fafafa1a |
| secondary-hover | #f5f5f5cc | #262626cc |

Focus uses the theme ring and a 2px outer halo; disabled controls use 50% opacity. Outline/ghost buttons use the accent surface on hover. Filled action hover changes the fill alpha without fading its label. Link buttons are underlined. Destructive labels retain the existing contrast-safe foreground.

## Verification scope

Palette tests compare runtime colors against the exported token snapshot. Existing GPUI tests cover keyboard activation, disabled controls, retained editor values and control scaling. Run `cargo test --offline --all-targets` and `cargo check --offline --no-default-features`.

This is style alignment, not a pixel-perfect screenshot certification. Host font availability, text rasterization, content, layout constraints and focus treatment can produce differences. Inspect `cargo run --example showcase` in both themes for application-level visual review. The style work does not add missing interaction implementations to desktop extension components.
