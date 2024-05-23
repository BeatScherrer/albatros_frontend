use iced::{advanced::Application, Element, executor, Command, widget::row, widget::{column, button}, Alignment};

use crate::{settings::Settings, profile::Profile, notifications::Notification};


#[derive(Copy, Clone, Debug, Default)]
pub enum Page {
    #[default]
    Settings,
    Profile
}

#[derive(Default)]
pub struct AlbatrosApplication {
    page: Page,
    value: Option<u32>,
    settings: Settings,
    profile: Profile,
    notifications: Vec<Notification>
}

#[derive(Clone, Debug)]
pub enum AlbatrosMessage {
    NavigateTo(Page),
    // Used to get bind the event in the home component
    NumericInputChanged(Option<u32>),
    SettingsChanged(Settings),
    NotificationClicked(Notification)
}

impl AlbatrosApplication {

  fn navigation(&self) -> Element<AlbatrosMessage> {
    column![
        button("settings").on_press(AlbatrosMessage::NavigateTo(Page::Settings)),
        button("profile").on_press(AlbatrosMessage::NavigateTo(Page::Profile)),
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
            AlbatrosMessage::NavigateTo(route) => self.page = route,
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
        match self.page {
            Page::Settings => {
              self.settings.view()
            },
            Page::Profile => {
              self.profile.view()
            }
        },
      ].align_items(Alignment::Center).into()
    }
}
