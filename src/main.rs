use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i32
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement)
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
            self.value += 1;
            }
            Message::Decrement => {
            self.value -= 1;
            }
        }
    }
}


fn main() -> iced::Result {
  iced::run("A cool counter", Counter::update, Counter::view)
}
