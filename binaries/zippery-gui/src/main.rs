#![windows_subsystem = "windows"]
mod manager;
use manager::Manager;
pub fn main() -> iced::Result {

    iced::application(Manager::new, Manager::update, Manager::view)
        .title(|state: &Manager| {  format!("Zippery - {}", state.current_dir.display())}).run()

}