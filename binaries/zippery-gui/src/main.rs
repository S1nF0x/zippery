#![windows_subsystem = "windows"]
use iced::{
    Element, Subscription, Task, Theme, widget::{center, operation, space}, window
};
use std::collections::BTreeMap;

mod launch;
mod windows;

use launch::LaunchMode;
use windows::manager::config::Config;
use windows::{Window, WindowMessage};

pub fn main() -> iced::Result {
    let mode = launch::parse_args();
    iced::daemon(move || App::new(mode.clone()), App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}

struct App {
    windows: BTreeMap<window::Id, Window>,
    theme: Theme,
    pending: Option<(Window, Task<WindowMessage>)>,
}

#[derive(Debug, Clone)]
enum Message {
    Window(window::Id, windows::WindowMessage),
    WindowOpened(window::Id),
    WindowClosed(window::Id),
}

impl App {
    fn load_theme() -> Theme {
        std::fs::read("config.bin")
            .ok()
            .and_then(|bytes| bitcode::decode::<Config>(&bytes).ok())
            .map(|cfg| cfg.ui.theme.theme)
            .unwrap_or_default()
            .into()
    }

    fn new(mode: LaunchMode) -> (Self, Task<Message>) {
        let theme = Self::load_theme();

        let windows = BTreeMap::new();

        let settings = match &mode {
            LaunchMode::Browse => window::Settings::default(),
            LaunchMode::Extract(_) => window::Settings {
                size: iced::Size::new(900.0, 600.0),
                ..window::Settings::default()
            },

            LaunchMode::AddToArchive(_) => window::Settings {
                size: iced::Size::new(500.0, 350.0),
                ..window::Settings::default()
            },
        };

        let (window, task) = Window::from_mode(mode);
        let (_, open) = window::open(settings);
        (
            Self {
                windows,
                theme,
                pending: Some((window, task)),
            },
            open.map(Message::WindowOpened),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(id, msg) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    return window.update(msg).map(move |m| Message::Window(id, m));
                }
                Task::none()
            }

            Message::WindowOpened(id) => {
                if let Some((window, task)) = self.pending.take() {
                    self.windows.insert(id, window);

                    Task::batch([
                        operation::focus(format!("input-{id}")),
                        task.map(move |msg| Message::Window(id, msg)),
                    ])
                } else {
                    Task::none()
                }
            }

            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view().map(move |msg| Message::Window(window_id, msg))).into()
        } else {
            space().into()
        }
    }

    fn theme(&self, _window: window::Id) -> Theme {
        self.theme.clone()
    }


    fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}
