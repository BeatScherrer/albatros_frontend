/* NOTE:
 * The notifications have the following requirements
 * - [ ] Pop up relative to the root node (top middle or top right)
 * - [ ] Support swiping the notification to clear (side or up, depending on position)
 * - [ ] Add click handler
 * - [ ] stack notification history in a smooth manner and to not pollute the screen space
 * - [ ] add interface to add a notification
 * - [ ] add interface to clear a notification
 *
 * it's not clear to yet on how this should be designed in the best way. Some thaughts
 *
 * Messages:
 *
 * - on single notification click emit message 'NotificationClicked(Notification)'
 * - on new notification data: add 
 *
 */

use iced::{widget::{overlay, text}, Element, Length, Size, advanced::layout, Color, Border};
use iced::advanced::widget::Widget;
use iced::advanced::renderer;

use crate::AlbatrosMessage;

#[derive(Clone, Default, Debug)]
pub enum NotificationLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error
}

#[derive(Clone, Debug, Default)]
pub struct Notification {
    level: NotificationLevel,
    title: String,
    message: String
}

    pub fn notification() -> Notification {
        Notification::default()
    }


impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Notification
  where Renderer: renderer::Renderer
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size{
            width: Length::Fill,
            height: Length::Shrink
        }
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::Node::new(Size::new(100.0, 50.0))
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // TODO: we want to draw relative to the root node
        renderer.fill_quad(renderer::Quad{
            bounds: layout.bounds(),
            border: Border::rounded(10),
            ..renderer::Quad::default()
        }, Color::WHITE)
    }
}

impl <Message, Theme, Renderer> From<Notification> for Element<'_, Message, Theme, Renderer>
  where Renderer: renderer::Renderer
{
    fn from(value: Notification) -> Self {
        Self::new(value)
    }
}

