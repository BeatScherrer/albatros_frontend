use iced::Element;
use iced::widget::text;

use crate::AlbatrosMessage;

#[derive(Clone, Debug, Default)]
pub struct Profile {
    name: String,
}

impl Profile {
    pub fn view(self: &Self) -> Element<AlbatrosMessage> {
        text(format!("Hello from profile {:?}", self)).into()
    }
}
