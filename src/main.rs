use iced::{Element, widget::{row, column, button}};
use profile::Profile;
use settings::Settings;

mod settings;
mod profile;

#[derive(Copy, Clone, Debug, Default)]
pub enum Route {
    #[default]
    Settings,
    Profile
}

#[derive(Default)]
struct Application {
    route: Route,
    value: Option<u32>,
    settings: Settings,
    profile: Profile
}

#[derive(Clone, Debug)]
pub enum Message {
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

  fn navigation(&self) -> Element<Message> {
    column![
        button("settings").on_press(Message::NavigateTo(Route::Settings)),
        button("profile").on_press(Message::NavigateTo(Route::Profile)),
    ].into()
}

    pub fn view(&self) -> Element<Message> {
      row![
        self.navigation(),
        match self.route {
            Route::Settings => {
              self.settings.view()
            },
            Route::Profile => {
              self.profile.view()
            }
        }
      ].into()
    }

}


fn main() -> iced::Result {
    iced::run("Albatros", Application::update, Application::view)
}
