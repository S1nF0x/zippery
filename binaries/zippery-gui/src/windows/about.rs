use iced::{
    Alignment,Border, Color, Element, Task,
    widget::{column, container, mouse_area, row, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    Close,
    Homepage,
    Hover(usize, bool)
}

#[derive(Debug, Clone)]
pub struct About {
    hovered: Option<usize>,
}

impl About {
    pub fn new() -> Self {
        Self { hovered: None }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Hover(id, true) => {
                self.hovered = Some(id);
            }
            Message::Hover(_, false) => {
                self.hovered = None;
            }
            Message::Homepage => {
                let _ = open::that("https://github.com/S1nF0x/zippery");
            }
            Message::Close => {}
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Zippery").size(20),
            text("Version 0.1.1"),
            text("Made with ❤"),
            row![simple_button("OK", Message::Close, 0, self.hovered), simple_button("Source Code", Message::Homepage, 1, self.hovered)]
            .spacing(20)
        ].align_x(Alignment::Center)
        .spacing(10)
        .into()
    }
}

fn simple_button<'a>(
    label: &'a str,
    msg: Message,
    id: usize,
    hovered: Option<usize>,
) -> Element<'a, Message> {
    let is_hovered = hovered == Some(id);

    mouse_area(
        container(text(label)).center_x(100)
            .style(move |_| container::Style {
                background: if is_hovered {
                    Some(Color::from_rgba(0.1, 0.1, 0.1, 0.2).into())
                } else {
                    None
                },
                border: Border {
                    width: 1.0,
                    color: Color::from_rgb8(100, 100, 100),
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
    )
    .on_enter(Message::Hover(id, true))
    .on_exit(Message::Hover(id, false))
    .on_press(msg)
    .into()
}
