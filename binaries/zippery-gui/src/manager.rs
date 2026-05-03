use std::path::PathBuf;
use iced::{Color, ContentFit, Element, Task};
use iced::Length::Fill;
use iced::widget::image::Handle;
use iced::widget::{column, container, image, mouse_area, row, scrollable, space, stack, text};

mod background;
mod config;
mod error;
mod file;
mod message;

use config::Config;
use file::{FileItem, rename, sort_files, view_file_row, view_header};
use message::Message;
use background::load_background;

pub struct Manager {
    background: Option<Handle>,
    pub config: Config,
    pub current_dir: std::path::PathBuf,
    files: Vec<FileItem>,
    hovered: Option<usize>,
    selected: Vec<usize>,
    renaming: Option<usize>,
    buffer: String,
    error: Option<String>,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            background: None,
            config: Config::default(),
            current_dir: dirs::desktop_dir().or_else(|| dirs::home_dir()).unwrap_or_default(),
            files: vec![],
            selected: vec![],
            renaming: None,
            hovered: None,
            buffer: String::new(),
            error: None,
        }
    }
}

impl Manager {
    pub fn new() -> (Self, Task<Message>) {
        let mut m = Self::default();
        m.load_config();
        m.read_dir();
        sort_files(&mut m.files, &m.config.sort);
        let task = m.config.ui.bg_cache.as_ref().map(|bg| {
                Task::done(Message::LoadBackground(Some(PathBuf::from(bg))))
            }).unwrap_or(Task::none());
        (m, task)
    }

    pub fn load_config(&mut self){
        if let Ok(bytes) = std::fs::read("config.bin") {
            if let Ok(cfg) = bitcode::decode::<Config>(&bytes) {
                self.config = cfg;
            }
        }
    }

    fn save_config(&self) {
        let bytes = bitcode::encode(&self.config);
        let _ = std::fs::write("config.bin", bytes);
    }

    fn read_dir(&mut self) {
        self.files = file::read_dir(&self.current_dir);
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut task = Task::none();
        match message {
            Message::Menu(menu_action) => match menu_action.as_str() {
                "File" => {
                    let _ = iced::window::open(iced::window::Settings {
                        size: iced::Size::new(400.0, 150.0),
                        position: iced::window::Position::Centered,
                        ..Default::default()
                    });
                }
                _ => {}
            },

            Message::Goup => {
                if let Some(parent) = self.current_dir.parent() {
                    self.current_dir = parent.to_path_buf();
                    self.read_dir();
                }
            }

             Message::Hover(i) => {
                self.hovered = Some(i);
            }

            Message::Unhover(i) => {
                if self.hovered == Some(i) {
                    self.hovered = None;
                }
            }
            
            Message::Click(index) => {
                if self.selected.len() != 1 || index != self.selected[0] {
                    self.selected = vec![index];
                    self.renaming = None;
                } else {
                    if let Some(file) = self.files.get(index) {
                        self.renaming = Some(index);
                        self.buffer = file.path.file_name()
                        .map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                    }
                }
            }

            Message::Open(index) => {
                if let Some(file) = self.files.get(index) {
                    let path = &file.path;
                    if file.path.is_dir() {
                        self.current_dir = file.path.clone();
                        self.read_dir();
                    } else {
                        let _ = open::that(path);
                    }
                    self.renaming = None;
                }
            }

            Message::ClearSelection => {
                self.selected.clear();
                self.renaming = None;
            }

            Message::Rename(index) => {
                if let Some(file) = self.files.get_mut(index) {
                    match rename(&file.path, &self.buffer) {
                        Ok(path) => {
                            file.path = path;
                            self.renaming = None;
                            self.buffer.clear();
                        }
                        Err(e) => {
                            self.error = Some(e.to_string());
                        }
                    }
                }
            }

            Message::Input(s) => self.buffer = s,

            Message::CloseError => self.error = None,

            Message::SortBy(new_col) => {
                let (old_col, asc) = &self.config.sort;
                if new_col == *old_col {
                    self.config.sort = (new_col, !asc);
                } else {
                    self.config.sort = (new_col, *asc);
                }
                sort_files(&mut self.files, &self.config.sort);
            }

            Message::PickBackground => {
                task = Task::perform(async {
                    let file = rfd::FileDialog::new()
                        .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
                        .pick_file();

                    file
                    }, |path| {
                    Message::LoadBackground(path)
                });
            }

            Message::LoadBackground(Some(path)) => {
                task = load_background(path);
            }

           Message::BackgroundLoaded(Some((path, handle))) => {
                self.config.ui.bg_cache = Some(path.to_string_lossy().to_string());
                self.background = Some(handle);
            }
            _ => {}
        }
        task
}

    pub fn view(&self) -> Element<'_, Message> {
        let background: Option<image::Image<_>> = self.background.as_ref().map(|handle| {
            image(handle.clone())
                .content_fit(ContentFit::Cover)
                .width(Fill)
                .height(Fill)
        });
        let background = match background {
            Some(bg) => container(bg),
            None => container(space()).width(Fill).height(Fill),
        };

        let file_button = mouse_area(
        container(text("File").size(11)).padding(0).center_x(35)
        ).on_press(Message::Menu("File".to_string()));

        let custom_button = mouse_area(
        container(text("Custom").size(11)).padding(0).center_x(35)
        ).on_press(Message::PickBackground);

        let go_up_button = mouse_area(text("◀").size(12))
            .on_press(Message::Goup);

        let current_path = container(text(format!("📂 {}", self.current_dir.display()))
        .size(12)).height(14).style(|_theme| container::Style {
            border: iced::Border {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2).into(),
            width: 1.0,
            radius: 0.into(),
            },
            ..Default::default()
        }).width(Fill);
        
        let tool_bar = row![ file_button, custom_button ];
        let address_bar = row![ go_up_button, current_path ].spacing(2);
        let view_header = view_header(&self.config.sort, &self.config.ui.column);
        let file_list = column(self.files.iter().enumerate().map(|(i, f)| {
            view_file_row(i, f, &self.config.ui.column,
                &self.hovered, &self.selected, &self.renaming, &self.buffer)
        })).spacing(0);

        stack![
            background,

            mouse_area(container(space()).width(Fill).height(Fill))
                .on_press(Message::ClearSelection),

            column![
                tool_bar,
                address_bar,
                view_header,
                scrollable(file_list).width(Fill),
            ].spacing(0),

            self.error.as_ref().map(error::error_modal_view)
        ]
        .into()
    }
}



impl Drop for Manager {
    fn drop(&mut self) {
        self.save_config();
    }
}