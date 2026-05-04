use crate::launch::LaunchMode;
use iced::{Element, Task};
pub mod manager;
use manager::Manager;
#[derive(Debug, Clone)]
pub enum WindowMessage {
    FileManager(manager::Message),
    //Compress(Compress::Message),
    //Extract(extract::Message),
}

#[derive(Debug, Clone)]
pub enum Window {
    Manager(Manager),
    //Compress(Add),
    //Extract(Extract),
}

impl Window {
    pub fn from_mode(mode: LaunchMode) -> (Self, Task<WindowMessage>) {
        match mode {
            LaunchMode::Browse => {
                let (manager, task) = Manager::new();
                (
                    Window::Manager(manager),
                    task.map(WindowMessage::FileManager),
                )
            }

            LaunchMode::Extract(_path) => {
                todo!("Extract not implemented")
            }

            LaunchMode::AddToArchive(_files) => {
                todo!("Compress not implemented")
            }
        }
    }

    pub fn update(&mut self, msg: WindowMessage) -> Task<WindowMessage> {
        match (self, msg) {
            (Window::Manager(manager), WindowMessage::FileManager(m)) => {
                manager.update(m).map(WindowMessage::FileManager)
            }
        }
    }

    pub fn view(&self) -> Element<'_, WindowMessage> {
        match self {
            Window::Manager(manager) => manager.view().map(WindowMessage::FileManager),
        }
    }
}
