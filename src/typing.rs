pub mod mode;
pub mod statistics;

pub use mode::{Mode, QuoteLength, Seconds, WordCount};
pub use statistics::TestStatistics;

use crate::monkeytype::{Language, MonkeyType};

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{Block, Paragraph, Wrap},
};

pub struct TestState {
    language: Language,
    mode: Mode,
    test_text: String,
    typed_text: Vec<char>,
    monkey: MonkeyType,
    statistics: TestStatistics,
}

impl TestState {
    pub fn new() -> crate::Result<Self> {
        let language = Language::default();
        Ok(Self {
            monkey: MonkeyType::new(language)?,
            language,
            mode: Mode::default(),
            test_text: String::new(),
            typed_text: Vec::new(),
            statistics: TestStatistics::new(),
        })
    }

    pub fn new_test(&mut self) -> crate::Result<()> {
        if self.monkey.language != self.language {
            self.monkey.set_language(self.language)?;
        }

        self.typed_text = Vec::new();

        self.test_text = match &self.mode {
            Mode::Quote(lens) => {
                let quote = self.monkey.random_quote(lens)?;
                quote.text.clone()
            }
            Mode::Words(word_count) => {
                if let Some(words) = self.monkey.random_words(word_count) {
                    words
                        .iter()
                        .map(|word| word.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                } else {
                    return Err(crate::Error::NoWordsForLanguage(self.language));
                }
            }
            _ => todo!(),
        };

        self.statistics.reset();

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

    pub fn handle_key_press(&mut self, key_code: KeyCode) -> crate::Result<()> {
        match key_code {
            KeyCode::Char(c) => {
                let current_index = self.typed_text.len();
                if let Some(actual_c) = self.test_text.chars().nth(current_index) {
                    self.statistics.new_char(current_index, c, actual_c);
                }
                if current_index == self.test_text.len() {
                    self.statistics.end();
                }
                self.typed_text.push(c);
            }
            KeyCode::Backspace => {
                let _ = self.typed_text.pop();
            }
            KeyCode::Tab => self.new_test()?,
            _ => (),
        }

        Ok(())
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.typed_text.len() >= self.test_text.len() {
            self.statistics.render_end(area, buf);
        } else {
            let [statistics, body] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                .areas(area);

            self.statistics.render(statistics, buf);

            let mut text = Vec::with_capacity(self.test_text.len());
            let typed_text_len = self.typed_text.len();

            for (i, c) in self.test_text.chars().enumerate() {
                let color = {
                    if i < typed_text_len {
                        if c == self.typed_text[i] {
                            Style::new().fg(Color::White)
                        } else {
                            Style::new().fg(Color::Red)
                        }
                    } else if i == typed_text_len {
                        Style::new().fg(Color::DarkGray).bg(Color::Gray)
                    } else {
                        Style::new().fg(Color::DarkGray)
                    }
                };
                if c == ' ' {
                    text.push(Span::styled("·", color));
                } else {
                    text.push(Span::styled(c.to_string(), color));
                }
            }

            let text = Line::from_iter(text);

            Paragraph::new(text)
                .wrap(Wrap { trim: true })
                .block(Block::bordered())
                .render(body, buf);
        }
    }
}
