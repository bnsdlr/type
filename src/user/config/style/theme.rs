use super::ratatui_wrappers;

use crate::app::quick_menu::QuickMenuItem;

use ratatui::style::Color;
use serde::{Deserialize, Serialize, de, ser};

use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct ThemeSerializer {
    bg: Option<ratatui_wrappers::Color>,
    main: Option<ratatui_wrappers::Color>,
    caret: Option<ratatui_wrappers::Color>,
    text: Option<ratatui_wrappers::Color>,
    sub: Option<ratatui_wrappers::Color>,
    sub_alt: Option<ratatui_wrappers::Color>,
    error: Option<ratatui_wrappers::Color>,
    error_extra: Option<ratatui_wrappers::Color>,
    colorful_error: Option<ratatui_wrappers::Color>,
    colorful_error_extra: Option<ratatui_wrappers::Color>,
    untyped_letter: Option<ratatui_wrappers::Color>,
}

impl ThemeSerializer {
    fn theme(self, name: String) -> Theme {
        let defaults = Theme::default();

        let bg = self.bg.map(Color::from).unwrap_or(defaults.bg);
        let main = self.main.map(Color::from).unwrap_or(defaults.main);
        let caret = self.caret.map(Color::from).unwrap_or(defaults.caret);
        let text = self.text.map(Color::from).unwrap_or(defaults.text);
        let sub = self.sub.map(Color::from).unwrap_or(defaults.sub);
        let sub_alt = self.sub_alt.map(Color::from).unwrap_or(defaults.sub_alt);
        let error = self.error.map(Color::from).unwrap_or(defaults.error);
        let error_extra = self
            .error_extra
            .map(Color::from)
            .unwrap_or(defaults.error_extra);
        let colorful_error = self
            .colorful_error
            .map(Color::from)
            .unwrap_or(defaults.colorful_error);
        let colorful_error_extra = self
            .colorful_error_extra
            .map(Color::from)
            .unwrap_or(defaults.colorful_error_extra);
        let untyped_letter = self
            .untyped_letter
            .map(Color::from)
            .unwrap_or(defaults.untyped_letter);

        Theme {
            name,
            bg,
            main,
            caret,
            text,
            sub,
            sub_alt,
            error,
            error_extra,
            colorful_error,
            colorful_error_extra,
            untyped_letter,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub main: Color,
    pub caret: Color,
    pub text: Color,
    pub sub: Color,
    pub sub_alt: Color,
    pub error: Color,
    pub error_extra: Color,
    pub colorful_error: Color,
    pub colorful_error_extra: Color,
    pub untyped_letter: Color,
}

impl Theme {
    fn get_dir_path() -> PathBuf {
        PathBuf::from(crate::DATA_DIR).join("themes")
    }

    pub fn get_path(name: &str) -> PathBuf {
        Theme::get_dir_path().join(format!("{name}.json"))
    }

    pub fn load(name: String) -> crate::Result<Self> {
        let path = Self::get_path(&name);
        let content = fs::read(path)?;
        let theme_serializer: ThemeSerializer = serde_json::from_slice(&content)?;
        let theme = theme_serializer.theme(name);
        Ok(theme)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Theme, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let name: String = de::Deserialize::deserialize(deserializer)?;
        Ok(Self::load(name).expect("could not deserialize theme"))
    }

    pub fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.name)
    }

    pub fn all() -> crate::Result<Vec<Theme>> {
        let mut themes = Vec::new();

        for entry in fs::read_dir(Theme::get_dir_path())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name) = name.to_str() {
                    if let Some((file_name, extension)) = name.rsplit_once(".") {
                        if extension == "json" {
                            themes.push(Theme::load(file_name.to_string())?);
                        }
                    }
                }
            }
        }

        Ok(themes)
    }

    pub fn all_quick_menu_items() -> crate::Result<QuickMenuItem> {
        Ok(QuickMenuItem::from_iter(Self::all()?))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            bg: Color::Black,
            main: Color::Yellow,
            caret: Color::LightYellow,
            text: Color::White,
            sub: Color::Gray,
            sub_alt: Color::Indexed(244),
            error: Color::Red,
            error_extra: Color::LightRed,
            colorful_error: Color::Magenta,
            colorful_error_extra: Color::LightMagenta,
            untyped_letter: Color::DarkGray,
        }
    }
}
