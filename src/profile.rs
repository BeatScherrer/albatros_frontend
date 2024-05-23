use iced::Element;
use iced::widget::text;

use crate::Message;

#[derive(Clone, Debug, Default)]
pub struct Profile {
    name: String,
}

impl Profile {
    pub fn view(self: &Self) -> Element<Message> {
        text(format!("Hello from profile {:?}", self)).into()
    }
}
