use std::time::{Duration, Instant};

use iced::advanced::overlay;

use iced::advanced::widget::Tree;
use iced::advanced::{layout, renderer, widget, Clipboard, Layout, Shell};
use iced::event::Event;
use iced::widget::{button, column, container, horizontal_rule, horizontal_space, row, text};
use iced::{advanced::Widget, Element, Length, Renderer, Size, Theme};
use iced::{event, mouse, window, Alignment, Color, Point, Rectangle, Vector};

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
    Dummy,
}

#[derive(Debug, Clone, Default)]
pub struct Notification {
    pub title: String,
    pub message: String,
}

pub struct Notifications<'a, Message> {
    content: Element<'a, Message>,
    notifications: Vec<Element<'a, Message>>,
    timeout_secs: u64,
}

impl<'a, Message> Notifications<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(
        content: impl Into<Element<'a, Message>>,
        notifications: &'a [Notification],
        on_close: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        let notifications = notifications
            .iter()
            .enumerate()
            .map(|(index, notification)| {
                container(column![
                    container(
                        row![
                            text(notification.title.as_str()),
                            horizontal_space(),
                            button("X").on_press((on_close)(index)).padding(3),
                        ]
                        .align_items(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding(5),
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

        // TODO: switch .explain() on debug flag smh
        Self {
            content: content.into().explain(Color::BLACK),
            notifications,
            timeout_secs: DEFAULT_TIMEOUT,
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for Notifications<'a, Message> {
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
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
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        std::iter::once(Tree::new(&self.content))
            .chain(self.notifications.iter().map(Tree::new))
            .collect()
    }

    fn tag(&self) -> widget::tree::Tag {
        struct Marker;
        widget::tree::Tag::of::<Marker>()
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let instants = state.state.downcast_mut::<Vec<Option<Instant>>>();
        let (content_state, notification_state) = state.children.split_at_mut(1);

        let content = self.content.as_widget_mut().overlay(
            &mut content_state[0],
            layout,
            renderer,
            translation,
        );

        let notifications = (!self.notifications.is_empty()).then(|| {
            overlay::Element::new(Box::new(Overlay {
                position: Point { x: 0.0, y: 0.0 },
                notifications: &mut self.notifications,
                state: notification_state,
                instants,
            }))
        });

        let overlays = content.into_iter().chain(notifications).collect::<Vec<_>>();
        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Vec::<Option<Instant>>::new())
    }
}

impl<'a, Message> From<Notifications<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: Notifications<'a, Message>) -> Self {
        Element::new(value)
    }
}

struct Overlay<'a, 'b, Message> {
    position: Point,
    notifications: &'b mut [Element<'a, Message>],
    state: &'b mut [Tree],
    instants: &'b mut [Option<Instant>],
}

impl<'a, 'b, Message> overlay::Overlay<Message, Theme, Renderer> for Overlay<'a, 'b, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);

        layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            Length::Fill,
            Length::Fill,
            10.into(),
            10.0,
            Alignment::End,
            self.notifications,
            self.state,
        )
        .translate(Vector::new(self.position.x, self.position.y))
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        // match &event {
        //     Event::Window(Event::RedrawRequested(now)) => {
        //         let mut next_redraw: Option<window::RedrawRequest> = None;
        //
        //         // self.instants
        //         //     .iter_mut()
        //         //     .enumerate()
        //         //     .for_each(|(index, maybe_instant)| {
        //         //         if let Some(instant) = maybe_instant.as_mut() {
        //         //             let remaining = Duration::from_secs(self.timeout_secs)
        //         //                 .saturating_sub(instant.elapsed());
        //         //
        //         //             if remaining == Duration::ZERO {
        //         //                 maybe_instant.take();
        //         //                 shell.publish((self.on_close)(index));
        //         //                 next_redraw = Some(window::RedrawRequest::NextFrame);
        //         //             } else {
        //         //                 let redraw_at = window::RedrawRequest::At(*now + remaining);
        //         //                 next_redraw = next_redraw
        //         //                     .map(|redraw| redraw.min(redraw_at))
        //         //                     .or(Some(redraw_at));
        //         //             }
        //         //         }
        //         //     });
        //
        //         if let Some(redraw) = next_redraw {
        //             shell.request_redraw(redraw);
        //         }
        //     }
        //     _ => (),
        // }

        let viewport = layout.bounds();

        self.notifications
            .iter_mut()
            .zip(self.state.iter_mut())
            .zip(layout.children())
            .zip(self.instants.iter_mut())
            .map(|(((child, state), layout), instant)| {
                let mut local_messages = vec![];
                let mut local_shell = Shell::new(&mut local_messages);

                let status = child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    &mut local_shell,
                    &viewport,
                );

                // if !local_shell.is_empty() {
                //     instant.take();
                // }

                shell.merge(local_shell, std::convert::identity);

                status
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let viewport = layout.bounds();
        for ((child, state), layout) in self
            .notifications
            .iter()
            .zip(self.state.iter())
            .zip(layout.children())
        {
            // TODO: How can we add an explain here to debug the notification?
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, &viewport);
        }
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.notifications
                .iter()
                .zip(self.state.iter_mut())
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.notifications
            .iter()
            .zip(self.state.iter())
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn is_over(&self, layout: Layout<'_>, _renderer: &Renderer, cursor_position: Point) -> bool {
        layout
            .children()
            .any(|layout| layout.bounds().contains(cursor_position))
    }
}
