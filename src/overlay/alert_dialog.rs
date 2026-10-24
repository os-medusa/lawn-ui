use std::rc::Rc;

use gpui::{Context, SharedString, Window};

use crate::primitives::button::ButtonVariant;

pub(super) type AlertDialogHandler = Rc<
    dyn Fn(&mut AlertDialog, &mut Window, &mut Context<crate::overlay::root::Root>) -> bool
        + 'static,
>;

#[derive(Clone, Default)]
pub struct DialogButtonProps {
    pub(super) show_cancel: bool,
    pub(super) ok_text: Option<SharedString>,
    pub(super) ok_variant: Option<ButtonVariant>,
    pub(super) cancel_text: Option<SharedString>,
}

impl DialogButtonProps {
    pub fn show_cancel(mut self, show: bool) -> Self {
        self.show_cancel = show;
        self
    }

    pub fn ok_text(mut self, text: impl Into<SharedString>) -> Self {
        self.ok_text = Some(text.into());
        self
    }

    pub fn ok_variant(mut self, variant: ButtonVariant) -> Self {
        self.ok_variant = Some(variant);
        self
    }

    pub fn cancel_text(mut self, text: impl Into<SharedString>) -> Self {
        self.cancel_text = Some(text.into());
        self
    }
}

pub struct AlertDialog {
    pub(super) title: Option<SharedString>,
    pub(super) description: Option<SharedString>,
    pub(super) button_props: DialogButtonProps,
    pub(super) on_ok: Option<AlertDialogHandler>,
    pub(super) on_cancel: Option<AlertDialogHandler>,
}

impl Default for AlertDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertDialog {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            button_props: DialogButtonProps {
                show_cancel: true,
                ok_text: Some("OK".into()),
                ok_variant: Some(ButtonVariant::Default),
                cancel_text: Some("Cancel".into()),
            },
            on_ok: None,
            on_cancel: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn button_props(mut self, props: DialogButtonProps) -> Self {
        self.button_props = props;
        self
    }

    pub fn show_cancel(mut self, show: bool) -> Self {
        self.button_props.show_cancel = show;
        self
    }

    pub fn confirm(self) -> Self {
        self.show_cancel(true)
    }

    pub fn ok_text(mut self, text: impl Into<SharedString>) -> Self {
        self.button_props.ok_text = Some(text.into());
        self
    }

    pub fn ok_variant(mut self, variant: ButtonVariant) -> Self {
        self.button_props.ok_variant = Some(variant);
        self
    }

    pub fn cancel_text(mut self, text: impl Into<SharedString>) -> Self {
        self.button_props.cancel_text = Some(text.into());
        self
    }

    pub fn on_ok<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut AlertDialog, &mut Window, &mut Context<crate::overlay::root::Root>) -> bool
            + 'static,
    {
        self.on_ok = Some(Rc::new(callback));
        self
    }

    pub fn on_cancel<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut AlertDialog, &mut Window, &mut Context<crate::overlay::root::Root>) -> bool
            + 'static,
    {
        self.on_cancel = Some(Rc::new(callback));
        self
    }
}
