//! PE parsing error types.

use std::fmt;

#[derive(Debug)]
pub enum PeParseError {
    UnexpectedEof(&'static str),
    InvalidSignature(&'static str),
    Unsupported(&'static str),
    Invalid(&'static str),
}

impl fmt::Display for PeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeParseError::UnexpectedEof(ctx) => write!(f, "unexpected EOF: {ctx}"),
            PeParseError::InvalidSignature(ctx) => write!(f, "invalid signature: {ctx}"),
            PeParseError::Unsupported(ctx) => write!(f, "unsupported: {ctx}"),
            PeParseError::Invalid(ctx) => write!(f, "invalid: {ctx}"),
        }
    }
}

impl std::error::Error for PeParseError {}
