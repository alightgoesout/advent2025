mod error;
mod input;
mod solution;

pub use error::Error;
pub use solution::Solution;
pub type Result<T> = std::result::Result<T, Error>;
pub(crate) use error::error;

pub fn solutions() -> Vec<Box<dyn Solution>> {
	vec![]
}
