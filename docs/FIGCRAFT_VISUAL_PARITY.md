# Figcraft Design System Visual Parity Specification

This document details the visual alignment, design tokens, geometry, and styling parity between the **Figcraft Design System** (`glyph-ui_design_system.figcraft` / Luma Neutral collection) and the [`lawn-ui`](https://github.com/os-medusa/lawn-ui) Rust GPUI implementation.

---

## 1. Design Tokens & Foundations (Luma Neutral)

### Control Heights (`ThemeMetrics`)
* **SM**: `28px` (`rems(28./14.)` = 2.0rem)
* **MD (Default)**: `36px` (`rems(36./14.)` = 2.571rem)
* **LG**: `40px` (`rems(40./14.)` = 2.857rem)

### Radius Scale (`ThemeRadius`)
* `none`: `0px`
* `radius/checkbox`: `5px`
* `radius/sm`: `6px`
* `radius/md`: `8px` (Standard controls, input fields, buttons, and cards)
* `radius/lg`: `10px` (Luma base radius `0.625rem`)
* `radius/xl`: `14px` (Dialogs & modal sheets)
* `radius/2xl`: `18px`
* `radius/3xl`: `22px`
* `radius/4xl`: `26px`
* `radius/pill` / `full`: `999px` / `9999px` (Badges, switches, avatar chips)

### Typography
* **Font Family**: `Inter`
* **Baseline**: 14px (`rems(px / 14.)`)
* **Weights**: Regular (400), Medium (500), SemiBold (600), Bold (700)
* **Scale**: 12px, 14px, 16px, 18px, 20px, 24px, 30px, 36px, 48px

### Semantic Palette (Light / Dark)
| Token | Light Mode (Hex / HSL) | Dark Mode (Hex / HSL) | Usage |
|---|---|---|---|
| `background` | `#FFFFFF` | `#0A0A0A` | Base app background |
| `foreground` | `#0A0A0A` | `#FAFAFA` | Primary body typography |
| `card` | `#FFFFFF` | `#171717` | Card & surface container |
| `card_foreground` | `#0A0A0A` | `#FAFAFA` | Card headings and copy |
| `popover` | `#FFFFFF` | `#171717` | Menus, tooltips, popovers |
| `primary` | `#171717` | `#E5E5E5` | High-emphasis actions & active states |
| `primary_foreground` | `#FAFAFA` | `#171717` | Text on primary elements |
| `secondary` | `#F5F5F5` | `#262626` | Secondary buttons and chips |
| `secondary_foreground` | `#171717` | `#FAFAFA` | Secondary label text |
| `muted` | `#F5F5F5` | `#262626` | Subtle containers & tabs track |
| `muted_foreground` | `#737373` | `#A3A3A3` | Subtitles, hints, icons |
| `border` / `input` | `#E5E5E5` | `rgba(255, 255, 255, 0.1)` | Surface borders and input strokes |
| `ring` | `rgba(10, 10, 10, 0.24)` | `rgba(255, 255, 255, 0.24)` | Accessible focus rings |
| `destructive` | `#E7000B` | `#FF6467` | Errors, danger actions, delete alerts |

---

## 2. Component Visual Specification Matrix (28 Core Components)

### Controls & Text Inputs
* **Button** (`src/primitives/button.rs`):
  * Heights: 28px (SM), 36px (MD), 40px (LG).
  * Variants: Primary, Secondary, Outline, Ghost, Destructive, Link.
  * Border radii: 6px (SM), 8px (MD), 10px (LG).
  * States: Default, Hover (with `control_hover`), Active, Disabled (`opacity: 0.5`).
* **Input** (`src/forms/input.rs`):
  * Height: 36px, radius 8px (`md`), 1px border `#E5E5E5`.
  * States: Default, Filled, Focus (2px ring), Error (`destructive`), Disabled.
* **Select** (`src/forms/select.rs`):
  * Trigger: 36px, chevron trailing icon (16x16).
  * Popover Menu: 8px radius, border 1px, shadow-md, item hover `accent`.
* **Textarea** (`src/forms/textarea.rs`):
  * Default height: 100px, radius 8px, 1px border.

### Toggles, Sliders & Selection
* **Checkbox** (`src/primitives/checkbox.rs`):
  * Dimensions: 16x16px (32px touch bounding box).
  * Corner radius: **5px** (`radius_checkbox`).
  * States: Unchecked, Checked (filled `#171717` with white checkmark), Indeterminate, Disabled.
* **Switch** (`src/primitives/switch.rs`):
  * Track: 44x20px, rounded-full.
  * Thumb: 16x16px (or 24x16 pill thumb), animated translation.
  * Fills: Off `#E5E5E5`, On `#171717`.
* **Radio & RadioGroup** (`src/primitives/radio.rs`):
  * Indicator: 16x16px circle, 1px border.
  * Dot: 8x8px inner circle filled with `primary`.
* **Slider** (`src/primitives/slider.rs`):
  * Track height: 6-8px with `primary` range fill.
  * Thumb: 16x16px circle with white stroke and elevation shadow.
* **Progress** (`src/primitives/progress.rs`):
  * Track height: 8-12px, rounded-full.

### Navigation & Data Display
* **Tabs** (`src/primitives/tabs.rs`):
  * Container height: 36px, radius 8px, background `muted`.
  * Active pill: white surface with subtle shadow (`active_tab`).
* **Breadcrumb** (`src/primitives/breadcrumb.rs`):
  * Height: 36px, separator chevron 16x16, muted text with active foreground end.
* **Pagination** (`src/primitives/pagination.rs`):
  * Controls height: 36px, active page box 36x36 with primary outline/fill.
* **Table** (`src/primitives/table.rs`):
  * Header height: 40-48px, medium muted labels.
  * Rows: 48px height, 1px border-bottom, hover highlight.
* **Accordion** (`src/primitives/accordion.rs`):
  * Row height: 48px with rotating trailing chevron (16x16).
* **Calendar** (`src/primitives/calendar.rs`):
  * Month navigation header, 7-column day grid, 36x36 rounded-full cells, selected day filled `primary`.
* **Sidebar** (`src/navigation/sidebar.rs`):
  * Items: 36px height, 16x16 leading icon, active item accent fill.

### Overlays, Surfaces & Feedback
* **Card** (`src/surfaces/card.rs`):
  * Radius: 10px (`radius/lg`), 1px border, padding 16-24px.
* **Item** (`src/primitives/item.rs`):
  * List card item with 24x24 leading icon box, title, description, and chevron.
* **Alert** (`src/primitives/alert.rs`):
  * Radius: 8px, 1px border, leading icon, title & description, optional dismiss button.
* **Dialog** (`src/surfaces/modal.rs`):
  * Radius: 14px (`radius/xl`), backdrop overlay, top-right close button.
* **Popover & Dropdown Menu** (`src/surfaces/popover.rs`):
  * Radius: 8px, shadow-md, 1px border.
* **Tooltip** (`src/surfaces/tooltip.rs`):
  * Inverse surface (`#171717` in light mode), 12px text, 6px radius.
* **Badge** (`src/primitives/badge.rs`):
  * Height: 22px, radius 999px (pill), 12px semibold text.
* **Avatar** (`src/primitives/avatar.rs`):
  * Sizes: 24px, 32px, 40px, rounded-full with status indicator dot.
* **Skeleton** (`src/primitives/skeleton.rs`):
  * Shimmer / pulse animation with muted background.
* **Empty State** (`src/primitives/empty_state.rs`):
  * Centered layout with illustration, heading, description, and action button.
* **Lucide Icons**:
  * Embedded vector set matching Figcraft's Lucide 16x16 & 20x20 icons.
