use std::fmt::Display;
use std::num::ParseIntError;
use winnow::error::ParseError;

#[derive(Debug, Clone)]
pub struct Error(pub String);

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self(format!("IO error: {value}"))
	}
}

impl From<ParseIntError> for Error {
	fn from(value: ParseIntError) -> Self {
		Self(format!("Error while parsing integer: {value}"))
	}
}

impl<I, E> From<ParseError<I, E>> for Error
where
	I: Display,
	E: Display,
{
	fn from(value: ParseError<I, E>) -> Self {
		Self(format!(
			"Parsing error at offset {}: {}",
			value.offset(),
			value.input()
		))
	}
}

macro_rules! error {
	($($x:expr),+$(,)?) => { $crate::Error(format!($($x),+)) };
}

pub(crate) use error;
