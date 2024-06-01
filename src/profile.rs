use iced::widget::text;
use iced::Element;

use crate::application::AlbatrosMessage;

#[derive(Clone, Debug, Default)]
pub struct Profile {
    name: String,
}

impl Profile {
    pub fn view(self: &Self) -> Element<AlbatrosMessage> {
        text(format!("Hello from profile {:?}", self)).into()
    }
}
