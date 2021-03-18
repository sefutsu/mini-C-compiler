use crate::knormal;

// 変数が生きているかどうか判定
pub fn is_alive(x: &str, program: &[knormal::Sent]) -> bool {
  for sent in program.iter() {
    if sent.is_alive(x) {
      return true;
    }
  }
  false
}

impl knormal::Expr {
  pub fn is_alive(&self, x: &str) -> bool {
    match self {
      Self::Op1(_, y) => (*x == *y),
      Self::Op2(_, y, z) => (*x == *y) || (*x == *z),
      Self::Var(y) => (*x == *y),
      Self::Call(_, args) => {
        for arg in args {
          if *arg == *x {
            return true;
          }
        }
        false
      },
      _ => false,
    }
  }
}

impl knormal::Sent {
  fn is_alive(&self, x: &str) -> bool {
    match self {
      Self::Assign(_, e) => e.is_alive(x),
      Self::IfElse(y, s, t) => 
        (*x == *y) || is_alive(x, s) || is_alive(x, t),
      Self::Return(Some(y)) => (*x == *y),
      _ => false,
    }
  }
}