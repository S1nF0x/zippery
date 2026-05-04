use std::path::PathBuf;
use iced::widget::image::Handle;

use super::config::SortColumn;
#[derive(Debug, Clone)]
pub enum Request {
    OpenAbout,
    // future requests
}

#[derive(Debug, Clone)]
pub enum Message {
    Request(Request),
    Click(usize),
    ClearSelection,
    Hover(usize),
    Unhover(usize),
    Open(usize),
    Rename(usize),
    Input(String),
    Goup,
    CloseError,
    SortBy(SortColumn),
    //Error(String)  For future error handling.
    PickBackground,
    LoadBackground(Option<PathBuf>),
    BackgroundLoaded(Option<(PathBuf, Handle)>)
}