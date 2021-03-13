pub mod id {
  static mut COUNTER: u32 = 0;
  pub fn generate() -> String {
    unsafe {
      COUNTER += 1;
      format!("x.{}", COUNTER)
    }
  }
  pub fn null() -> String {
    ".".to_string()
  }
  pub fn ret_val() -> String {
    ".RET_VAL".to_string()
  }
}

pub mod label {
  static mut COUNTER: u32 = 0;
  pub fn end(s: &str) -> String {
    format!(".{}.end", s)
  }
  pub fn generate(s: &str) -> String {
    unsafe {
      COUNTER += 1;
      format!(".{}.{}", s, COUNTER)
    }
  }
}