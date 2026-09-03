use std::{fmt::{self, Display}};

#[derive(Debug)]
pub enum InputError {
    Parse(std::num::ParseIntError),
    Exception(std::io::Error),
}

impl From<std::io::Error> for InputError {
    fn from(error: std::io::Error) -> Self {
        Self::Exception(error)
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Parse(error)
        
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exception(error) =>
                write!(f, "Erro inesperado: {error}"),
            Self::Parse(error) => 
                write!(f, "Erro ao converter: {error}"),
        }
    }
}