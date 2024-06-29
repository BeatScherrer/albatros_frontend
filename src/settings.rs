use iced::widget::center;
use iced::widget::column;
use iced::widget::text;
use iced::{Element, Length};

use crate::application::AlbatrosMessage;

#[derive(Clone, Debug, Default)]
pub struct Settings {
    name: String,
    value: u32,
}

impl Settings {
    pub fn view(self: &Self) -> Element<AlbatrosMessage> {
        column![text(format!("hello from settings: {:?}", self))]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
