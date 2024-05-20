use components::{
    home::home_component,
    settings::{settings_component, Settings},
};
use iced::{widget::{center, row, button}, Element};

mod components;

#[derive(Copy, Clone, Debug, Default)]
enum Route {
    #[default]
    Home,
    Settings,
}

#[derive(Default)]
struct Application {
    route: Route,
    value: Option<u32>,
    settings: Settings,
}

#[derive(Clone, Debug)]
enum Message {
    NavigateTo(Route),
    // Used to get bind the event in the home component
    NumericInputChanged(Option<u32>),
    SettingsChanged(Settings),
}

impl Application {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(route) => self.route = route,
            Message::NumericInputChanged(value) => self.value = value,
            Message::SettingsChanged(settings) => {
                println!("settings changed: {:?}", settings)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
      row![
        button("home").on_press(Message::NavigateTo(Route::Home)),
        button("settings").on_press(Message::NavigateTo(Route::Settings)),
        match self.route {
            Route::Home => center(home_component(self.value, Message::NumericInputChanged))
                .padding(20),
            Route::Settings => {
                center(settings_component(&self.settings, Message::SettingsChanged))
            }
        }
      ].into()
    }
}

fn main() -> iced::Result {
    iced::run("Albatros", Application::update, Application::view)
}
