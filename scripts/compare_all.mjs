import fs from 'fs';

// Read AST summary
const ast = JSON.parse(fs.readFileSync('C:/Users/fakelog/.gemini/antigravity/brain/11c44f6c-7614-443d-82ad-6d29dac273c7/ast_extracted_summary.json', 'utf8'));

// Read rust files
const files = {
  Button: fs.readFileSync('D:/lawn-ui/src/primitives/button.rs', 'utf8'),
  Input: fs.readFileSync('D:/lawn-ui/src/forms/input.rs', 'utf8'),
  Select: fs.readFileSync('D:/lawn-ui/src/forms/select.rs', 'utf8'),
  Textarea: fs.readFileSync('D:/lawn-ui/src/forms/textarea.rs', 'utf8'),
  Badge: fs.readFileSync('D:/lawn-ui/src/primitives/badge.rs', 'utf8'),
  Switch: fs.readFileSync('D:/lawn-ui/src/primitives/switch.rs', 'utf8'),
  Checkbox: fs.readFileSync('D:/lawn-ui/src/primitives/checkbox.rs', 'utf8'),
  Radio: fs.readFileSync('D:/lawn-ui/src/primitives/radio.rs', 'utf8'),
  Slider: fs.readFileSync('D:/lawn-ui/src/primitives/slider.rs', 'utf8'),
  Tabs: fs.readFileSync('D:/lawn-ui/src/layout/tabs.rs', 'utf8'),
  Breadcrumb: fs.readFileSync('D:/lawn-ui/src/primitives/breadcrumb.rs', 'utf8'),
  Pagination: fs.readFileSync('D:/lawn-ui/src/primitives/pagination.rs', 'utf8'),
  DropdownMenu: fs.readFileSync('D:/lawn-ui/src/overlay/menu.rs', 'utf8'),
  Accordion: fs.readFileSync('D:/lawn-ui/src/primitives/accordion.rs', 'utf8'),
  Tooltip: fs.readFileSync('D:/lawn-ui/src/overlay/tooltip.rs', 'utf8'),
  Popover: fs.readFileSync('D:/lawn-ui/src/overlay/popover.rs', 'utf8'),
  Alert: fs.readFileSync('D:/lawn-ui/src/primitives/alert.rs', 'utf8'),
  Progress: fs.readFileSync('D:/lawn-ui/src/primitives/progress.rs', 'utf8'),
  Avatar: fs.readFileSync('D:/lawn-ui/src/primitives/avatar.rs', 'utf8'),
  Skeleton: fs.readFileSync('D:/lawn-ui/src/primitives/skeleton.rs', 'utf8'),
  Card: fs.readFileSync('D:/lawn-ui/src/primitives/card.rs', 'utf8'),
  Dialog: fs.readFileSync('D:/lawn-ui/src/overlay/dialog.rs', 'utf8'),
  Table: fs.readFileSync('D:/lawn-ui/src/primitives/table.rs', 'utf8'),
  Sidebar: fs.readFileSync('D:/lawn-ui/src/layout/sidebar.rs', 'utf8'),
  Calendar: fs.readFileSync('D:/lawn-ui/src/primitives/calendar.rs', 'utf8'),
  Item: fs.readFileSync('D:/lawn-ui/src/primitives/item.rs', 'utf8'),
  Empty: fs.readFileSync('D:/lawn-ui/src/primitives/empty.rs', 'utf8')
};

console.log('Loaded all rust files.');
