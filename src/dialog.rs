use crate::event::{Event, EventBuilder};
use async_from::{AsyncFrom, async_trait};
use serde::Deserialize;

#[derive(Clone)]
pub struct Dialog {
    title: String,
    dialogs: Vec<DialogBox>,
}

impl Dialog {
    pub fn new(title: String, dialogs: Vec<DialogBox>) -> Dialog {
        Dialog { title, dialogs }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_dialogs(&self) -> &Vec<DialogBox> {
        &self.dialogs
    }
}

#[derive(Deserialize)]
pub struct DialogBuilder {
    title: String,
    dialogs: Vec<DialogBoxBuilder>,
}

#[async_trait]
impl AsyncFrom<DialogBuilder> for Dialog {
    async fn async_from(value: DialogBuilder) -> Self {
        let mut dialogs = Vec::new();
        for dialog_builder in value.dialogs {
            dialogs.push(DialogBox::async_from(dialog_builder).await);
        }
        Dialog {
            title: value.title,
            dialogs,
        }
    }
}

#[derive(Clone)]
pub struct DialogBox {
    description: String,
    options: Vec<DialogOption>,
}

impl DialogBox {
    pub fn new(description: String, options: Vec<DialogOption>) -> DialogBox {
        DialogBox {
            description,
            options,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_options(&self) -> &Vec<DialogOption> {
        &self.options
    }
}

#[derive(Deserialize)]
pub struct DialogBoxBuilder {
    description: String,
    options: Vec<DialogOptionBuilder>,
}

#[async_trait]
impl AsyncFrom<DialogBoxBuilder> for DialogBox {
    async fn async_from(value: DialogBoxBuilder) -> Self {
        let mut options = Vec::new();
        for option_builder in value.options {
            options.push(DialogOption::async_from(option_builder).await);
        }
        DialogBox {
            description: value.description,
            options,
        }
    }
}

#[derive(Clone)]
pub struct DialogOption {
    description: String,
    events: Vec<Event>,
    next: usize,
}

impl DialogOption {
    pub fn new(description: String, events: Vec<Event>, next: usize) -> DialogOption {
        DialogOption {
            description,
            events,
            next,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_next(&self) -> usize {
        self.next
    }
}

#[derive(Deserialize)]
pub struct DialogOptionBuilder {
    description: String,
    #[serde(default = "Vec::new")]
    events: Vec<EventBuilder>,
    next: usize,
}

#[async_trait]
impl AsyncFrom<DialogOptionBuilder> for DialogOption {
    async fn async_from(value: DialogOptionBuilder) -> Self {
        let mut events = Vec::new();
        for event_builder in value.events {
            events.push(Event::async_from(event_builder).await);
        }
        DialogOption {
            description: value.description,
            events,
            next: value.next,
        }
    }
}
