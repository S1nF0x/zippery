use bitcode::{Encode, Decode};

#[derive(Clone, Decode, Encode)]
pub struct ColumnState {
    pub name_width: f32,
    pub size_width: f32,
    pub date_width: f32,
}

#[derive(Clone, Decode, Encode)]
pub struct ThemeConfig {
    pub theme: ThemeKind,
    pub follow_system: bool
}

#[derive(Clone, Default, Decode, Encode)]
pub struct UiConfig {
    pub column: ColumnState,
    pub bg_cache: Option<String>,
    pub theme: ThemeConfig
}


#[derive(Clone, Default, Decode, Encode)]
pub struct Config {
    pub ui: UiConfig,
    pub sort: (SortColumn, bool)
}

#[derive(Debug, Clone, PartialEq, Decode, Encode)]
pub enum SortColumn {
    Name,
    Size,
    Modified
}

impl Default for SortColumn {
    fn default() -> Self {
        SortColumn::Name
    }
}
impl Default for ThemeConfig {
    fn default() -> Self {
        Self { theme: ThemeKind::Light, follow_system: true}
    }
}

impl Default for ColumnState {
    fn default() -> Self {
        Self { name_width: 300.0, size_width: 100.0, date_width: 150.0 }
    }
}

#[derive(Clone, Decode, Encode)]
pub enum ThemeKind {
    Light,
    Dark
}