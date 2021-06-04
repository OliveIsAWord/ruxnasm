use crate::{scanner, tokenizer};

#[derive(Clone)]
pub enum Warning {
    Scanner(scanner::Warning),
    Tokenizer(tokenizer::Warning),
}

impl From<scanner::Warning> for Warning {
    fn from(warning: scanner::Warning) -> Self {
        Self::Scanner(warning)
    }
}

impl From<tokenizer::Warning> for Warning {
    fn from(warning: tokenizer::Warning) -> Self {
        Self::Tokenizer(warning)
    }
}

#[derive(Clone)]
pub enum Error {
    Scanner(scanner::Error),
    Tokenizer(tokenizer::Error),
}

impl From<scanner::Error> for Error {
    fn from(error: scanner::Error) -> Self {
        Self::Scanner(error)
    }
}

impl From<tokenizer::Error> for Error {
    fn from(error: tokenizer::Error) -> Self {
        Self::Tokenizer(error)
    }
}