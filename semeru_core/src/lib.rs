#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

pub use self::module::{Module};
pub use self::errors::{Result, Error};

pub mod errors {
  error_chain! {}
}

pub mod module;
