pub mod app;
pub mod monkeytype;
pub mod user;
pub mod typing;

pub use app::App;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const STATE_DIR: &str = "state";
pub const DATA_DIR: &str = "data";
