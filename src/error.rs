use std::fmt;

use serde::{de, ser};

#[derive(Debug)]
pub enum EncodeError {
    Message(String),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for EncodeError {}

impl ser::Error for EncodeError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::Message(message.to_string())
    }
}

impl From<serde_json::Error> for EncodeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Message(error.to_string())
    }
}

#[derive(Debug)]
pub enum DecodeError {
    Message(String),
    TrailingData,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(message) => write!(f, "{}", message),
            Self::TrailingData => write!(f, "trailing data"),
        }
    }
}

impl std::error::Error for DecodeError {}

impl de::Error for DecodeError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::Message(message.to_string())
    }
}

impl From<serde_json::Error> for DecodeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Message(error.to_string())
    }
}
