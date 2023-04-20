use std::marker::PhantomData;

use iced_native::{event, mouse, overlay, Element, Event, Point, Rectangle};
use iced_native::{
    widget::{Operation, Tree},
    Clipboard, Layout, Shell, Widget,
};

use crate::menu_theme;

// pub fn theme_handler<'a, Message, Renderer, Theme>(
//     child: impl Into<Element<'a, Message, Renderer>>,
//     theme: <Renderer as iced_native::Renderer>::Theme,
// ) -> ThemeHandler<'a, Message, Renderer>
// where
//     Renderer: iced_native::Renderer + 'a,
// {
//     ThemeHandler::new(child, theme)
// }

pub type NewRenderer<Theme = menu_theme::Theme> =
    iced_graphics::Renderer<iced_wgpu::Backend, Theme>;

pub fn new_theme_handler<'a, Message, Renderer, R>(
    child: impl Into<Element<'a, Message, R>>,
    theme: <R as iced_native::Renderer>::Theme,
) -> ThemeHandler<'a, Message, Renderer, R>
where
    Renderer: iced_native::Renderer + 'a,
    R: iced_native::Renderer + 'a,
{
    ThemeHandler {
        theme,
        child: child.into(),
        n: PhantomData,
    }
}

pub struct ThemeHandler<'a, Message, Renderer, R>
where
    R: iced_native::Renderer + 'a,
{
    pub theme: <R as iced_native::Renderer>::Theme,
    pub child: Element<'a, Message, R>,
    n: std::marker::PhantomData<Renderer>,
}

impl<'a, Message, Renderer, R> Widget<Message, Renderer> for ThemeHandler<'a, Message, Renderer, R>
where
    R: iced_native::Renderer + 'a,
    Renderer: iced_native::Renderer + 'a,
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
        let renderer = to_new_type::<Renderer, R>(renderer);
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
        let renderer = to_new_type::<Renderer, R>(renderer);
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
        let renderer = to_new_type::<Renderer, R>(renderer);

        let size = limits.max();

        iced_native::layout::Node::with_children(
            size,
            vec![self.child.as_widget().layout(renderer, limits)],
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
        let renderer = unsafe { std::mem::transmute::<&mut Renderer, &mut R>(renderer) };
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
        let renderer = to_new_type::<Renderer, R>(renderer);
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
        let renderer = to_new_type::<Renderer, R>(renderer);
        let len = layout.children().fold(0, |mut acc, _x| {
            acc += 1;
            acc
        });
        let overlay = self.child.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        );

        unsafe {
            std::mem::transmute::<
                Option<overlay::Element<'b, Message, R>>,
                Option<overlay::Element<'b, Message, Renderer>>,
            >(overlay)
        }
    }
}

fn to_new_type<'a, Renderer, NewRenderer>(renderer: &Renderer) -> &'a NewRenderer
where
    Renderer: iced_native::Renderer + 'a,
{
    unsafe { std::mem::transmute::<&Renderer, &NewRenderer>(renderer) }
}

// impl<'a, Message, Renderer, NewRenderer>
//     From<ThemeHandler<'a, Message, Renderer, NewRenderer>
//     for Element<'a, Message, Renderer>
// where
//     Renderer: iced_native::Renderer + 'a,
//     Message: 'a,
// {
//     fn from(
//         value: ThemeHandler<
//             'a,
//             Message,
//             Renderer,
//             ,
//         >,
//     ) -> Self {
//         Element::new(value)
//     }
// }

impl<'a, Message, Renderer> From<ThemeHandler<'a, Message, Renderer, NewRenderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + iced_native::Renderer,
    NewRenderer: 'a + iced_native::Renderer,
{
    fn from(value: ThemeHandler<'a, Message, Renderer, NewRenderer>) -> Self {
        Self::new(value)
    }
}

