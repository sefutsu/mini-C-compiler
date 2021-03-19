use crate::knormal;
use std::collections::HashSet;

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

fn free_variable(prog: &[knormal::Sent]) -> HashSet<String> {
  let mut free: HashSet<String> = HashSet::new();
  let mut rem: HashSet<String> = HashSet::new();
  for sent in prog {
    let (f, r) = sent.free_variable();
    // free += f - rem
    free = free.union(
      &f.difference(&rem).map(|x| x.to_string()).collect()
    ).map(|x| x.to_string()).collect();
    // rem += r
    rem = rem.union(&r).map(|x| x.to_string()).collect();
  }
  free
}

impl knormal::Expr {
  fn free_variable(&self) -> HashSet<String> {
    match self {
      Self::Op1(_, x) => [x].iter().map(|x| x.to_string()).collect(),
      Self::Op2(_, x, y) => [x, y].iter().map(|x| x.to_string()).collect(),
      Self::Var(x) => [x].iter().map(|x| x.to_string()).collect(),
      Self::Int(_) | Self::Float(_) => HashSet::<String>::new(),
      Self::Call(_, args) => args.iter().map(|x| x.to_string()).collect(),
    }
  }
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
  fn free_variable(&self) -> (HashSet<String>, HashSet<String>) {
    match self {
      Self::Assign(x, e) => (e.free_variable(), [x].iter().map(|x| x.to_string()).collect()),
      Self::IfElse(x, s, t) => {
        (
          [x].iter().map(|x| x.to_string()).collect::<HashSet<String>>()
          .union(&free_variable(s)).map(|x| x.to_string()).collect::<HashSet<String>>()
          .union(&free_variable(t)).map(|x| x.to_string()).collect::<HashSet<String>>()
        , HashSet::new()
        )
      },
      Self::Return(Some(x)) => ([x].iter().map(|x| x.to_string()).collect(), HashSet::new()),
      _ => (HashSet::new(), HashSet::new()),
    }
  }
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

#[test]
fn test_free_variable() {
  assert_eq!(
    knormal::Sent::Assign("x".to_string(), knormal::Expr::Op2(crate::ast::Op2::Add, "y".to_string(), "z".to_string())).free_variable(),
    (["y", "z"].iter().map(|x| x.to_string()).collect::<HashSet<_>>(),
    ["x"].iter().map(|x| x.to_string()).collect::<HashSet<_>>())
  );
}