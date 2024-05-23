use iced::Element;
use iced::widget::{column, text};

use crate::application::AlbatrosMessage;

#[derive(Clone, Debug, Default)]
pub struct Settings {
    name: String,
    value: u32,
}

impl Settings {
    pub fn view(self: &Self) -> Element<AlbatrosMessage> {
        // column!(
        //     text("hello world")
        // ).into()
        text(format!("hello from settings: {:?}", self)).into()
    }
}
