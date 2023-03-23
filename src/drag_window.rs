use iced_graphics::alignment;
use iced_native::text::Text;
use iced_native::{layout, renderer, widget::Tree, Layout, Widget};
use iced_native::{mouse, touch, Color, Element, Event, Length, Point, Rectangle, Size};

/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
#[derive(Debug)]
pub struct DragWindow<'a, Message> {
    width: Length,
    height: Length,
    message: Message,
    title: Option<&'a str>,
    title_color: Option<Color>,
}

impl<'a, Message> DragWindow<'a, Message> {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: impl Into<Length>, height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: height.into(),
            message,
            title: None,
            title_color: None,
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: Length::Fill,
            message,
            title: None,
            title_color: None,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: Length::Fill,
            height: height.into(),
            message,
            title: None,
            title_color: None,
        }
    }

    pub fn set_title(mut self, title: Option<&'a str>) -> Self {
        self.title = title;
        self
    }

    pub fn set_title_color(mut self, title_color: Option<Color>) -> Self {
        self.title_color = title_color;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DragWindow<'a, Message>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
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
        _state: &mut iced_native::widget::Tree,
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
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        if let Some(title) = &self.title {
            let bounds = layout.bounds();

            let horizontal_alignment = alignment::Horizontal::Center;
            let vertical_alignment = alignment::Vertical::Center;

            let x = match horizontal_alignment {
                alignment::Horizontal::Left => bounds.x,
                alignment::Horizontal::Center => bounds.center_x(),
                alignment::Horizontal::Right => bounds.x + bounds.width,
            };

            let y = match vertical_alignment {
                alignment::Vertical::Top => bounds.y,
                alignment::Vertical::Center => bounds.center_y(),
                alignment::Vertical::Bottom => bounds.y + bounds.height,
            };

            renderer.fill_text(Text {
                content: title,
                bounds: Rectangle { x, y, ..bounds },
                size: renderer.default_size(),
                color: if let Some(color) = self.title_color {
                    color
                } else {
                    style.text_color
                },
                font: Default::default(),
                horizontal_alignment,
                vertical_alignment,
            })
        }
    }
}

impl<'a, Message, Renderer> From<DragWindow<'a, Message>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Message: 'a + Clone,
{
    fn from(space: DragWindow<'a, Message>) -> Element<'a, Message, Renderer> {
        Element::new(space)
    }
}
