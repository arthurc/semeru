use errors::Result;

pub trait Module {

  fn build(
    self
  ) -> Result<()>;

}
