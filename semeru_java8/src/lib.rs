extern crate semeru_core;

use semeru_core::{Module, Result};

#[derive(Default)]
pub struct Java8Module;

impl Java8Module {
  pub fn new() -> Self {
    Default::default()
  }
}

impl Module for Java8Module {

  fn build(self) -> Result<()> {
    Ok(())
  }

}
