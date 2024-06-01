use iced::{Alignment, Event};
use iced::{advanced::Widget, Theme, Renderer, Size, Length, Element};
use iced::widget::{button, column, row, horizontal_rule, container, text, horizontal_space};

#[allow(dead_code, unused_imports)]

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
 * the following resource explains on how to control widgets by commands. In particular:
 * how a text_input field can be focused based on its id.
 *
 * this would be analogous to how we can dispose or add a notification
 * https://github.com/fogarecious/iced_tutorial/blob/main/tutorial/controlling_widgets_by_commands.md
 *
 * TODO: the 'toast' example is what is required more or less
 *
 */

pub const DEFAULT_TIMEOUT: u64 = 5;

pub enum Message {
  Event(Event),
  Dummy
}

#[derive(Debug, Clone, Default)]
pub struct Notification {
    title: String,
    message: String,
}

pub struct Notifications<'a, Message> {
    content: Element<'a, Message>,
    notifications: Vec<Element<'a, Message>>,
    timeout_secs: u64,
}

impl <'a, Message> Notifications<'a, Message>
where Message: 'a + Clone,
{
pub fn new(content: impl Into<Element<'a, Message>>, notifications: &'a [Notification], on_close: impl Fn(usize) -> Message + 'a) -> Self {
            let notifications = notifications
                .iter()
                .enumerate()
                .map(|(index, notification)| {
                    container(column![
                        container(
                            row![
                                text(notification.title.as_str()),
                                horizontal_space(),
                                button("X")
                                    .on_press((on_close)(index))
                                    .padding(3),
                            ]
                            .align_items(Alignment::Center)
                        )
                        .width(Length::Fill)
                        .padding(5),
                        // .style(match toast.status {
                        //     Status::Primary => primary,
                        //     Status::Secondary => secondary,
                        //     Status::Success => success,
                        //     Status::Danger => danger,
                        // }),
                        horizontal_rule(1),
                        container(text(notification.message.as_str()))
                            .width(Length::Fill)
                            .padding(5)
                            .style(container::rounded_box),
                    ])
                    .max_width(200)
                    .into()
                })
                .collect();

                Self {
                content: content.into(),
                notifications,
                timeout_secs: DEFAULT_TIMEOUT,
                }
           }
}

impl <'a, Message> Widget<Message, Theme, Renderer> for Notifications<'a, Message> {
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.content.as_widget().layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport);
    }
}


impl<'a, Message> From<Notifications<'a, Message>> for Element<'a, Message>
where Message: 'a
      {
        fn from(value: Notifications<'a, Message>) -> Self {
            Element::new(value)
    }
    }
