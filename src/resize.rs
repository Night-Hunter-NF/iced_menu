use iced_native::{event, mouse, overlay, touch, Color, Element, Event, Point, Rectangle, Size};
use iced_native::{
    renderer,
    widget::{Operation, Tree},
    Clipboard, Layout, Shell, Widget,
};

#[derive(Debug, Clone, PartialEq)]
enum Dragging {
    HorizontalRight,
    HorizontalLeft,
    VerticalTop,
    VerticalBottom,
    Both,
    None,
}

struct ResizeState {
    dragging: Dragging,
    window_size: Size,
    window_position: Point,
    show: bool,
}

impl Default for ResizeState {
    fn default() -> Self {
        Self {
            dragging: Dragging::None,
            window_size: Size::new(0.0, 0.0),
            window_position: Point::new(0.0, 0.0),
            show: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResizeEvent {
    ResizeXY(Size),
    ResizeWindow(Rectangle),
}

pub fn resize<'a, Message, Renderer, F: 'a + Fn(ResizeEvent) -> Message>(
    element: impl Into<Element<'a, Message, Renderer>>,
    event_handler: F,
) -> Resize<'a, Message, Renderer> {
    Resize::new(element, event_handler)
}

pub struct Resize<'a, Message, Renderer> {
    element: Element<'a, Message, Renderer>,
    handle_events: Box<dyn Fn(ResizeEvent) -> Message + 'a>,
}

impl<'a, Message, Renderer> Resize<'a, Message, Renderer> {
    pub fn new<F: 'a + Fn(ResizeEvent) -> Message>(
        element: impl Into<Element<'a, Message, Renderer>>,
        event_handler: F,
    ) -> Self {
        Self {
            element: element.into(),
            handle_events: Box::new(event_handler),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Resize<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn width(&self) ->  iced_native::Length {
        iced_native::Length::Fill
    }

    fn height(&self) -> iced_native::Length {
        iced_native::Length::Fill
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.element)]
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        iced_native::widget::tree::Tag::of::<ResizeState>()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        iced_native::widget::tree::State::new(ResizeState::default())
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.element))
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.element.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let mut state = tree.state.downcast_mut::<ResizeState>();
        let mut bounds = Rectangle::new(state.window_position, state.window_size);

        match event {
            Event::Window(ref event) => match event {
                iced_native::window::Event::Resized { width, height } => {
                    state.window_size = Size::new(width.clone() as f32, height.clone() as f32);
                }
                iced_native::window::Event::Moved { x, y } => {
                    state.window_position = Point::new(x.clone() as f32, y.clone() as f32);
                }
                _ => {}
            },
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let mut size = bounds.size();
                let Point { x, y } = cursor_position;

                match state.dragging {
                    Dragging::HorizontalRight => {
                        if x < bounds.width || x > bounds.width {
                            size.width = x;
                            shell.publish((self.handle_events)(ResizeEvent::ResizeXY(size)));
                        }
                    }
                    Dragging::VerticalBottom => {
                        if y < bounds.height || y > bounds.y {
                            size.height = y;
                            shell.publish((self.handle_events)(ResizeEvent::ResizeXY(size)));
                        }
                    }
                    Dragging::HorizontalLeft => {
                        if x < bounds.x || x > bounds.x {
                            bounds.width = bounds.width - x;
                            bounds.x = bounds.x + x;
                            shell.publish((self.handle_events)(ResizeEvent::ResizeWindow(bounds)));
                        }
                    }
                    Dragging::VerticalTop => {
                        if y < bounds.y || y > bounds.y {
                            bounds.height = bounds.height - y;
                            bounds.y = bounds.y + y;
                            shell.publish((self.handle_events)(ResizeEvent::ResizeWindow(bounds)));
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.dragging != Dragging::None {
                    state.dragging = Dragging::None;
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.show {
                    let bounds = layout.bounds();

                    let Point { x, y } = cursor_position;

                    if x < bounds.x + 5.0 {
                        state.dragging = Dragging::HorizontalLeft;
                        return event::Status::Captured;
                    } else if x > bounds.x + bounds.width - 5.0 {
                        state.dragging = Dragging::HorizontalRight;
                        return event::Status::Captured;
                    } else if y < bounds.y + 5.0 {
                        state.dragging = Dragging::VerticalTop;
                        return event::Status::Captured;
                    } else if y > bounds.y + bounds.height - 5.0 {
                        state.dragging = Dragging::VerticalBottom;
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        self.element.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let size = limits.max();

        iced_native::layout::Node::with_children(
            size,
            vec![self.element.as_widget().layout(renderer, limits)],
        )
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        viewport: &iced_native::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::TRANSPARENT,
        );

        self.element.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<ResizeState>();

        if state.show {
            let bounds = layout.bounds();

            let Point { x, y } = cursor_position;

            if x < bounds.x + 5.0 {
                return mouse::Interaction::ResizingHorizontally;
            }
            if x > bounds.x + bounds.width - 5.0 {
                return mouse::Interaction::ResizingHorizontally;
            }
            if y < bounds.y + 5.0 {
                return mouse::Interaction::ResizingVertically;
            }
            if y > bounds.y + bounds.height - 5.0 {
                return mouse::Interaction::ResizingVertically;
            }
        }

        self.element.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.element.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Resize<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
{
    fn from(value: Resize<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}
