use errors::Result;
use vergen;

pub struct VM {}

impl VM {
    pub fn build() -> Result<Builder> {
        Builder::new()
    }

    pub fn run(&mut self) {
        use rustc_version_runtime;

        println!("Initializing Semeru...");
        println!("Version: {}", vergen::semver());
        println!("Platform: {}", vergen::target());
        println!("Semeru git commit: {}", vergen::sha());
        let rustc_meta = rustc_version_runtime::version_meta();
        println!(
            "Rustc version: {} {:?}",
            rustc_meta.semver, rustc_meta.channel
        );
    }
}

pub struct Builder {}

impl Builder {
    pub fn new() -> Result<Self> {
        Ok(Builder {})
    }

    pub fn build(&self) -> Result<VM> {
        Ok(VM {})
    }
}
