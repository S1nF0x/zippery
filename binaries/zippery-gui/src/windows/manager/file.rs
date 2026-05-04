use iced::{Color, Element, Theme, alignment};
use iced::widget::container::Style;
use iced::widget::image::Handle;
use iced::widget::{button, container, image, mouse_area, row, rule, text, text_input};
use windows_icons::get_icon_by_path;
use std::path::PathBuf;
use std::{fs, io};
use std::time::SystemTime;
use chrono::{DateTime, Local};

use super::config::{ColumnState, SortColumn};
use super::message::Message;

#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub size: Option<usize>,
    pub modified: Option<SystemTime>,
}

impl FileItem {
    pub fn display_modified(&self) -> String {
        self.modified
            .map(format_time)
            .unwrap_or_default()
    }
}

fn vr<'a>() -> Element<'a, Message> {
    rule::vertical(0.4).style(|_theme| rule::Style {
            color: iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.3, 
            },
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
        .into()
    
}

fn format_time(t: SystemTime) -> String {
    let dt: DateTime<Local> = t.into();
    dt.format("%Y-%m-%d %H:%M").to_string()
}

pub fn read_dir(dir: &PathBuf) -> Vec<FileItem> {
    let file_items: Vec<FileItem> = std::fs::read_dir(dir).map(|entries|{
            entries.flatten().map(|entry| {
                let path =entry.path();
                if let Ok(meta) = path.metadata() {
                    if path.is_dir(){
                        let modified = meta.modified().ok();
                        FileItem {
                            path,
                            size: None,
                            modified
                        }
                    }else{
                        let size = Some(meta.len() as usize);
                        let modified = meta.modified().ok();
                        FileItem {
                            path,
                            size,
                            modified
                        }
                    }
                }
                else{
                    FileItem { path, size: None, modified: None }
                }
            }).collect()}).unwrap_or_default();
    file_items
}

pub fn sort_files(files: &mut Vec<FileItem>, config: &(SortColumn, bool)) {
    files.sort_by(|a, b| {
        let dir_order = b.path.is_dir().cmp(&a.path.is_dir());
        if dir_order != std::cmp::Ordering::Equal {
            return dir_order;
        }
        let (col, dsc) = config;
        let order = match col {
            SortColumn::Name => {
                let a_name = a.path.file_name().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
                let b_name = b.path.file_name().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
                a_name.cmp(&b_name)
            }
            SortColumn::Size => {
                a.size.unwrap_or(0).cmp(&b.size.unwrap_or(0))
            }
            SortColumn::Modified => {
                a.modified.unwrap_or(std::time::UNIX_EPOCH)
                    .cmp(&b.modified.unwrap_or(std::time::UNIX_EPOCH))
            }
        };

        if !*dsc { order } else { order.reverse() }
    });
}

pub fn rename(old_path: &PathBuf, new_name: &str) -> Result<PathBuf, io::Error> {
    if new_name.trim().is_empty() {
        return Ok(old_path.clone());
    }

    let parent = old_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No parent directory"))?;

    let new_path = parent.join(new_name);

    fs::rename(old_path, &new_path)?;

    Ok(new_path)
}

pub fn view_header<'a>(sort: &'a (SortColumn, bool), cols: &'a ColumnState ) -> Element<'a, Message> {
    let sort_indicator = |col: &SortColumn| -> &'static str {
        match sort {
            (c, dsc) if c == col => if *dsc { "▼" } else { "▲" },
            _ => "",
        }
    };

    row![
        button(text(format!("Name {}", sort_indicator(&SortColumn::Name))).size(11))
            .on_press(Message::SortBy(SortColumn::Name))
            .style(button::text).width(cols.name_width),
        button(text(format!("Size {}", sort_indicator(&SortColumn::Size))).size(11).align_x(alignment::Horizontal::Right))
            .on_press(Message::SortBy(SortColumn::Size))
            .style(button::text).width(cols.size_width),
        button(text(format!("Modified {}", sort_indicator(&SortColumn::Modified))).size(11).align_x(alignment::Horizontal::Left))
            .on_press(Message::SortBy(SortColumn::Modified))
            .style(button::text).width(cols.date_width),
    ].padding(0).into()
}

pub fn view_file_row<'a>(index: usize, file: &'a FileItem, cols: &'a ColumnState, hovered: &Option<usize>, selected: &Vec<usize>, renaming: &Option<usize>, buffer: &String) -> Element<'a, Message> {
    let name = file.path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let icon = get_icon_by_path(&file.path).unwrap();

    let (w, h) = icon.dimensions();
    let handle = Handle::from_rgba(w, h, icon.into_raw());
    let is_renaming = *renaming == Some(index);
    let is_hovered = *hovered == Some(index);
    let name_widget: Element<Message> = if is_renaming {
        text_input("", buffer)
            .on_input(|s| Message::Input(s))
            .on_submit(Message::Rename(index)).size(12).padding(0).into()
        } else {
        text(name).size(12).into()
    };
    let name_col_width = cols.name_width - 32.0;

    let content =container(row![
        image(handle).width(16).height(16),

        container(name_widget).width(name_col_width).height(14).clip(true),

        vr(),

        text(file.size.map(|n| n.to_string()).unwrap_or_default())
            .size(12)
            .width(cols.size_width)
            .height(12)
            .align_x(alignment::Horizontal::Right),

        vr(),

        text(file.display_modified())
            .size(12)
            .width(cols.date_width)
            .align_x(alignment::Horizontal::Left),

        vr()
    ].spacing(2)).style(file_style(selected.contains(&index), is_hovered));
    
    mouse_area(content)
        .on_enter(Message::Hover(index))
        .on_exit(Message::Unhover(index))
        .on_press(Message::Click(index))
        .on_double_click(Message::Open(index))
        .into()
}

fn file_style(is_selected: bool, is_hovered: bool) -> impl Fn(&Theme) -> Style {
    move |_theme| {
        if is_selected {
            Style {
                background: Some(Color::from_rgba(0.82, 0.90, 0.99, 0.7).into()),
                ..Style::default()
            }
        } else if is_hovered{
            Style{
                background: Some(Color::from_rgba(0.82, 0.90, 0.99, 0.35).into()),
                ..Style::default()
            }
        }
        else{ Style::default() }
    }
}