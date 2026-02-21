#![feature(decl_macro)]
#![feature(str_from_raw_parts)]
#![feature(stmt_expr_attributes)]
mod reader;
pub use reader::Reader;

mod bytecode;
pub use bytecode::*;

mod decoder;
pub use decoder::*;

mod fmt;
pub use fmt::*;

#[derive(Debug)]
pub enum Error {
    Malformed,
    Exhausted,
    Unimplemented,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Malformed => writeln!(f, "Encountered malformed bytes."),
            Error::Exhausted => writeln!(f, "Reached end of file, expected more bytes."),
            Error::Unimplemented => writeln!(f, "Could not parse: unimplemented."),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
