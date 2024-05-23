use iced::Element;
use iced::widget::{column, text};

use crate::Message;

#[derive(Clone, Debug, Default)]
pub struct Settings {
    name: String,
    value: u32,
}

impl Settings {
    pub fn view(self: &Self) -> Element<Message> {
        // column!(
        //     text("hello world")
        // ).into()
        text(format!("hello from settings: {:?}", self)).into()
    }
}
