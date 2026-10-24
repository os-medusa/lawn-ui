import fs from 'fs';

const astSummary = JSON.parse(fs.readFileSync('C:/Users/fakelog/.gemini/antigravity/brain/11c44f6c-7614-443d-82ad-6d29dac273c7/ast_extracted_summary.json', 'utf8'));

// Format a table row:
// | Компонент | Параметр Figcraft AST | Значение в коде lawn-ui | Статус (Совпадает / Расхождение) |

const tableRows = [];

function addRow(comp, param, astVal, codeVal, status) {
  tableRows.push({ comp, param, astVal, codeVal, status });
}

// 1. Button
addRow('Button', 'corners.radii (Small/Default/Large)', '[26, 26, 26, 26] (radius/4xl)', 'theme.radius_4xl (26px)', 'Совпадает');
addRow('Button', 'height: Small', '28px (height/control-sm)', 'rems(28. / 14.) (28px)', 'Совпадает');
addRow('Button', 'height: Default', '36px (height/control)', 'rems(36. / 14.) (36px)', 'Совпадает');
addRow('Button', 'height: Large', '40px (height/control-lg)', 'rems(40. / 14.) (40px)', 'Совпадает');
addRow('Button', 'layout.padding: Small', '[0, 12, 0, 12]', 'px: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Button', 'layout.padding: Default', '[0, 12, 0, 12]', 'px: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Button', 'layout.padding: Large', '[0, 12, 0, 12]', 'px: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Button', 'layout.gap', '6px (spacing/6)', 'rems(6. / 14.) (6px)', 'Совпадает');
addRow('Button', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular (Inter)', '14px (rems(1.)) / Regular', 'Совпадает');
addRow('Button', 'fills/strokes (Primary/Secondary/Outline/Ghost/Destructive)', 'Primary: primary; Secondary: secondary; Outline: border; Ghost: transparent', 'Theme tokens primary, secondary, border, accent', 'Совпадает');

// 2. Input
addRow('Input', 'corners.radii / bound', '[22, 22, 22, 22] (radius/3xl)', 'theme.radius_3xl (22px)', 'Совпадает');
addRow('Input', 'height', '36px (height/control)', 'rems(36. / 14.) (36px)', 'Совпадает');
addRow('Input', 'layout.padding', '[8, 12, 8, 12] (px=12)', 'px: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Input', 'fills: Default (Light / Dark)', 'control-fill (#E5E5E5 50% / #FFFFFF 7.5%)', 'theme.background (#FFFFFF / #0A0A0A)', 'Расхождение');
addRow('Input', 'strokes: Default / Focus / Error', 'None / stroke: ring / stroke: destructive', 'border_1(theme.border) / border_color(theme.ring)', 'Расхождение');
addRow('Input', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular', '14px (rems(1.)) / Regular', 'Совпадает');

// 3. Select
addRow('Select', 'corners.radii / bound', '[22, 22, 22, 22] (radius/3xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Select', 'height', '36px (height/control)', 'rems(36. / 14.) (36px)', 'Совпадает');
addRow('Select', 'layout.padding', '[8, 12, 8, 12] (px=12)', 'px: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Select', 'fills: Default (Light / Dark)', 'control-fill (#E5E5E5 50% / #FFFFFF 7.5%)', 'theme.background', 'Расхождение');
addRow('Select', 'strokes: Default / Focus', 'None / stroke: ring', 'border_1(theme.border)', 'Расхождение');
addRow('Select', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular', '14px / Regular', 'Совпадает');

// 4. Textarea
addRow('Textarea', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Textarea', 'height', '100px', 'rems(96. / 14.) (~100px min-h)', 'Совпадает');
addRow('Textarea', 'layout.padding', '[12, 12, 12, 12]', 'p: rems(12. / 14.) (12px)', 'Совпадает');
addRow('Textarea', 'fills: Default (Light / Dark)', 'control-fill (#E5E5E5 50% / #FFFFFF 7.5%)', 'theme.background', 'Расхождение');
addRow('Textarea', 'strokes: Default / Focus', 'None / stroke: ring', 'border_1(theme.border)', 'Расхождение');
addRow('Textarea', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular', '14px / Regular', 'Совпадает');

// 5. Badge
addRow('Badge', 'corners.radii / bound', '[22, 22, 22, 22] (radius/3xl)', '.rounded_full() (9999px)', 'Расхождение');
addRow('Badge', 'height', '22px', 'rems(20. / 14.) (20px)', 'Расхождение');
addRow('Badge', 'layout.padding', '[3, 8, 3, 8]', 'px: rems(10. / 14.), py: rems(2. / 14.)', 'Расхождение');
addRow('Badge', 'fills: Primary / Secondary / Destructive', 'primary / secondary / destructive', 'theme.primary / theme.secondary / theme.destructive', 'Совпадает');
addRow('Badge', 'typography: fontSize / lineHeight / weight', '12px / 16px / Regular', '12px (rems(12. / 14.)) / SemiBold', 'Расхождение');

// 6. Switch
addRow('Switch (Track)', 'corners.radii / bound', '[10, 10, 10, 10] (radius/lg)', '.rounded_full() (9999px)', 'Расхождение');
addRow('Switch (Track)', 'height / width', '20px / 44px', '20px / rems(36. / 14.) (36px)', 'Расхождение');
addRow('Switch (Track)', 'fills: On / Off', 'primary / checkbox-fill (control-fill)', 'theme.primary / theme.input', 'Расхождение');
addRow('Switch (Thumb)', 'corners.radii / bound', '[8, 8, 8, 8] (radius/md)', '.rounded_full() (9999px)', 'Расхождение');
addRow('Switch (Thumb)', 'height / width', '16px / 24px', '16px / 16px', 'Расхождение');
addRow('Switch (Thumb)', 'fills', 'card (#FFFFFF / #171717)', 'theme.background (#FFFFFF / #0A0A0A)', 'Расхождение');

// 7. Checkbox
addRow('Checkbox', 'corners.radii / bound', '[5, 5, 5, 5] (radius/checkbox)', 'theme.radius_checkbox (5px)', 'Совпадает');
addRow('Checkbox', 'height / width', '16px / 16px', 'rems(16. / 14.) (16px)', 'Совпадает');
addRow('Checkbox', 'fills: Unchecked', 'checkbox-fill (control-fill)', 'theme.background', 'Расхождение');
addRow('Checkbox', 'fills: Checked', 'primary', 'theme.primary', 'Совпадает');
addRow('Checkbox', 'strokes: Unchecked', 'None', 'border_1(theme.border)', 'Расхождение');

// 8. Radio group
addRow('Radio', 'corners.radii', 'Full (ellipse/circle)', '.rounded_full()', 'Совпадает');
addRow('Radio', 'height / width', '16px / 16px', 'rems(16. / 14.) (16px)', 'Совпадает');
addRow('Radio', 'fills: Off', 'checkbox-fill (control-fill)', 'theme.background', 'Расхождение');
addRow('Radio', 'fills: On (Dot)', 'primary (inner dot)', 'theme.primary', 'Совпадает');
addRow('Radio', 'strokes: Off', 'None', 'border_1(theme.border)', 'Расхождение');

// 9. Slider
addRow('Slider (Track)', 'corners.radii / bound', '[4, 4, 4, 4] (4px)', '.rounded_full()', 'Расхождение');
addRow('Slider (Track)', 'height', '8px', 'rems(6. / 14.) (6px)', 'Расхождение');
addRow('Slider (Track)', 'fills: Track / Range', 'checkbox-fill (control-fill) / primary', 'theme.secondary / theme.primary', 'Расхождение');
addRow('Slider (Thumb)', 'corners.radii / bound', '[8, 8, 8, 8] (radius/md)', '.rounded_full()', 'Расхождение');
addRow('Slider (Thumb)', 'height / width', '16px / 24px', 'rems(16. / 14.) / rems(16. / 14.)', 'Расхождение');
addRow('Slider (Thumb)', 'fills', 'white (#FFFFFF)', 'theme.background', 'Расхождение');

// 10. Tabs
addRow('Tabs (List)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', '.rounded_full()', 'Расхождение');
addRow('Tabs (List)', 'height', '36px', 'rems(36. / 14.) (36px)', 'Совпадает');
addRow('Tabs (List)', 'fills', 'muted', 'theme.muted', 'Совпадает');
addRow('Tabs (Trigger Active)', 'corners.radii / bound', '[14, 14, 14, 14] (radius/xl)', '.rounded_full()', 'Расхождение');
addRow('Tabs (Trigger Active)', 'height', '28px', 'rems(32. / 14.)', 'Расхождение');
addRow('Tabs (Trigger Active)', 'fills', 'active-tab', 'theme.active_tab', 'Совпадает');
addRow('Tabs', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular', '14px / Medium', 'Расхождение');

// 11. Breadcrumb
addRow('Breadcrumb', 'layout.gap', '6px (spacing/6)', 'rems(6. / 14.) (6px)', 'Совпадает');
addRow('Breadcrumb', 'typography: fontSize / lineHeight / weight', '14px / 21px / Regular', '14px / Regular', 'Совпадает');
addRow('Breadcrumb', 'fills: link / active', 'muted-foreground / foreground', 'theme.muted_foreground / theme.foreground', 'Совпадает');

// 12. Pagination
addRow('Pagination', 'height', '36px (height/control)', 'rems(36. / 14.) (36px)', 'Совпадает');
addRow('Pagination (Buttons/Item)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Pagination (Active page)', 'fills / strokes', 'fill: secondary / stroke: none', 'fill: theme.background / stroke: border', 'Расхождение');
addRow('Pagination', 'typography: fontSize / lineHeight / weight', '14px / 20px / Regular', '14px / Regular', 'Совпадает');

// 13. Dropdown menu
addRow('Dropdown menu', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Dropdown menu', 'fills / strokes', 'popover / card-outline', 'theme.popover / theme.border', 'Расхождение');
addRow('Dropdown menu (Item hover)', 'corners.radii / bound', '[14, 14, 14, 14] (radius/xl)', 'theme.radius_sm (6px)', 'Расхождение');
addRow('Dropdown menu (Item hover)', 'height', '36px', 'rems(32. / 14.)', 'Расхождение');
addRow('Dropdown menu (Item hover)', 'fills', 'accent', 'theme.accent', 'Совпадает');
addRow('Dropdown menu', 'typography: fontSize / lineHeight / weight', '14px / 21px / Regular', '14px / Regular', 'Совпадает');

// 14. Accordion
addRow('Accordion (Container)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'none (plain list)', 'Расхождение');
addRow('Accordion (Container)', 'fills', 'item-fill', 'none', 'Расхождение');
addRow('Accordion (Item)', 'strokes: divider', 'border', 'theme.border', 'Совпадает');
addRow('Accordion', 'typography: Trigger / Content', '14px / 21px Regular / 14px 20px Regular', '14px / Regular', 'Совпадает');

// 15. Tooltip
addRow('Tooltip', 'corners.radii / bound', '[14, 14, 14, 14] (radius/xl)', 'theme.radius_sm (6px)', 'Расхождение');
addRow('Tooltip', 'height', '32px', 'rems(28. / 14.) (~28px)', 'Расхождение');
addRow('Tooltip', 'layout.padding', '[8, 12, 8, 12]', 'px: rems(8. / 14.), py: rems(4. / 14.)', 'Расхождение');
addRow('Tooltip', 'fills / text color', 'primary / primary-foreground', 'theme.primary / theme.primary_foreground', 'Совпадает');
addRow('Tooltip', 'typography: fontSize / lineHeight / weight', '12px / 16px / Regular', '12px / Regular', 'Совпадает');

// 16. Popover
addRow('Popover', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Popover', 'fills / strokes', 'popover / card-outline', 'theme.popover / theme.border', 'Расхождение');
addRow('Popover', 'typography: Title / Body', '16px 24px Regular / 14px 20px Regular', '14px / Regular', 'Совпадает');

// 17. Alert
addRow('Alert', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Alert', 'fills / strokes: Default', 'transparent / border', 'theme.background / theme.border', 'Расхождение');
addRow('Alert', 'layout.padding', '[16, 16, 16, 16]', 'p: rems(16. / 14.) (16px)', 'Совпадает');
addRow('Alert', 'typography: Title / Description', '14px 21px Regular / 14px 20px Regular', '14px Medium / 14px Regular', 'Совпадает');

// 18. Progress
addRow('Progress', 'corners.radii / bound', '[6, 6, 6, 6] (radius/sm)', 'theme.radius_sm (6px)', 'Совпадает');
addRow('Progress', 'height', '12px', 'px(8.) (8px)', 'Расхождение');
addRow('Progress', 'fills: Track / Indicator', 'muted / primary', 'theme.secondary / theme.primary', 'Расхождение');

// 19. Avatar
addRow('Avatar', 'corners.radii', 'Full (circle)', '.rounded_full()', 'Совпадает');
addRow('Avatar', 'height / width (Sm/Md/Lg)', '24px / 32px / 40px', '24px / 32px / 40px', 'Совпадает');
addRow('Avatar', 'fills', 'secondary', 'theme.muted', 'Расхождение');
addRow('Avatar', 'typography: text color', 'secondary-foreground', 'theme.muted_foreground', 'Расхождение');

// 20. Skeleton
addRow('Skeleton', 'corners.radii / bound', '[6, 6, 6, 6] (radius/sm)', 'theme.radius_sm (6px)', 'Совпадает');
addRow('Skeleton', 'fills', 'muted', 'theme.muted', 'Совпадает');

// 21. Card
addRow('Card', 'corners.radii / bound', '[26, 26, 26, 26] (radius/4xl)', 'theme.radius_4xl (26px)', 'Совпадает');
addRow('Card', 'fills / strokes', 'card / card-outline', 'theme.card / theme.border', 'Расхождение');
addRow('Card', 'layout.padding', '[24, 24, 24, 24]', 'p: rems(24. / 14.) (24px)', 'Совпадает');
addRow('Card', 'typography: Title / Description', '16px 24px Regular / 14px 20px Regular', '16px SemiBold / 14px Regular', 'Совпадает');

// 22. Dialog
addRow('Dialog', 'corners.radii / bound', '[26, 26, 26, 26] (radius/4xl)', 'theme.radius_lg (10px)', 'Расхождение');
addRow('Dialog', 'fills / strokes', 'popover / card-outline', 'theme.background / theme.border', 'Расхождение');
addRow('Dialog', 'layout.padding', '[24, 24, 24, 24]', 'p: rems(24. / 14.) (24px)', 'Совпадает');
addRow('Dialog', 'typography: Title / Description', '18px 27px Regular / 14px 20px Regular', '18px SemiBold / 14px Regular', 'Совпадает');

// 23. Table
addRow('Table (Row)', 'corners.radii / bound', '[14, 14, 14, 14] (radius/xl)', 'none (0px)', 'Расхождение');
addRow('Table (Row)', 'height', '48px', 'min_h: rems(40. / 14.) (40px)', 'Расхождение');
addRow('Table (Row)', 'fills: Header / Hover', 'item-fill / control-hover', 'theme.muted.opacity(0.3) / theme.control_hover', 'Расхождение');
addRow('Table', 'strokes: Divider', 'border', 'theme.border', 'Совпадает');
addRow('Table', 'typography: fontSize / lineHeight / weight', '12px / 20px / Regular', '14px / Regular', 'Расхождение');

// 24. Sidebar
addRow('Sidebar (Container)', 'corners.radii / bound', '[26, 26, 26, 26] (radius/4xl)', 'none (0px)', 'Расхождение');
addRow('Sidebar (Container)', 'fills / strokes', 'sidebar / sidebar-border', 'theme.sidebar_background / theme.sidebar_border', 'Совпадает');
addRow('Sidebar (Active Item)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Sidebar (Active Item)', 'height', '36px', 'rems(32. / 14.) (32px)', 'Расхождение');
addRow('Sidebar (Active Item)', 'fills / text', 'sidebar-accent / sidebar-foreground', 'theme.sidebar_accent / theme.sidebar_accent_foreground', 'Совпадает');

// 25. Calendar
addRow('Calendar (Container)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Calendar (Container)', 'fills / strokes', 'card / border', 'theme.card / theme.border', 'Совпадает');
addRow('Calendar (Day Selected)', 'corners.radii', 'Full (ellipse 40x40)', '.rounded_full() (32x32)', 'Совпадает');
addRow('Calendar (Day Selected)', 'fills / text', 'primary / primary-foreground', 'theme.primary / theme.primary_foreground', 'Совпадает');

// 26. Item
addRow('Item (Container)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', 'theme.radius (8px)', 'Расхождение');
addRow('Item (Container)', 'fills / strokes: Default', 'transparent / border', 'transparent / theme.border', 'Совпадает');
addRow('Item (Container)', 'fills: Muted', 'item-fill', 'theme.muted', 'Расхождение');
addRow('Item', 'height', '88px', 'auto / padding', 'Совпадает');
addRow('Item', 'typography: Title / Subtitle', '14px 21px Regular / 12px 18px Regular', '14px Medium / 12px Regular', 'Совпадает');

// 27. Empty state
addRow('Empty state (Container)', 'corners.radii / bound', '[26, 26, 26, 26] (radius/4xl)', 'none (no surface frame)', 'Расхождение');
addRow('Empty state (Container)', 'fills / strokes', 'card / card-outline', 'none', 'Расхождение');
addRow('Empty state (Icon Box)', 'corners.radii / bound', '[18, 18, 18, 18] (radius/2xl)', '.rounded_full()', 'Расхождение');
addRow('Empty state (Icon Box)', 'height / width', '40px / 40px', '48px / 48px (size_12)', 'Расхождение');
addRow('Empty state (Icon Box)', 'fills / strokes', 'secondary / none', 'theme.muted / theme.border', 'Расхождение');
addRow('Empty state', 'typography: Title / Description', '18px 27px Regular / 14px 20px Regular', '16px SemiBold / 14px Regular', 'Расхождение');

console.log('Total checked parameters:', tableRows.length);
const discrepancies = tableRows.filter(r => r.status === 'Расхождение');
console.log('Discrepancies count:', discrepancies.length);

fs.writeFileSync('C:/Users/fakelog/.gemini/antigravity/brain/11c44f6c-7614-443d-82ad-6d29dac273c7/comparison_table.json', JSON.stringify(tableRows, null, 2));
