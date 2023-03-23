mod menu_theme;

use iced::{
    widget::{button, container, row, svg},
    Application, Color, Length, Rectangle,
};
use iced_window::{drag_window, menu::*, resize::resize};

fn main() {
    MenuTester::run(iced::Settings {
        window: iced::window::Settings {
            decorations: false,
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
    .unwrap();
}

struct MenuTester {}

#[derive(Debug, Clone)]
enum Message {
    WindowEvents(iced_window::window::WindowEvents),
}

impl Application for MenuTester {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = menu_theme::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, iced::Command::none())
    }

    fn title(&self) -> String {
        "Menu Tester".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::WindowEvents(event) => {
                return iced_window::window::Window::event_handler(event)
            }
        };
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let media = MenuTree::with_children(
            button("Media"),
            vec![
                MenuTree::new(button("Open File")),
                MenuTree::new(button("Open Recent")),
            ],
        );

        // let theme_han = ThemeHandler {
        //     child: MenuBar::new(vec![media]).into(),
        //     theme: menu_theme::Theme::default(),
        // };

        iced_window::window::Window::view(vec![media],Message::WindowEvents).into()
    }
}
