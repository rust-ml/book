use std::{io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidMath(String, String, usize), // reason, element, line
    InvalidReference(String),
    InvalidBibliography(String),
    InvalidDvisvgm(String),
    BinaryNotFound(which::Error),
    UnevenNumberDollar,
    Io(io::Error),
}
