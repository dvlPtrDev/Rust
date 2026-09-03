use std::fmt::{self, Display};

use crate::presentation::errors::InputError;
use crate::application::errors::ApplicationError;

#[derive(Debug)]
pub enum Errors {
    InputException(InputError),
    ApplicationException(ApplicationError),
}

impl From<InputError> for Errors {
    fn from(error: InputError) -> Self {
        Self::InputException(error)
    }
}

impl From<ApplicationError> for Errors {
    fn from(error: ApplicationError) -> Self {
        Self::ApplicationException(error)
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputException(error) => write!(f, "{error}"),
            Self::ApplicationException(error) => write!(f, "{error}"),
        }
    }
}