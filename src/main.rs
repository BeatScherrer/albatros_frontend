use iced::{Element, widget::{row, column, button}, Alignment, advanced::Application, Command, executor};
use notifications::{notification, Notification};
use profile::Profile;
use settings::Settings;

mod settings;
mod profile;
mod notifications;

#[derive(Copy, Clone, Debug, Default)]
pub enum Route {
    #[default]
    Settings,
    Profile
}

#[derive(Default)]
struct AlbatrosApplication {
    route: Route,
    value: Option<u32>,
    settings: Settings,
    profile: Profile,
    notifications: Vec<Notification>
}

#[derive(Clone, Debug)]
pub enum AlbatrosMessage {
    NavigateTo(Route),
    // Used to get bind the event in the home component
    NumericInputChanged(Option<u32>),
    SettingsChanged(Settings),
    NotificationClicked(Notification)
}

impl AlbatrosApplication {

  fn navigation(&self) -> Element<AlbatrosMessage> {
    column![
        button("settings").on_press(AlbatrosMessage::NavigateTo(Route::Settings)),
        button("profile").on_press(AlbatrosMessage::NavigateTo(Route::Profile)),
    ].spacing(10).padding(10).into()
}
}

impl Application for AlbatrosApplication {
    type Executor = executor::Default;
    type Message = AlbatrosMessage;
    type Theme = iced::Theme;
    type Flags = ();
    type Renderer = iced::Renderer;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::default(),
            Command::none()
            )

    }

    fn title(&self) -> String {
        String::from("Albatros")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AlbatrosMessage::NavigateTo(route) => self.route = route,
            AlbatrosMessage::NumericInputChanged(value) => self.value = value,
            AlbatrosMessage::SettingsChanged(settings) => {
                println!("settings changed: {:?}", settings)
            }
            AlbatrosMessage::NotificationClicked(_) => todo!(),
        }

        Command::none()
    }


    fn view(&self) -> Element<'_, AlbatrosMessage> {
      row![
        self.navigation(),
        match self.route {
            Route::Settings => {
              self.settings.view()
            },
            Route::Profile => {
              self.profile.view()
            }
        },
      ].align_items(Alignment::Center).into()
    }
}


fn main() -> iced::Result {
    color_eyre::install().unwrap();

    AlbatrosApplication::run(iced::Settings::default())
}
