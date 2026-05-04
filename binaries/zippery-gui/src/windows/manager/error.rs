use iced::{Element, Length::Fill};
use iced::widget::{button, column, container, text};
use super::message::Message;

pub fn error_modal_view(err: &'_ String) -> Element<'_, Message> {
    container(
        container(
            column![
                text(err),
                button("OK").on_press(Message::CloseError),
            ]
            .spacing(10)
        )
        .padding(20)
        .style(|_| container::Style {
            background: Some(iced::Color::WHITE.into()),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    )
    .width(200).height(100).center_x(Fill).center_y(Fill)
    .style(|_| container::Style {
        background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
        ..Default::default()
    })
    .into()
}