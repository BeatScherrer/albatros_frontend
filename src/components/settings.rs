use iced::{
    widget::{button, component, text, text_input, Component},
    Element,
};

#[derive(Clone, Debug, Default)]
pub struct Settings {
    name: String,
    value: u32,
}

pub struct SettingsComponent<'a, Message> {
    settings: &'a Settings,
    on_change: Box<dyn Fn(Settings) -> Message>,
}

impl<'a, Message> SettingsComponent<'a, Message> {
    pub fn new(settings: &'a Settings, on_change: impl Fn(Settings) -> Message + 'static) -> Self {
        Self {
            settings,
            on_change: Box::new(on_change),
        }
    }
}

pub fn settings_component<Message>(
    settings: &Settings,
    on_change: impl Fn(Settings) -> Message + 'static,
) -> SettingsComponent<Message> {
    SettingsComponent::new(settings, on_change)
}

pub enum Event {
    ChangeSettings(Settings),
    ExportSettings(Settings),
    ImportSettings(Settings),
}

impl<'a, Message, Theme> Component<Message, Theme> for SettingsComponent<'a, Message>
where
    Theme: text::Catalog + button::Catalog + text_input::Catalog + 'static,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, _event: Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
        text("hello world").into()
    }
}

impl<'a, Message, Theme> From<SettingsComponent<'a, Message>> for Element<'a, Message, Theme>
where
    Theme: text::Catalog + button::Catalog + text_input::Catalog + 'static,
    Message: 'a
{
    fn from(settings_component: SettingsComponent<'a, Message>) -> Self {
        component(settings_component)
    }
}
