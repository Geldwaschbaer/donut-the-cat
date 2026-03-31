use crate::action::Action;

pub struct Dialog {
    title: &'static str,
    dialogs: Vec<DialogBox>,
}

impl Dialog {
    pub fn new(title: &'static str, dialogs: Vec<DialogBox>) -> Dialog {
        Dialog { title, dialogs }
    }

    pub fn get_title(&self) -> &'static str {
        self.title
    }

    pub fn get_dialogs(&self) -> &Vec<DialogBox> {
        &self.dialogs
    }
}

pub struct DialogBox {
    description: &'static str,
    options: Vec<DialogOption>,
}

impl DialogBox {
    pub fn new(description: &'static str, options: Vec<DialogOption>) -> DialogBox {
        DialogBox {
            description,
            options,
        }
    }

    pub fn get_description(&self) -> &'static str {
        &self.description
    }

    pub fn get_options(&self) -> &Vec<DialogOption> {
        &self.options
    }
}

pub struct DialogOption {
    description: &'static str,
    action: Box<dyn Action>,
    next: usize,
}

impl DialogOption {
    pub fn new(description: &'static str, action: Box<dyn Action>, next: usize) -> DialogOption {
        DialogOption {
            description,
            action,
            next,
        }
    }

    pub fn get_description(&self) -> &'static str {
        self.description
    }

    pub fn get_action(&self) -> &Box<dyn Action> {
        &self.action
    }

    pub fn get_next(&self) -> usize {
        self.next
    }
}
