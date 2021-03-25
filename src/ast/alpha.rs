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
}

impl Program {
  pub fn alpha(self) -> Self {
    Self {functions: self.functions.into_iter().map(|x| x.alpha()).collect()}
  }
}