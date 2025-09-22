pub mod quote;
pub mod words;

pub use quote::{Quote, QuoteLanguage, Quotes};
pub use words::{Language, Words};

use crate::typing::{WordCount, QuoteLength};

pub struct MonkeyType {
    pub language: Language,
    quotes: Option<Quotes>,
    words: Words,
}

impl MonkeyType {
    pub fn new(language: Language) -> crate::Result<Self> {
        Ok(Self {
            quotes: Quotes::from_language(&language)?,
            words: Words::from_language(&language)?,
            language,
        })
    }

    pub fn random_words(&self, count: WordCount) -> Vec<&String> {
        self.words.random(count)
    }

    pub fn random_quote(&self, quote_lengths: Vec<QuoteLength>) -> Option<&Quote> {
        if let Some(quotes) = &self.quotes {
            quotes.random(quote_lengths)
        } else {
            None
        }
    }
}
