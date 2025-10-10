pub mod theme;
mod ratatui_wrappers;

pub use theme::Theme;

use serde::{Deserialize, Serialize};
use ratatui::widgets::BorderType;

#[derive(Default, Deserialize, Serialize)]
pub struct Style {
    #[serde(
        deserialize_with = "Theme::deserialize",
        serialize_with = "Theme::serialize"
    )]
    pub theme: Theme,
    #[serde(
        deserialize_with = "ratatui_wrappers::BorderType::deserialize",
        serialize_with = "ratatui_wrappers::BorderType::serialize"
    )]
    pub border_type: BorderType,
}
