#[macro_use]
extern crate error_chain;
extern crate rustc_version_runtime;
pub extern crate semuru_core as core;

pub use self::errors::{Error, Result};
pub use self::vm::VM;
pub use self::compiler::Compiler;

pub mod errors {
    error_chain!{}
}

mod vergen;
mod vm;
mod compiler;
