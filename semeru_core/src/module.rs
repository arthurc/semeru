use errors::Result;

pub trait Module {

  fn build(
    self
  ) -> Result<()>;

}

#[derive(Default)]
pub struct NullModule;

impl NullModule {

  pub fn new() -> Self {
    Default::default()
  }

}

impl Module for NullModule {

  fn build(self) -> Result<()> {
    Ok(())
  }

}
