use application::AlbatrosApplication;
use iced::advanced::Application;

mod application;
mod settings;
mod profile;
mod notifications;


fn main() -> iced::Result {
    color_eyre::install().unwrap();

    AlbatrosApplication::run(iced::Settings::default())
}

