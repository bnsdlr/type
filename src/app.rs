#![allow(dead_code, clippy::new_without_default, clippy::single_match)]

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, BorderType, ListItem, Widget},
};

const TITLE: &str = "Type";

#[derive(PartialEq, Eq)]
enum Tab {
    Main,
    Typing,
    Help,
    Settings,
}

impl Tab {
    fn from_number(num: usize) -> Option<Tab> {
        match num {
            n if n == Tab::Main as usize => Some(Tab::Main),
            n if n == Tab::Typing as usize => Some(Tab::Typing),
            n if n == Tab::Help as usize => Some(Tab::Help),
            n if n == Tab::Settings as usize => Some(Tab::Settings),
            _ => None,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Self::Main => "Main",
            Self::Typing => "Typing",
            Self::Help => "Help",
            Self::Settings => "Settings",
        }
    }

    fn as_list_item(&self, is_current: bool) -> ListItem {
        let item = ListItem::new(self.to_string());

        let fg_color = if is_current {
            Color::Cyan
        } else {
            Color::White
        };

        item.fg(fg_color)
    }
}

pub struct App {
    exit: bool,
    current_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            current_tab: Tab::Typing,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> crate::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> crate::Result<()> {
        match key_event.kind {
            KeyEventKind::Press => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => self.exit = true,
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.exit = true,
                (KeyCode::Char('1'), KeyModifiers::NONE) => self.set_tab_from_num(0),
                (KeyCode::Char('2'), KeyModifiers::NONE) => self.set_tab_from_num(1),
                (KeyCode::Char('3'), KeyModifiers::NONE) => self.set_tab_from_num(2),
                (KeyCode::Char('4'), KeyModifiers::NONE) => self.set_tab_from_num(3),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn set_tab_from_num(&mut self, num: usize) {
        if let Some(tab) = Tab::from_number(num) {
            self.current_tab = tab;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        let [heading, body] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .horizontal_margin(1)
            .areas(area);

        let heading = Layout::horizontal([
            Constraint::Length((TITLE.len() + 5) as u16),
            Constraint::Fill(1),
        ])
        .vertical_margin(1)
        .split(heading);

        Text::from(TITLE)
            .style(Style::default().fg(Color::Cyan).bold())
            .render(heading[0], buf);

        let tabs = [Tab::Typing, Tab::Help, Tab::Settings];

        let constraints = tabs
            .iter()
            .map(|tab| Constraint::Length(tab.to_string().len() as u16 + 4));

        let tab_layouts = Layout::horizontal(constraints)
            .flex(layout::Flex::End)
            .areas::<3>(heading[1])
            .to_vec();

        for (tab, layout) in tabs.iter().zip(tab_layouts) {
            let is_current = *tab == self.current_tab;
            let fg = if is_current {
                Color::Cyan
            } else {
                Color::White
            };
            let mut text = Text::from(tab.to_string()).fg(fg);
            if is_current {
                text = text.bold();
            }
            text.render(layout, buf);
        }

        match self.current_tab {
            Tab::Main => {
                let [body] = Layout::horizontal([Constraint::Percentage(100)])
                    .flex(layout::Flex::Center)
                    .areas(body);

                Block::bordered()
                    .border_type(BorderType::Plain)
                    .render(body, buf);

                // Line::from("Welcome to Type!")
                //     .style(Style::default())
                //     .render(center, buf);
            }
            Tab::Typing => {
                let [_, body, _] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(100),
                    Constraint::Fill(1),
                ])
                .areas(body);

                let [top, body, bottom] = Layout::vertical([
                    Constraint::Min(8),
                    Constraint::Percentage(70),
                    Constraint::Min(5),
                ])
                .areas(body);

                // Top
                {
                    let [options, live_stats] =
                        Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
                            .flex(layout::Flex::SpaceBetween)
                            .areas(top);

                    Block::bordered()
                        .border_style(Color::Green)
                        .render(options, buf);

                    Block::bordered()
                        .border_style(Color::Yellow)
                        .render(live_stats, buf);
                }

                // Body
                {
                    Block::bordered()
                        .render(body, buf);
                }

                // Bottom
                {
                    Block::bordered()
                        .border_style(Color::Green)
                        .render(bottom, buf);
                }
            }
            _ => (),
        }
    }
}
