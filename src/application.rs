#![allow(dead_code, unused_variables)]

use iced::{
    advanced::Application,
    executor,
    widget::{button, center, column, container, row, text},
    Color, Command, Element, Length,
};

use crate::{
    notifications::{Notification, Notifications},
    profile::Profile,
    settings::Settings,
};

#[derive(Copy, Clone, Debug, Default)]
pub enum Page {
    #[default]
    Settings,
    Profile,
}

#[derive(Default)]
pub struct AlbatrosApplication {
    page: Page,
    value: Option<u32>,
    settings: Settings,
    profile: Profile,
    notifications: Vec<Notification>,
    debug: bool,
}

#[derive(Clone, Debug)]
pub enum AlbatrosMessage {
    NavigateTo(Page),
    Dummy(u64),
    DebugToggled(bool),
}

impl AlbatrosApplication {
    fn navigation(&self) -> Element<AlbatrosMessage> {
        column![
            button("settings")
                .on_press(AlbatrosMessage::NavigateTo(Page::Settings))
                .width(Length::Fill),
            button("profile")
                .on_press(AlbatrosMessage::NavigateTo(Page::Profile))
                .width(Length::Fill),
        ]
        .spacing(10)
        .padding(10)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
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
            AlbatrosApplication {
                notifications: vec![Notification {
                    title: String::from("hello"),
                    message: String::from("World"),
                }],
                debug: true,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Albatros")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AlbatrosMessage::NavigateTo(route) => self.page = route,
            AlbatrosMessage::Dummy(_) => println!("dummy message received"),
            AlbatrosMessage::DebugToggled(value) => self.debug = value,
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, AlbatrosMessage> {
        let element: Element<'_, AlbatrosMessage> = row![
            container(self.navigation()).width(200).height(Length::Fill),
            center(match self.page {
                Page::Settings => self.settings.view(),
                Page::Profile => self.profile.view(),
            })
            .width(Length::FillPortion(5))
            .height(Length::Fill),
            container(Notifications::new(
                column![text("hello"), text("world")],
                &self.notifications,
                |s: usize| AlbatrosMessage::Dummy(1),
            ))
            .width(0), // NOTE: hack to hide the widget
        ]
        .into();

        if self.debug {
            element.explain(Color::BLACK)
        } else {
            element
        }
    }
}
