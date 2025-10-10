pub mod app;
pub mod monkeytype;
pub mod user;
pub mod typing;
pub mod error;

pub use app::App;
pub use error::{Error, Result};
pub use user::{Config, config::style::{Theme, Style}};

pub const DATA_DIR: &str = "data";
pub const CONFIG_DIR: &str = "config";
pub const CONFIG_FILE: &str = "config.json";

pub const CHARS_PER_WORD: f32 = 5.;
