use iced::{Element, Length};
use iced::widget::{column, container, text};

#[derive(Default)]
pub struct PlaceholderWindow;

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

impl PlaceholderWindow {
    pub fn update(&mut self, _msg: Message) {
    }

    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("🚧 Coming soon")
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}