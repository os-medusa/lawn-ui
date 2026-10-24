pub use gpui;
pub use gpui_base as base;
pub use gpui_base::StyledExt;
pub use gpui_kit_assets as assets;
pub use gpui_kit_assets::icon_assets;
pub use gpui_platform::application;

#[cfg(feature = "chat")]
pub mod chat;
pub mod forms;
pub mod icon;
pub mod layout;
pub mod overlay;
pub mod primitives;
pub mod settings;
pub mod theme;

// Submodule aliases for direct path compatibility
pub mod accordion {
    pub use crate::primitives::accordion::*;
}
pub mod alert {
    pub use crate::primitives::alert::*;
}
#[cfg(feature = "chat")]
pub mod attachment {
    pub use crate::chat::attachment::*;
}
pub mod avatar {
    pub use crate::primitives::avatar::*;
}
pub mod badge {
    pub use crate::primitives::badge::*;
}
pub mod breadcrumb {
    pub use crate::primitives::breadcrumb::*;
}
#[cfg(feature = "chat")]
pub mod bubble {
    pub use crate::chat::bubble::*;
}
pub mod button {
    pub use crate::primitives::button::*;
}
pub mod calendar {
    pub use crate::primitives::calendar::*;
}
pub mod card {
    pub use crate::primitives::card::*;
}
pub mod checkbox {
    pub use crate::primitives::checkbox::*;
}
pub mod dialog {
    pub use crate::overlay::alert_dialog::*;
    pub use crate::overlay::dialog::*;
}
pub mod empty {
    pub use crate::primitives::empty::*;
}
pub mod form {
    pub use crate::forms::field::*;
}
pub mod input {
    pub use crate::forms::input::*;
    pub use crate::forms::textarea::*;
}
pub mod item {
    pub use crate::primitives::item::*;
}
pub mod menu {
    pub use crate::overlay::menu::*;
}
#[cfg(feature = "chat")]
pub mod message {
    pub use crate::chat::message::*;
}
#[cfg(feature = "chat")]
pub mod message_scroller {
    pub use crate::chat::message_scroller::*;
}
pub mod combobox {
    pub use crate::forms::combobox::*;
}
pub mod date_picker {
    pub use crate::forms::date_picker::*;
}
pub mod number_input {
    pub use crate::forms::number_input::*;
}
pub mod otp_input {
    pub use crate::forms::otp_input::*;
}
pub mod pagination {
    pub use crate::primitives::pagination::*;
}
pub mod popover {
    pub use crate::overlay::popover::*;
}
pub mod radio {
    pub use crate::primitives::radio::*;
}
pub mod searchable_list {
    pub use crate::primitives::traits::SearchableListItem;
}
pub mod select {
    pub use crate::forms::select::*;
}
pub mod separator {
    pub use crate::primitives::separator::*;
}
pub mod setting {
    pub use crate::settings::*;
}
pub mod sheet {
    pub use crate::overlay::sheet::*;
}
pub mod sidebar {
    pub use crate::layout::sidebar::*;
}
pub mod skeleton {
    pub use crate::primitives::skeleton::*;
}
pub mod slider {
    pub use crate::primitives::slider::*;
}
pub mod switch {
    pub use crate::primitives::switch::*;
}
pub mod table {
    pub use crate::primitives::table::*;
}
pub mod tabs {
    pub use crate::layout::tabs::*;
}
pub mod toast {
    pub use crate::overlay::toast::*;
}
pub mod toggle {
    pub use crate::primitives::toggle::*;
}
pub mod tooltip {
    pub use crate::overlay::tooltip::*;
}
pub mod tree {
    pub use crate::primitives::tree::*;
}

#[cfg(feature = "chat")]
pub use chat::{
    Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentTitle, Bubble,
    BubbleVariant, Message, MessageAlignment, MessageContent, MessageFooter, MessageHeader,
    MessageScroller, MessageScrollerState,
};
pub use forms::{
    Combobox, ComboboxItem, DatePicker, Field, Input, InputEvent, InputState, NumberInput,
    OtpInput, Select, SelectItem, Textarea, TextareaState,
};
pub use icon::{Icon, IconName, IconNamed, IconSize};
pub use layout::{
    ResizablePanel, ResizablePanelGroup, ResizeHandle, ResizeHandleContext, ResizeHandleRenderer,
    ResizeHandleState, ScrollArea, Sidebar, SidebarContent, SidebarFooter, SidebarGroup,
    SidebarGroupLabel, SidebarHeader, SidebarMenuButton, Tabs, TabsContent, TabsList, TabsTrigger,
    TitleBar, center, h_flex, h_resizable, resizable_panel, v_flex, v_resizable,
};
pub use overlay::{
    AlertDialog, ContextMenuExt, Dialog, DialogButtonProps, DialogContent, DialogDescription,
    DialogFooter, DialogHeader, DialogTitle, DropdownMenu, Menu, Notification, Popover,
    PopupMenuItem, Root, Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader,
    SheetSide, SheetTitle, Toast, ToastVariant, Tooltip, WindowExt,
};
pub use primitives::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Alert, AlertVariant, Avatar,
    AvatarSize, Badge, BadgeVariant, Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList,
    BreadcrumbPage, BreadcrumbSeparator, Button, ButtonGroup, ButtonSize, ButtonVariant,
    ButtonVariants, Calendar, Card, CardContent, CardDescription, CardFooter, CardHeader,
    CardTitle, Checkbox, Disableable, Empty, EmptyDescription, EmptyHeader, EmptyMedia,
    EmptyMediaVariant, EmptyTitle, Item, MetricBox, Orientation, Pagination, PresenceStatus,
    Progress, Radio, RadioGroup, SearchableListItem, Selectable, Separator, Side, Sizable, Size,
    Skeleton, Slider, Switch, Table, TableBody, TableCaption, TableCell, TableHead, TableHeader,
    TableRow, ThemeMode, Toggle, ToggleGroup, ToggleSize, ToggleVariant, Tree, TreeNode,
};
pub use settings::{
    NumberFieldOptions, SelectIndex, SettingField, SettingGroup, SettingItem, SettingPage, Settings,
};
pub use theme::{AccentTheme, ActiveTheme, Theme, ThemeAppearance, ThemeColors, ThemeRadius};

/// Initializes `lawn-ui` and the underlying `gpui-base` headless runtime.
pub fn init(cx: &mut gpui::App) {
    // 1. Initialize gpui-base headless infrastructure
    gpui_base::init(cx);

    // 2. Install lawn-ui Theme as Global if not already installed
    if !cx.has_global::<Theme>() {
        Theme::set_global(cx, Theme::dark());
    } else {
        // 3. Project theme tokens onto gpui-base
        Theme::sync_base(cx);
    }
    overlay::root::init(cx);
}
