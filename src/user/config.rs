pub mod style;

pub use style::{Style, Theme};

use serde::{Deserialize, Serialize};

use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub style: Style,
}

impl Config {
    pub fn load() -> crate::Result<Self> {
        let config_slice = fs::read(PathBuf::from(crate::CONFIG_DIR).join(crate::CONFIG_FILE)).ok();

        if let Some(slice) = config_slice {
            serde_json::from_slice(&slice).map_err(|err| crate::Error::ParsingConfig(Box::new(err)))
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            style: Style::default(),
        }
    }
}
