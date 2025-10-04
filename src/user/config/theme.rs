pub mod color;

use ratatui::style::Color;
use serde::{Deserialize, Serialize, de, ser};

use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct ThemeSerializer {
    bg: Option<color::Color>,
    main: Option<color::Color>,
    caret: Option<color::Color>,
    text: Option<color::Color>,
    sub: Option<color::Color>,
    sub_alt: Option<color::Color>,
    error: Option<color::Color>,
    error_extra: Option<color::Color>,
    colorful_error: Option<color::Color>,
    colorful_error_extra: Option<color::Color>,
    untyped_letter: Option<color::Color>,
}

impl ThemeSerializer {
    fn to_theme(self, name: String) -> Theme {
        let defaults = Theme::default();

        let bg = self.bg.map(|c| Color::from(c)).unwrap_or(defaults.bg);
        let main = self.main.map(|c| Color::from(c)).unwrap_or(defaults.main);
        let caret = self.caret.map(|c| Color::from(c)).unwrap_or(defaults.caret);
        let text = self.text.map(|c| Color::from(c)).unwrap_or(defaults.text);
        let sub = self.sub.map(|c| Color::from(c)).unwrap_or(defaults.sub);
        let sub_alt = self
            .sub_alt
            .map(|c| Color::from(c))
            .unwrap_or(defaults.sub_alt);
        let error = self.error.map(|c| Color::from(c)).unwrap_or(defaults.error);
        let error_extra = self
            .error_extra
            .map(|c| Color::from(c))
            .unwrap_or(defaults.error_extra);
        let colorful_error = self
            .colorful_error
            .map(|c| Color::from(c))
            .unwrap_or(defaults.colorful_error);
        let colorful_error_extra = self
            .colorful_error_extra
            .map(|c| Color::from(c))
            .unwrap_or(defaults.colorful_error_extra);
        let untyped_letter = self
            .untyped_letter
            .map(|c| Color::from(c))
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

#[derive(Clone)]
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
        let theme = theme_serializer.to_theme(name);
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
        S: ser::Serializer {
        serializer.serialize_str(&self.name)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            bg: Color::Black,
            main: Color::White,
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
