pub mod resizable;
pub mod scroll_area;
pub mod sidebar;
pub mod tabs;
pub mod title_bar;

pub use resizable::{
    ResizablePanel, ResizablePanelGroup, ResizeHandle, ResizeHandleContext, ResizeHandleRenderer,
    ResizeHandleState, h_resizable, resizable_panel, v_resizable,
};
pub use scroll_area::ScrollArea;
pub use sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader,
    SidebarMenuButton,
};
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
pub use title_bar::TitleBar;

use gpui::{Div, Styled, div};

/// Creates a horizontal flexbox container (`flex flex-row items-center`).
#[inline]
pub fn h_flex() -> Div {
    div().flex().flex_row().items_center()
}

/// Creates a vertical flexbox container (`flex flex-col`).
#[inline]
pub fn v_flex() -> Div {
    div().flex().flex_col()
}

/// Creates a centered flexbox container (`flex items-center justify-center`).
#[inline]
pub fn center() -> Div {
    div().flex().items_center().justify_center()
}
