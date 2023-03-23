use iced_native::{layout, renderer, widget::Tree, Layout, Widget};
use iced_native::{mouse, touch, Color, Element, Event, Length, Point, Rectangle, Size};

/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
#[derive(Debug)]
pub struct DragWindow<Message> {
    width: Length,
    height: Length,
    message: Message,
}

impl<Message> DragWindow<Message> {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: impl Into<Length>, height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: height.into(),
            message,
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: Length::Fill,
            message,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: Length::Fill,
            height: height.into(),
            message,
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for DragWindow<Message>
where
    Renderer: iced_native::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::Node::new(limits.resolve(Size::ZERO))
    }

    fn on_event(
        &mut self,
        state: &mut iced_native::widget::Tree,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced_native::event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    shell.publish(self.message.clone())
                }
            }
            _ => {}
        }

        iced_native::event::Status::Ignored
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::new(0.0, 0.0, 0.0, 0.0),
        );
    }
}

impl<'a, Message, Renderer> From<DragWindow<Message>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: 'a + Clone,
{
    fn from(space: DragWindow<Message>) -> Element<'a, Message, Renderer> {
        Element::new(space)
    }
}
