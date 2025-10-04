use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition},
};

use std::time::SystemTime;

#[derive(Debug)]
pub struct Char {
    diff: u128,
    index: usize,
    typed: char,
    actual: Option<char>,
}

impl Char {
    pub fn new(diff: u128, index: usize, typed: char, actual: Option<char>) -> Self {
        Self {
            diff,
            index,
            typed,
            actual,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.actual.is_none()
    }

    pub fn is_error(&self) -> bool {
        self.actual.is_some()
    }

    pub fn is_char(&self, char: char) -> bool {
        self.actual == Some(char) || (self.actual.is_none() && self.typed == char)
    }
}

pub struct TestStatistics {
    started: Option<SystemTime>,
    ended: Option<SystemTime>,
    last_char_typed: Option<SystemTime>,
    chars: Vec<Char>,
}

impl TestStatistics {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            started: None,
            ended: None,
            last_char_typed: None,
            chars: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.started = None;
        self.ended = None;
        self.last_char_typed = None;
        self.chars = Vec::new();
    }

    pub fn start(&mut self) {
        self.started = Some(SystemTime::now());
    }

    pub fn end(&mut self) {
        self.ended = Some(SystemTime::now());
    }

    fn started(&mut self) -> SystemTime {
        match self.started {
            Some(time) => time,
            None => {
                self.start();
                self.started
                    .expect("Started should always be set after calling sefl.start()")
            }
        }
    }

    fn last_char_typed(&mut self) -> SystemTime {
        match self.last_char_typed {
            Some(time) => time,
            None => self.started(),
        }
    }

    fn ended_or_now(&self) -> SystemTime {
        match self.ended {
            Some(time) => time,
            None => SystemTime::now(),
        }
    }

    pub fn new_char(&mut self, index: usize, typed: char, actual: char) {
        let now = SystemTime::now();
        let diff = match now.duration_since(self.last_char_typed()) {
            Ok(diff) => diff.as_millis(),
            Err(_) => 0,
        };

        if typed == actual {
            self.chars.push(Char::new(diff, index, typed, None));
        } else {
            self.chars.push(Char::new(diff, index, typed, Some(actual)));
        }

        self.last_char_typed = Some(now);
    }

    pub fn wpm(&self) -> usize {
        let cpm = self.cpm();
        cpm / crate::CHARS_PER_WORD as usize
    }

    pub fn cpm(&self) -> usize {
        let started = match self.started {
            Some(time) => time,
            None => return 0,
        };

        let (right_chars, _) =
            self.chars
                .iter()
                .fold((0, Vec::new()), |(count, mut already), c| {
                    if c.is_ok() {
                        let count = if already.contains(&c.index) {
                            count
                        } else {
                            already.push(c.index);
                            count + 1
                        };
                        (count, already)
                    } else {
                        (count, already)
                    }
                });

        let ended = self.ended_or_now();

        let seconds = ended.duration_since(started).unwrap().as_secs_f64();
        let minutes = seconds / 60.0;

        (right_chars as f64 / minutes).round() as usize
    }

    fn wpm_and_error_indexes_for_each_char(chars: &[Char]) -> (Vec<usize>, Vec<usize>) {
        let word_count = 1 + chars.iter().filter(|c| c.is_char(' ')).count();

        let mut errors = Vec::new();
        let mut wpms = vec![(0, 0); word_count];

        let mut chars = chars.iter();
        chars.next();

        let mut current_word_index = 0;

        for char in chars {
            let is_end = char.is_char(' ');

            wpms[current_word_index].1 += 1;
            wpms[current_word_index].0 +=
                (60_000. / char.diff as f32 / crate::CHARS_PER_WORD).round() as usize;
            if char.is_error() {
                errors.push(char.index);
            }

            if is_end {
                current_word_index += 1;
            }
        }

        let wpms = wpms
            .iter()
            .map(|(wpm_sum, count)| if *count == 0 { 0 } else { wpm_sum / count })
            .collect();

        (wpms, errors)
    }

    fn right_wrong_char_count(chars: &[Char]) -> (usize, usize) {
        chars
            .iter()
            .fold((0, 0), |(right, wrong), c| match c.is_ok() {
                true => (right + 1, wrong),
                false => (right, wrong + 1),
            })
    }

    pub fn accuracy(&self) -> f32 {
        let (right_count, wrong_count) = Self::right_wrong_char_count(&self.chars);
        (right_count as f32 / (right_count + wrong_count) as f32) * 100.0
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.started.is_some() {
            Text::raw(format!("{: >3} {: >5.2}%", self.wpm(), self.accuracy())).render(area, buf);
        }
    }

    pub fn render_end(&self, area: Rect, buf: &mut Buffer) {
        // TODO make this work and look good...
        let (wpms, _errors) = Self::wpm_and_error_indexes_for_each_char(&self.chars);

        let mut highest_wpm = usize::MIN;
        let mut lowest_wpm = usize::MAX;

        let mut data = Vec::new();

        for (i, wpm) in wpms.iter().enumerate() {
            if *wpm > highest_wpm {
                highest_wpm = *wpm;
            } else if *wpm < lowest_wpm {
                lowest_wpm = *wpm;
            }
            // data.push((*wpm as f64, i as f64));
            data.push((i as f64, *wpm as f64));
        }

        if lowest_wpm == usize::MAX {
            lowest_wpm = 0;
        }

        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&data),
        ];

        let lh_diff = highest_wpm.saturating_sub(lowest_wpm);

        let chart = Chart::new(datasets)
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().gray())
                    .bounds([0.0, wpms.len() as f64])
                    .labels([
                        "0".bold(),
                        (wpms.len() / 2).to_string().into(),
                        wpms.len().to_string().bold(),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().gray())
                    .bounds([lowest_wpm.saturating_sub(10) as f64, highest_wpm as f64])
                    .labels([
                        lowest_wpm.to_string().bold(),
                        (lowest_wpm + lh_diff / 2).to_string().bold(),
                        highest_wpm.to_string().bold(),
                    ]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        chart.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpm() {
        let mut statistics = TestStatistics::new();
        let chars = ['a', 'b', 'c', 'd', 'f', 'g', 'h', 'i', 'j', 'k'];

        for (i, char) in chars.iter().enumerate() {
            statistics.new_char(i, *char, *char);
        }

        if let Some(t) = statistics
            .ended_or_now()
            .checked_add(std::time::Duration::from_secs(5))
        {
            statistics.ended = Some(t);

            assert_eq!(statistics.cpm(), 120);
        } else {
            panic!("Couldn't add 5 secs to system time");
        }
    }

    #[test]
    fn accuracy() {
        let mut statistics = TestStatistics::new();
        let right_chars = ['a', 'b', 'c', 'd', 'f', 'g', 'h', 'i', 'j', 'k'];
        let right_chars_len = right_chars.len();

        for (i, char) in right_chars.iter().enumerate() {
            statistics.new_char(i, *char, *char);
        }

        let wrong_chars = ['l', 'm', 'n', 'o', 'p'];

        for (i, char) in wrong_chars.iter().enumerate() {
            statistics.new_char(i + right_chars_len, *char, 'a');
        }

        if let Some(t) = statistics
            .ended_or_now()
            .checked_add(std::time::Duration::from_secs(5))
        {
            statistics.ended = Some(t);

            assert_eq!(format!("{:.2}", statistics.accuracy()), "66.67".to_string());
        } else {
            panic!("Couldn't add 5 secs to system time");
        }
    }
}
