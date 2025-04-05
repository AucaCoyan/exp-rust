use std::{fmt::Display, string::FromUtf16Error};

#[derive(Debug)]
pub enum Base64Error {
    InvalidCharacter,
    Utf8Error(FromUtf16Error),
}

impl Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Base64Error::InvalidCharacter => write!(f, "Invalid character in input"),
            Base64Error::Utf8Error(ref e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Base64;

const BASE64_CHARSET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
impl Base64 {
    pub fn new() -> Self {
        Base64 {}
    }

    pub fn encode(&mut self, input: &str) -> Result<String, Base64Error> {
        Ok(String::new())
    }
    pub fn decode(&mut self, input: &str) -> Result<String, Base64Error> {
        Ok(String::new())
    }
}

impl Default for Base64 {
    fn default() -> Self {
        Self::new()
    }
}
