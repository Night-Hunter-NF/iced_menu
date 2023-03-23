use iced_native::{event, overlay, Element, Event, Point, Rectangle, mouse};
use iced_native::{
    widget::{Operation, Tree},
    Clipboard, Layout, Shell, Widget,
};

// pub fn theme_handler<'a, Message, Renderer, Theme>(
//     child: impl Into<Element<'a, Message, Renderer>>,
//     theme: <Renderer as iced_native::Renderer>::Theme,
// ) -> ThemeHandler<'a, Message, Renderer>
// where
//     Renderer: iced_native::Renderer + 'a,
// {
//     ThemeHandler::new(child, theme)
// }
// impl<'a, Message> ThemeHandler<'a, Message, Render<Theme>>
// {
//     pub fn new(
//         child: impl Into<Element<'a, Message, Render<Theme>>>,
//         theme: Theme,
//     ) -> Self {
//         Self {
//             theme,
//             child: child.into(),
//         }
//     }
// }

// pub type NewThemeRenderer<T> = iced_native::Renderer<T>;

pub struct ThemeHandler<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + 'a,
{
    pub theme: Renderer::Theme,
    pub child: Element<'a, Message, Renderer>,
}



impl<'a, Message, Renderer> Widget<Message, Renderer> for ThemeHandler<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + 'a,
    Renderer::Theme: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child))
    }

    fn width(&self) -> iced_native::Length {
        iced_native::Length::Shrink
    }

    fn height(&self) -> iced_native::Length {
        iced_native::Length::Shrink
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.child.as_widget().operate(
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
        self.child.as_widget_mut().on_event(
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
        self.child.as_widget().layout(renderer, limits)
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
        self.child.as_widget().draw(
            state,
            renderer,
            &self.theme,
            style,
            layout,
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
        self.child.as_widget().mouse_interaction(
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
        self.child.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<ThemeHandler<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + 'a,
    Renderer::Theme: Clone,
    Message: 'a,
{
    fn from(value: ThemeHandler<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
