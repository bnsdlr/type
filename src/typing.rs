pub mod mode;
pub mod statistics;

pub use mode::{Mode, QuoteLength, Seconds, WordCount};
pub use statistics::TestStatistics;

use crate::monkeytype::{Language, MonkeyType};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

use std::collections::HashSet;

pub struct TestState {
    language: Language,
    mode: Mode,
    test_text: String,
    typed_text: Vec<char>,
    was_typed_wrong: HashSet<usize>,
    monkey: MonkeyType,
    statistics: TestStatistics,
}

impl TestState {
    pub fn new() -> crate::Result<Self> {
        let language = Language::default();
        Ok(Self {
            monkey: MonkeyType::new(language)?,
            was_typed_wrong: HashSet::new(),
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
            Mode::Quote { lengths } => {
                let quote = self.monkey.random_quote(lengths)?;
                quote.text.clone()
            }
            Mode::Words { word_count, punctuation, numbers } => {
                if let Some(words) = self.monkey.random_words(word_count, *punctuation, *numbers) {
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

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> crate::Result<()> {
        match key_event.kind {
            KeyEventKind::Press => match key_event.modifiers {
                KeyModifiers::SHIFT | KeyModifiers::NONE => match key_event.code {
                    KeyCode::Char(c) => {
                        let current_index = self.typed_text.len();
                        if let Some(actual_c) = self.test_text.chars().nth(current_index) {
                            self.statistics.new_char(current_index, c, actual_c);
                            if c != actual_c {
                                self.was_typed_wrong.insert(current_index);
                            }
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
                },
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    pub fn render_options(&self, style: &crate::Style, area: Rect, buf: &mut Buffer) {}

    pub fn render(&self, style: &crate::Style, area: Rect, buf: &mut Buffer) {
        if self.typed_text.len() >= self.test_text.len() {
            self.statistics.render_end(area, buf);
        } else {
            let [statistics, body] =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

            self.statistics.render(statistics, buf);

            let mut text = Vec::with_capacity(self.test_text.len());
            let typed_text_len = self.typed_text.len();

            for (i, c) in self.test_text.chars().enumerate() {
                let color = {
                    if i < typed_text_len {
                        if c == self.typed_text[i] {
                            if self.was_typed_wrong.contains(&i) {
                                Style::new().underlined().underline_color(style.theme.error_extra)
                            } else {
                                Style::new().fg(style.theme.text)
                            }
                        } else {
                            Style::new().fg(style.theme.error)
                        }
                    } else if i == typed_text_len {
                        Style::new().fg(style.theme.untyped_letter).bg(style.theme.caret)
                    } else {
                        Style::new().fg(style.theme.untyped_letter)
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
                .render(body, buf);
        }
    }
}
