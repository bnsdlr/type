use ratatui::text::Text;
use ratatui::style::{Color, Stylize};

#[derive(PartialEq, Eq)]
pub enum Tab {
    Main,
    Typing,
    Help,
    Settings,
}

impl Tab {
    pub fn from_number(num: usize) -> Option<Tab> {
        match num {
            n if n == Tab::Main as usize => Some(Tab::Main),
            n if n == Tab::Typing as usize => Some(Tab::Typing),
            n if n == Tab::Help as usize => Some(Tab::Help),
            n if n == Tab::Settings as usize => Some(Tab::Settings),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Main => "Main",
            Self::Typing => "Typing",
            Self::Help => "Help",
            Self::Settings => "Settings",
        }
    }

    pub fn as_text_element(&self, current_tab: &Tab) -> Text<'_> {
        let is_current = *self == *current_tab;
        let fg = if is_current {
            Color::Cyan
        } else {
            Color::White
        };
        let mut text = Text::from(self.to_string()).fg(fg);
        if is_current {
            text = text.bold();
        }
        text
    }
}
