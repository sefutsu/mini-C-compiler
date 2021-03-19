use crate::knormal;
use std::collections::HashSet;

pub fn free_variable(prog: &[knormal::Sent]) -> HashSet<String> {
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
}

#[test]
fn test_free_variable() {
  assert_eq!(
    knormal::Sent::Assign("x".to_string(), knormal::Expr::Op2(crate::ast::Op2::Add, "y".to_string(), "z".to_string())).free_variable(),
    (["y", "z"].iter().map(|x| x.to_string()).collect::<HashSet<_>>(),
    ["x"].iter().map(|x| x.to_string()).collect::<HashSet<_>>())
  );
}