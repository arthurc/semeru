use nom::{be_u8, be_u16};

named!(cafebabe, tag!(b"\xCA\xFE\xBA\xBE"));

named!(cp_info<()>,
  do_parse!(
    tag: be_u8 >>
    (
      ()
    )
  )
);

named!(class_file<()>,
  do_parse!(
    cafebabe >>
    minor_version: be_u16 >>
    major_version: be_u16 >>
    constant_pool_count: be_u16 >>
    constant_pool: count!(cp_info, constant_pool_count as usize - 1) >>
    (
      ()
    )
  )
);

#[cfg(test)]
mod tests {
  use super::*;

  mod cafebabe {
    use super::*;

    #[test]
    fn given_a_class_file_header_then_it_should_be_parsable() {
      assert!(cafebabe(&[0xCA, 0xFE, 0xBA, 0xBE]).is_done());
    }
  }

  mod class_file {
    use super::*;

//    #[test]
    fn given_a_class_file_then_it_should_be_parsable() {
      assert!(class_file(&[
        0xCA, 0xFE, 0xBA, 0xBE,
        0x10,
        0x55
      ]).is_done());
    }
  }

}
