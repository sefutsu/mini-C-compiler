use crate::ast::*;
use crate::util::id;
use std::collections::{HashSet, HashMap};

fn find(env: &HashMap<String, String>, x: String) -> String {
  match env.get(&x) {
    None => x,
    Some(y) => y.clone(),
  }
}

impl Expr {
  fn alpha(self, env: &HashMap<String, String>) -> Self {
    match self {
      Self::Var(x) => Self::Var(find(&env, x)),
      Self::Op1(op, e) => Self::Op1(op, Box::new(e.alpha(env))),
      Self::Op2(op, e1, e2) => Self::Op2(op, Box::new(e1.alpha(env)), Box::new(e2.alpha(env))),
      Self::Assign(x, e) => {
        Self::Assign(find(&env, x), Box::new(e.alpha(env)))
      },
      Self::Call(f, v) => Self::Call(f, v.into_iter().map(|z| z.alpha(env)).collect()),
      _ => self,
    }
  }
}

impl Sent {
  fn alpha(self, env: &mut HashMap<String, String>, defined: &mut HashSet<String>) -> Self {
    match self {
      Self::Expression(e) => Self::Expression(Box::new(e.alpha(env))),
      Self::Decl(t, x) => {
        let x = if defined.contains(&x) {
          let new_id = id::generate();
          env.insert(x, new_id.clone());
          new_id
        } else {
          x
        };
        defined.insert(x.clone());
        Self::Decl(t, x)
      }
      Self::DeclAssign(t, x, e) => {
        let e = e.alpha(env);
        let x = if defined.contains(&x) {
          let new_id = id::generate();
          env.insert(x, new_id.clone());
          new_id
        } else {
          x
        };
        defined.insert(x.clone());
        Self::DeclAssign(t, x, Box::new(e))
      },
      Self::Sentences(v) => {
        let mut res = Vec::new();
        let mut new_env = env.clone();
        let mut new_def = defined.clone();
        for s in v.into_iter() {
          res.push(s.alpha(&mut new_env, &mut new_def));
        }
        Self::Sentences(res)
      },
      Self::IfElse(e, s, t) => Self::IfElse(Box::new(e.alpha(env)), Box::new(s.alpha(env, defined)), Box::new(t.alpha(env, defined))),
      Self::Return(e) => Self::Return(Box::new(e.alpha(env))),
      _ => self,
    }
  }
  fn conflict_check(&self, defined: &mut HashSet<String>) -> Option<String> {
    match self {
      Self::Decl(_, x) | Self::DeclAssign(_, x, _) => {
        let res = defined.get(x).map(|x| x.to_string());
        defined.insert(x.clone());
        res
      },
      Self::Sentences(v) => {
        let mut defined = HashSet::new();
        for s in v.iter() {
          if let Some(x) = s.conflict_check(&mut defined) {
            return Some(x)
          }
        }
        None
      },
      _ => None,
    }
  }
}

impl Function {
  fn alpha(self) -> Self {
    let mut defined = HashSet::new();
    for (_, x) in self.args.iter() {
      defined.insert(x.clone());
    }
    let content = self.content.alpha(&mut HashMap::new(), &mut defined);
    Self {ret_type: self.ret_type, name: self.name, args: self.args, content: content}
  }
  fn conflict_check(&self) -> Option<String> {
    let mut defined: HashSet<String> = HashSet::new();
    for (_, a) in self.args.iter() {
      if defined.contains(a) {
        return Some(a.clone())
      }
      defined.insert(a.clone());
    }
    self.content.conflict_check(&mut HashSet::new())
  }
}

impl Program {
  pub fn alpha(self) -> Result<Self, String> {
    let mut res = Vec::new();
    for fun in self.functions.into_iter() {
      if let Some(x) = fun.conflict_check() {
        return Err(format!("conflicting declaration: {}", x))
      }
      res.push(fun.alpha());
    }
    Ok(Self {functions: res})
  }
}