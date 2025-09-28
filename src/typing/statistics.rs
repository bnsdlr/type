use ratatui::prelude::*;

use std::time::SystemTime;

pub enum Char {
    Error(u128, (usize, (char, char))),
    Ok(u128, (usize, char)),
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
            self.chars.push(Char::Ok(diff, (index, typed)));
        } else {
            self.chars.push(Char::Error(diff, (index, (typed, actual))));
        }

        self.last_char_typed = Some(now);
    }

    pub fn wpm(&self) -> usize {
        let cpm = self.cpm();
        cpm / 5
    }

    pub fn cpm(&self) -> usize {
        let started = match self.started {
            Some(time) => time,
            None => return 0,
        };

        let (right_chars, _) =
            self.chars
                .iter()
                .fold((0, Vec::new()), |(count, mut already), c| match c {
                    Char::Ok(_, (index, _)) => {
                        let count = if already.contains(index) {
                            count
                        } else {
                            already.push(*index);
                            count + 1
                        };
                        (count, already)
                    }
                    Char::Error(..) => (count, already),
                });

        let ended = self.ended_or_now();

        let seconds = ended.duration_since(started).unwrap().as_secs_f64();
        let minutes = seconds / 60.0;

        (right_chars as f64 / minutes).round() as usize
    }

    fn right_wrong_char_count(chars: &[Char]) -> (usize, usize) {
        chars.iter().fold((0, 0), |(right, wrong), c| match c {
            Char::Ok(..) => (right + 1, wrong),
            Char::Error(..) => (right, wrong + 1),
        })
    }

    pub fn accuracy(&self) -> f32 {
        let (right_count, wrong_count) = Self::right_wrong_char_count(&self.chars);
        (right_count as f32 / (right_count + wrong_count) as f32) * 100.0
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.started.is_some() {
            Text::raw(format!("{: >3} {: >5.2}%", self.wpm(), self.accuracy())).render(area, buf);
        } else {
        }
    }

    pub fn render_end(&self, area: Rect, buf: &mut Buffer) {
        Text::raw(format!(
            "wpm: {}, accuracy: {:.2}%",
            self.wpm(),
            self.accuracy()
        ))
        .render(area, buf);
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
