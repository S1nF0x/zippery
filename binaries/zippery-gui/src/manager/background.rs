use std::path::PathBuf;
use iced::{Task, widget::image::Handle};
use super::message::Message;
use image;

pub fn load_background(path: PathBuf) -> Task<Message> {
    Task::perform(
        async move {
            let img = image::open(&path).ok()?;
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();

            Some((path, Handle::from_rgba(w, h, rgba.into_raw())))
        },
        Message::BackgroundLoaded,
    )
}