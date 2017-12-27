#[macro_use]
extern crate error_chain;

pub use self::module::{Module};
pub use self::errors::{Result, Error};

pub mod errors {
  error_chain! {}
}

pub mod module;
