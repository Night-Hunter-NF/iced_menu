use iced_graphics::{Color, Rectangle};
use iced_native::{
    command, row,
    widget::{button, container, svg},
    window, Command, Length,
};

use crate::{
    drag_window::DragWindow,
    menu::{self, MenuBar, MenuTree},
    resize::{resize, ResizeEvent},
    svgs,
};

pub struct Window;

#[derive(Debug, Clone)]
pub enum TitleEvents {
    Minimize,
    Restore,
    Close,
}

#[derive(Debug, Clone)]
pub enum WindowEvents {
    ResizeEvent(ResizeEvent),
    TitleEvent(TitleEvents),
    DragWindow,
}

impl Window {
    pub fn view<'a, Message, Renderer, F>(
        menu_roots: Vec<MenuTree<'a, Message, Renderer>>,
        event_handler: F,
        title: Option<&'a str>,
        title_color: Option<Color>,
    ) -> iced_native::Element<'a, Message, Renderer>
    where
        Renderer:
            iced_native::Renderer + iced_native::svg::Renderer + iced_native::text::Renderer + 'a,
        Renderer::Theme: iced_native::widget::container::StyleSheet
            + menu::StyleSheet
            + iced_native::widget::button::StyleSheet
            + iced_native::widget::svg::StyleSheet,
        Message: 'a + Clone,
        F: 'a + Clone + Fn(WindowEvents) -> Message,
    {
        let event_handler2 = event_handler.clone();
        let title_bar_buttons = row![
            button(svg(svgs::minimize_svg()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Minimize
                ))),
            button(svg(svgs::restore()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Restore
                ))),
            button(svg(svgs::close_svg()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::Close)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Close
                ))),
        ];

        resize(
            container(
                container(row![
                    MenuBar::new(menu_roots).close_condition(menu::CloseCondition {
                        leave: false,
                        click_outside: true,
                        click_inside: true
                    }),
                    DragWindow::with_width(Length::Fill, (event_handler)(WindowEvents::DragWindow))
                        .set_title(title)
                        .set_title_color(title_color),
                    title_bar_buttons
                ])
                .height(35.0),
            )
            .width(Length::Fill)
            .height(Length::Fill),
            move |e| (event_handler2)(WindowEvents::ResizeEvent(e)),
        )
        .into()
    }
}

impl Window {
    pub fn event_handler<Message>(event: WindowEvents) -> Command<Message> {
        match event {
            WindowEvents::ResizeEvent(re) => match re {
                ResizeEvent::ResizeXY(size) => {
                    return Command::single(command::Action::Window(window::Action::Resize {
                        width: size.width as u32,
                        height: size.height as u32,
                    }));
                }
                ResizeEvent::ResizeWindow(rec) => {
                    let Rectangle {
                        x,
                        y,
                        width,
                        height,
                    } = rec;
                    return Command::batch(vec![
                        Command::single(command::Action::Window(window::Action::Resize {
                            width: width as u32,
                            height: height as u32,
                        })),
                        Command::single(command::Action::Window(window::Action::Move {
                            x: x as i32,
                            y: y as i32,
                        })),
                    ]);
                }
            },
            WindowEvents::TitleEvent(te) => match te {
                TitleEvents::Minimize => {
                    return Command::single(command::Action::Window(window::Action::Minimize(
                        true,
                    )));
                }
                TitleEvents::Restore => {
                    return Command::single(command::Action::Window(window::Action::Minimize(
                        false,
                    )));
                }
                TitleEvents::Close => {
                    return Command::single(command::Action::Window(window::Action::Close));
                }
            },
            WindowEvents::DragWindow => {
                return Command::single(command::Action::Window(window::Action::Drag))
            }
        }
    }
}
