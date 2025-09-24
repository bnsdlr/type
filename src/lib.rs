#![feature(try_trait_v2)]

pub mod app;
pub mod monkeytype;
pub mod user;
pub mod typing;
pub mod error;

pub use app::App;
pub use error::{Error, Result};

pub const STATE_DIR: &str = "state";
pub const DATA_DIR: &str = "data";
