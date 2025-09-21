pub mod quote;
pub mod language;

pub use quote::QuoteLanguage;
pub use language::Language;

const DATA_DIR: &str = "data";

pub enum Mode {

}

pub struct Text {
    language: Language,
    mode: Mode,
    data: Option<String>,
}
