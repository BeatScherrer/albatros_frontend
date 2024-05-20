use components::home::home_component;
use iced::{widget::center, Element};

mod components;

#[derive(Copy, Clone, Debug, Default)]
enum Route {
    #[default]
    Home,
    Settings
}

#[derive(Default)]
struct Application {
    route: Route,
    value: Option<u32>
}


#[derive(Clone, Debug)]
enum Message {
    NavigateTo(Route),
    NumericInputChanged(Option<u32>)
}

impl Application {

    pub fn update(&mut self, message: Message) {
    match message {
        Message::NavigateTo(route) => self.route = route,
        Message::NumericInputChanged(value) => self.value = value,
    }
    }

    pub fn view(&self) -> Element<Message> {
      center(home_component(self.value, Message::NumericInputChanged))
      .padding(20)
      .into()
    }

}


fn main() -> iced::Result {
  iced::run("Albatros", Application::update, Application::view)
}
