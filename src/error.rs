use crate::monkeytype::Language;
use crate::typing::QuoteLength;

use std::fmt;

pub type BoxError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    NoWordsForLanguage(Language),
    NoQuoteWithLengths(Vec<QuoteLength>),
    NoQuotesForLanguage(Language),
    Generic(BoxError),
}

impl Error {
    pub fn as_error(self) -> BoxError {
        match self {
            Self::Generic(err) => err,
            _ => self.as_string().into()
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::NoWordsForLanguage(language) => {
                format!("There are no words for the language: {language}")
            }
            Self::NoQuoteWithLengths(lengths) => {
                format!("There are no quotes with lengths: {lengths:?}")
            }
            Self::NoQuotesForLanguage(language) => {
                format!("There are no quotes for the language: {language}")
            }
            Self::Generic(err) => err.to_string(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

macro_rules! impl_error_for {
    ($type:ty) => {
        impl From<$type> for Error {
            fn from(err: $type) -> Self {
                Self::Generic(Box::new(err))
            }
        }
    };
}

impl_error_for!(serde_json::Error);
impl_error_for!(std::io::Error);
