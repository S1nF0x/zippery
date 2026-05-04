#![windows_subsystem = "windows"]
use iced::{
    Element, Subscription, Task, Theme,
    widget::{center, space},
    window,
};
use std::collections::BTreeMap;

mod launch;
mod windows;

use launch::LaunchMode;
use windows::manager::config::{Config, config_path};
use windows::{Window, WindowMessage};

use crate::windows::manager::config::ThemeKind;

pub fn main() -> iced::Result {
    let mode = launch::parse_args();
    iced::daemon(move || App::new(mode.clone()), App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}

struct App {
    windows: BTreeMap<window::Id, Window>,
    theme: Theme,
    pending: Vec<(Window, Task<WindowMessage>)>,
}

enum Message {
    Window(window::Id, WindowMessage),
    WindowOpened(window::Id),
    WindowClosed(window::Id),
}

impl App {
    fn load_theme() -> Theme {
        if let Some(path) = config_path() {
            std::fs::read(path)
                .ok()
                .and_then(|bytes| bitcode::decode::<Config>(&bytes).ok())
                .map(|cfg| cfg.ui.theme.theme)
                .unwrap_or_default()
                .into()
        } else {
            return ThemeKind::default().into();
        }
    }

    fn new(mode: LaunchMode) -> (Self, Task<Message>) {
        let theme = Self::load_theme();

        let windows = BTreeMap::new();
        let (window, task) = Window::from_mode(mode);
        let settings = window.settings();
        let (_, open) = window::open(settings);
        (
            Self {
                windows,
                theme,
                pending: vec![(window, task)],
            },
            open.map(Message::WindowOpened),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title().clone())
            .unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(id, WindowMessage::Request(req)) => match req {
                windows::Request::OpenWindow(window) => {
                    let settings = window.settings();
                    let (_, open) = window::open(settings);
                    self.pending.push((window, Task::none()));
                    open.map(Message::WindowOpened)
                }
                windows::Request::CloseWindow => {
                    let close = window::close(id);
                    let exit = self.close_window(id);
                    Task::batch([close, exit])
                }
            },

            Message::Window(id, msg) => {
                // If clicked window is not the last (top) window, flash it and ignore
                let top_id = self.windows.keys().last().copied();
                if top_id != Some(id) {
                    return if let Some(top) = top_id {
                        window::gain_focus(top) // flash/bring top window to front
                    } else {
                        Task::none()
                    };
                }

                if let Some(window) = self.windows.get_mut(&id) {
                    window.update(msg).map(move |m| Message::Window(id, m))
                } else {
                    Task::none()
                }
            }

            Message::WindowOpened(id) => {
                // Pop the oldest pending window (FIFO).
                if !self.pending.is_empty() {
                    let (window, task) = self.pending.remove(0);
                    self.windows.insert(id, window);
                    task.map(move |m| Message::Window(id, m))
                } else {
                    Task::none()
                }
            }

            Message::WindowClosed(id) => self.close_window(id),
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(
                window
                    .view()
                    .map(move |msg| Message::Window(window_id, msg)),
            )
            .into()
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

    fn close_window(&mut self, id: window::Id) -> Task<Message> {
        self.windows.remove(&id);

        if self.windows.is_empty() {
            iced::exit()
        } else {
            Task::none()
        }
    }
}
