use crate::knormal;

#[derive(Debug)]
enum Alive {
  Alive,
  NotUse,
  Dead,
}
impl std::ops::BitOr<Self> for Alive {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    match (self, rhs) {
      (Alive::Alive, _) | (_, Alive::Alive) => Alive::Alive,
      (Alive::Dead, _) | (_, Alive::Dead) => Alive::NotUse,
      _ => Alive::NotUse,
    }
  }
}
#[allow(dead_code)]
impl Alive {
  fn to_bool(&self) -> bool {
    match self {
      Self::Alive => true,
      _ => false,
    }
  }
  fn from(b: bool) -> Self {
    match b {
      true => Self::Alive,
      false => Self::NotUse,
    }
  }
}

// 変数が生きているかどうか判定
pub fn is_alive(x: &str, program: &[knormal::Sent]) -> bool {
  for sent in program.iter() {
    match sent.is_alive(x) {
      Alive::Alive => return true,
      Alive::Dead => return false,
      _ => (),
    }
  }
  false
}

impl knormal::Expr {
  fn is_alive(&self, x: &str) -> bool {
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
  fn is_alive(&self, x: &str) -> Alive {
    match self {
      Self::Assign(y, e) => {
        if *x == *y {
          Alive::Dead
        } else {
          Alive::from(e.is_alive(x))
        }
      },
      Self::IfElse(y, s, t) => 
        Alive::from((*x == *y) || is_alive(x, s) || is_alive(x, t)),
      Self::Return(Some(y)) => Alive::from(*x == *y),
      _ => Alive::NotUse,
    }
  }
}