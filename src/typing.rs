pub mod mode;

pub use mode::{Mode, QuoteLength, WordCount, Seconds};

use crate::monkeytype::{MonkeyType, Language, QuoteLanguage};

pub struct TestState {
    language: Language,
    mode: Mode,
    test_text: Vec<char>,
    text_typed: Vec<char>,
    monkey: MonkeyType,
}

impl TestState {
    pub fn new() -> crate::Result<Self> {
        let language = Language::default();
        Ok(Self {
            monkey: MonkeyType::new(language)?,
            language,
            mode: Mode::default(),
            test_text: Vec::new(),
            text_typed: Vec::new(),
        })
    }

    pub fn new_test(&mut self) {

    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}
