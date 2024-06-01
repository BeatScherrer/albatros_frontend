use application::AlbatrosApplication;
use iced::advanced::Application;

mod application;
mod notifications;
mod profile;
mod settings;

fn main() -> iced::Result {
    color_eyre::install().unwrap();

    AlbatrosApplication::run(iced::Settings::default())
}
