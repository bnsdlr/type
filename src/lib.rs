pub mod app;
pub mod monkeytype;

pub use app::App;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
