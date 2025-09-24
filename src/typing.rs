pub mod mode;

pub use mode::{Mode, QuoteLength, WordCount, Seconds};

use crate::monkeytype::{MonkeyType, Language, QuoteLanguage};

use ratatui::{
    prelude::*,
    widgets::Widget
};

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

    pub fn new_test(&mut self) -> crate::Result<()> {
        if self.monkey.language != self.language {
            self.monkey.set_language(self.language)?;
        }

        let test_text = match &self.mode {
            Mode::Quote(lens) => {
                let quote = self.monkey.random_quote(lens);
            }
            _ => todo!(),
        };

        Ok(())
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}

impl Widget for TestState {
    fn render(self, area: Rect, buf: &mut Buffer) {

    }
}
