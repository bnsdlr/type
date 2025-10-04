#![allow(dead_code, clippy::new_without_default, clippy::single_match)]

mod quick_menu;
mod tab;

use quick_menu::QuickMenu;
use tab::Tab;

use crate::Config;
use crate::monkeytype::{Language, QuoteLanguage};
use crate::typing::{Mode, QuoteLength, Seconds, TestState, WordCount};
use crate::user::Stats;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, BorderType, Widget},
};

const TITLE: &str = "Type";

pub struct App {
    exit: bool,
    current_tab: Tab,
    test_state: TestState,
    config: Config,
    stats: Stats,
    quick_menu: QuickMenu,
}

impl App {
    pub fn new() -> crate::Result<Self> {
        Ok(App {
            exit: false,
            current_tab: Tab::Typing,
            // test_state: TestState::new()?.mode(Mode::Quote(vec![QuoteLength::Short])),
            test_state: TestState::new()?.mode(Mode::Words(WordCount::W10)),
            config: Config::load()?,
            stats: Stats::load(),
            quick_menu: QuickMenu::new(),
        })
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
        if self.quick_menu.is_visible() {
            self.quick_menu.handle_key_event(key_event)?;
        } else {
            match self.current_tab {
                Tab::Typing => self.test_state.handle_key_event(key_event)?,
                _ => (),
            }
        }

        match key_event.kind {
            KeyEventKind::Press => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.exit = true,
                (KeyCode::Char('1'), KeyModifiers::CONTROL) => self.set_tab_from_num(0),
                (KeyCode::Char('2'), KeyModifiers::CONTROL) => self.set_tab_from_num(1),
                (KeyCode::Char('3'), KeyModifiers::CONTROL) => self.set_tab_from_num(2),
                (KeyCode::Char('?'), KeyModifiers::CONTROL) => self.set_tab_from_num(3),
                (KeyCode::Esc, KeyModifiers::NONE) => self.quick_menu.toggle(),
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

        self.quick_menu.render(&self.config.theme, area, buf);

        let heading = Layout::horizontal([
            Constraint::Length((TITLE.len() + 5) as u16),
            Constraint::Fill(1),
        ])
        .vertical_margin(1)
        .split(heading);

        Text::from(TITLE)
            .style(Style::default().fg(self.config.theme.text).bold())
            .render(heading[0], buf);

        {
            let tabs = [Tab::Typing, Tab::Settings];

            let constraints = tabs
                .iter()
                .map(|tab| Constraint::Length(tab.to_string().len() as u16 + 4));

            let tab_layouts = Layout::horizontal(constraints)
                .flex(layout::Flex::End)
                .areas::<2>(heading[1])
                .to_vec();

            for (tab, layout) in tabs.iter().zip(tab_layouts) {
                tab.as_text_element(&self.config.theme, &self.current_tab)
                    .render(layout, buf);
            }
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

                let [top, _margin, body, bottom] = Layout::vertical([
                    Constraint::Min(8),
                    Constraint::Length(1),
                    Constraint::Percentage(70),
                    Constraint::Min(5),
                ])
                .areas(body);

                // Top
                {
                    self.test_state.render_options(&self.config.theme, top, buf);
                }

                // Body
                {
                    self.test_state.render(&self.config.theme, body, buf);
                }

                // Bottom
                {}
            }
            _ => (),
        }
    }
}
