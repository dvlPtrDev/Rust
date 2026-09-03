
use std::{
    fmt::{
        self, 
        Display
    }, 
    write
};

#[derive(Debug)]
pub enum ApplicationError {
    FileExists,
    FileNotExists,
    DirNotExists,
    UserHomeNotFound,
    Exception(String),
}

impl From<std::io::Error> for ApplicationError {
    fn from(error: std::io::Error) -> Self {
        Self::Exception(error.to_string())
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileExists => 
                write!(f, "arquivo já existente!"),
            Self::FileNotExists =>
                write!(f, "Arquivo não encontrado"),
            Self::DirNotExists => 
                write!(f, "Diretório não encontrado"),
            Self::UserHomeNotFound => 
                write!(f, "Diretório do usuário não encontrado"),
            Self::Exception(error) => 
                write!(f, "Um erro inesperado aconteceu: {error}"),
        }
    }

}