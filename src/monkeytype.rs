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

    pub fn random_words(&self, count: &WordCount) -> Option<Vec<&String>> {
        self.words.random(count)
    }

    pub fn random_quote(&self, quote_lengths: &Vec<QuoteLength>) -> crate::Result<&Quote> {
        if let Some(quotes) = &self.quotes {
            match quotes.random(quote_lengths) {
                Some(quote) => Ok(quote),
                None => Err(crate::Error::NoQuoteWithLengths(quote_lengths.clone()))
            }
        } else {
            Err(crate::Error::NoQuotesForLanguage(self.language))
        }
    }

    pub fn set_language(&mut self, language: Language) -> crate::Result<()> { 
        self.quotes = Quotes::from_language(&language)?;
        self.words = Words::from_language(&language)?;
        self.language = language;
        Ok(())
    }
}
