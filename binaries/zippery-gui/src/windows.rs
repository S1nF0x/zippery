use crate::{launch::LaunchMode, windows::about::About};
use iced::{Element, Task};
mod about;
pub mod manager;
use manager::Manager;

#[derive(Debug, Clone)]

pub enum Request {
    OpenWindow(Window),
    CloseWindow,
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Request(Request),
    About(about::Message),
    Manager(manager::Message),
    //Compress(Compress::Message),
    //Extract(extract::Message),
}

#[derive(Debug, Clone)]
pub enum Window {
    Manager(Manager),
    About(About),
    //Compress(Add),
    //Extract(Extract),
}

impl Window {
    pub fn from_mode(mode: LaunchMode) -> (Self, Task<WindowMessage>) {
        match mode {
            LaunchMode::Browse => {
                let (manager, task) = Manager::new();
                (Window::Manager(manager), task.map(WindowMessage::Manager))
            }

            LaunchMode::Extract(_path) => {
                todo!("Extract not implemented")
            }

            LaunchMode::AddToArchive(_files) => {
                todo!("Compress not implemented")
            }
        }
    }

    pub fn settings(&self) -> iced::window::Settings {
        match self {
            Window::Manager(_) => iced::window::Settings::default(),
            Window::About(_) => iced::window::Settings {
                size: iced::Size::new(400.0, 300.0),
                position: iced::window::Position::Centered,
                ..Default::default()
            },
        }
    }



    pub fn update(&mut self, msg: WindowMessage) -> Task<WindowMessage> {
        match (self, msg) {
            (Window::Manager(m), WindowMessage::Manager(msg)) => {
                match msg {
                    manager::Message::Request(manager::Request::OpenAbout) => Task::done(
                        WindowMessage::Request(Request::OpenWindow(Window::About(About::new()))),
                    ),
                    other => m.update(other).map(WindowMessage::Manager),
                }
            }

            (Window::About(a), WindowMessage::About(msg)) => match msg {
                about::Message::Close => Task::done(WindowMessage::Request(Request::CloseWindow)),
                other => a.update(other).map(WindowMessage::About),
            },

            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, WindowMessage> {
        match self {
            Window::About(about) => about.view().map(WindowMessage::About),
            Window::Manager(manager) => manager.view().map(WindowMessage::Manager),
        }
    }

    pub fn title(&self) -> String {
    match self {
        Window::About(_) => "About".into(),
        Window::Manager(manager) => manager.title(),
    }
}
}
