pub mod item;

pub use item::QuickMenuItem;

use crate::{Theme, Config};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, List, ListState, Paragraph},
};

pub struct QuickMenu {
    visible: bool,
    input: Vec<char>,
    current_index: usize,
    history: Vec<Vec<char>>,
    list_state: ListState,
    options: QuickMenuItem,
}

impl QuickMenu {
    pub fn new() -> Self {
        let options = Theme::all_quick_menu_items()
            .unwrap()
            .label(Some("themes".to_string()));
        QuickMenu {
            visible: false,
            input: Vec::new(),
            current_index: 0,
            history: Vec::new(),
            list_state: ListState::default(),
            options,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, config: &mut Config) -> crate::Result<()> {
        match key_event.kind {
            KeyEventKind::Press => match key_event.modifiers {
                KeyModifiers::CONTROL => match key_event.code {
                    KeyCode::Char('h') => self.current_index = self.current_index.saturating_sub(1),
                    KeyCode::Char('l') => {
                        self.current_index =
                            self.current_index.saturating_add(1).min(self.input.len())
                    }
                    KeyCode::Char('a') => self.current_index = 0,
                    KeyCode::Char('e') => self.current_index = self.input.len(),
                    // FIXME: j does somehow not work... I don't know why...
                    KeyCode::Char('n') | KeyCode::Char('j') => self.list_state.select_next(),
                    KeyCode::Char('p') | KeyCode::Char('k') => self.list_state.select_previous(),
                    // FIXME: G does not work
                    KeyCode::Char('G') => self.list_state.select_last(),
                    KeyCode::Char('g') => self.list_state.select_first(),
                    _ => (),
                },
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
                    KeyCode::Enter => {
                        self.select(config);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        Ok(())
    }

    fn select(&self, config: &mut Config) {
        if let Some(current) = self.list_state.selected() {
        }
    }

    fn format_input(&self) -> Vec<String> {
        let (words, _) = self
            .input
            .iter()
            .fold((Vec::new(), 0), |(mut acc, mut index), char| {
                while acc.get(index).is_none() {
                    acc.push(String::new());
                }
                if *char != ' ' {
                    acc[index].push(*char);
                } else {
                    index += 1;
                }
                (acc, index)
            });

        words
    }

    pub fn render(&mut self, style: &crate::Style, area: Rect, buf: &mut Buffer) {
        if self.visible {
            let list_width = 80;
            let list_item_width = area.width.min(list_width);
            let words = self.format_input();
            let items = self.options.as_list_items_filtered(None, list_item_width, &words);

            let list_area_len = items.len().clamp(3, 30);

            let horizontal =
                Layout::horizontal([Constraint::Length(list_width)]).flex(layout::Flex::Center);
            let vertical = Layout::vertical([
                Constraint::Max(8),
                Constraint::Length(3 + list_area_len as u16),
            ]);
            let [_, area] = vertical.areas(area);
            let [area] = horizontal.areas(area);

            let [input_area, list_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

            let block = Block::bordered()
                .border_type(style.border_type)
                .border_style(style.theme.main);

            Clear.render(input_area, buf);

            let search_input = if self.input.is_empty() {
                Line::from("   Search...").fg(style.theme.untyped_letter)
            } else {
                let mut text = Vec::with_capacity(self.input.len());
                text.push(Span::raw("   "));

                for (i, char) in self.input.iter().enumerate() {
                    let color = if i == self.current_index {
                        Style::new().fg(style.theme.text).bg(style.theme.caret)
                    } else {
                        Style::new().fg(style.theme.text)
                    };
                    text.push(Span::styled(char.to_string(), color))
                }

                if self.current_index >= self.input.len() {
                    text.push(Span::styled(" ", Style::new().bg(style.theme.caret)));
                }

                Line::from_iter(text)
            };

            Paragraph::new(search_input)
                .block(block)
                .render(input_area, buf);

            if !items.is_empty() {
                Clear.render(list_area, buf);

                let item_list = List::new(items)
                    .block(
                        Block::bordered()
                            .border_type(style.border_type)
                            .border_style(style.theme.main),
                    )
                    .highlight_style(
                        Style::new()
                            .fg(style.theme.bg)
                            .bg(style.theme.main),
                    );

                StatefulWidget::render(item_list, list_area, buf, &mut self.list_state);
            }

            // Paragraph::new(Line::from(format!(
            //     "current_index: {}, len: {}",
            //     self.current_index,
            //     self.input.len()
            // )))
            // .render(list, buf);
        }
    }
}
