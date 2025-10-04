use crate::Theme;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Clear, Paragraph},
};

pub struct QuickMenu {
    visible: bool,
    input: Vec<char>,
    current_index: usize,
    history: Vec<Vec<char>>,
}

impl QuickMenu {
    pub fn new() -> Self {
        QuickMenu {
            visible: false,
            input: Vec::new(),
            current_index: 0,
            history: Vec::new(),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> crate::Result<()> {
        match key_event.kind {
            KeyEventKind::Press => match key_event.modifiers {
                KeyModifiers::SHIFT | KeyModifiers::NONE => match key_event.code {
                    KeyCode::Char(char) => {
                        self.input.insert(self.current_index, char);
                        self.current_index = self.current_index.saturating_add(1);
                    }
                    KeyCode::Backspace => {
                        if self.current_index != 0 {
                            self.current_index = self.current_index.saturating_sub(1);
                            self.input.remove(self.current_index);
                        }
                    }
                    _ => (),
                },
                KeyModifiers::CONTROL => match key_event.code {
                    KeyCode::Char('h') => self.current_index = self.current_index.saturating_sub(1),
                    KeyCode::Char('l') => {
                        self.current_index = self
                            .current_index
                            .saturating_add(1)
                            .min(self.input.len())
                    }
                    KeyCode::Char('a') => self.current_index = 0,
                    KeyCode::Char('e') => self.current_index = self.input.len(),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        Ok(())
    }

    pub fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        if self.visible {
            let horizontal =
                Layout::horizontal([Constraint::Length(80)]).flex(layout::Flex::Center);
            let vertical = Layout::vertical([Constraint::Max(8), Constraint::Length(10)]);
            let [_, area] = vertical.areas(area);
            let [area] = horizontal.areas(area);

            let [input, list] =
                Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

            let block = Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(theme.main);

            Clear.render(input, buf);

            let search_input = if self.input.is_empty() {
                Line::from("   Search...").fg(theme.untyped_letter)
            } else {
                let mut text = Vec::with_capacity(self.input.len());
                text.push(Span::raw("   "));

                for (i, char) in self.input.iter().enumerate() {
                    let color = if i == self.current_index {
                        Style::new().fg(theme.text).bg(theme.caret)
                    } else {
                        Style::new().fg(theme.text)
                    };
                    text.push(Span::styled(char.to_string(), color))
                }

                if self.current_index >= self.input.len() {
                    text.push(Span::styled(" ", Style::new().bg(theme.caret)));
                }

                Line::from_iter(text)
            };

            Paragraph::new(search_input).block(block).render(input, buf);

            Clear.render(list, buf);
            Paragraph::new(Line::from(format!(
                "current_index: {}, len: {}",
                self.current_index,
                self.input.len()
            )))
            .render(list, buf);
        }
    }
}
