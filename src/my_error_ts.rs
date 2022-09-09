use std::{
    fmt::Display,
    error::Error,
    num::ParseIntError,
    io,
};

#[derive(Debug)]
pub enum MyErrorTs {
    UsageError,
    IO(io::Error),
    PIE(ParseIntError),
}

impl Display for MyErrorTs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MyErrorTs {}
