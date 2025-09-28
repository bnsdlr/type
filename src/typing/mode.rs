use serde::{Deserialize, Serialize, de};

use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum QuoteLength {
    All = 0,
    Short = 100,
    Medium = 300,
    Long = 600,
    Thicc = 9999,
}

impl QuoteLength {
    pub fn deserialize_json<'de, D>(deserializer: D) -> Result<QuoteLength, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: usize = de::Deserialize::deserialize(deserializer)?;
        Ok(QuoteLength::from(s))
    }
}

impl From<usize> for QuoteLength {
    fn from(length: usize) -> Self {
        if length <= Self::Short as usize {
            Self::Short
        } else if length <= Self::Medium as usize {
            Self::Medium
        } else if length <= Self::Long as usize {
            Self::Long
        } else {
            Self::Thicc
        }
    }
}

impl fmt::Display for QuoteLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Self::All => "all",
            Self::Short => "short",
            Self::Medium => "medium",
            Self::Long => "long",
            Self::Thicc => "thicc",
        };

        write!(f, "{string}")
    }
}

pub enum Seconds {
    S15,
    S30,
    S60,
    S120,
    Custom(usize),
}

impl Seconds {
    pub fn as_usize(&self) -> usize {
        match self {
            Self::S15 => 15,
            Self::S30 => 30,
            Self::S60 => 60,
            Self::S120 => 120,
            Self::Custom(secs) => *secs,
        }
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}

pub enum WordCount {
    W10,
    W25,
    W50,
    W100,
    Custom(usize),
}

impl WordCount {
    pub fn as_usize(&self) -> usize {
        match self {
            Self::W10 => 10,
            Self::W25 => 25,
            Self::W50 => 50,
            Self::W100 => 100,
            Self::Custom(words) => *words,
        }
    }
}

impl fmt::Display for WordCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}

pub enum Mode {
    Time(Seconds),
    Words(WordCount),
    Quote(Vec<QuoteLength>),
}

impl Default for Mode {
    fn default() -> Self {
        Self::Time(Seconds::S60)
    }
}
