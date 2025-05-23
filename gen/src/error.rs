use std::fmt::Display;

pub enum Error {
    Gen(GenError)
}

pub struct GenError {
    msg: String
}

impl GenError {
    pub fn error<T>(msg: T) -> Error
    where T: Into<String> {
        Error::Gen(Self { msg: msg.into() })
    }
}
