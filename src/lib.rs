#[macro_use]
extern crate error_chain;
extern crate rustc_version_runtime;
pub extern crate semeru_core as core;

#[cfg(feature = "java8")]
extern crate semeru_java8;

pub mod java8 {
    #[cfg(feature = "java8")]
    pub use semeru_java8::Java8Module as Module;

    #[cfg(not(feature = "java8"))]
    pub type Module = ::core::module::NullModule;
}

pub use self::errors::{Error, Result};
pub use self::vm::VM;
pub use self::compiler::Compiler;

pub mod errors {
    error_chain!{}
}

mod vergen;
mod vm;
mod compiler;
