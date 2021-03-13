use crate::ast;
use crate::util;

#[derive(Debug, Clone)]
pub enum Expr {
  Int(i32),
  Float(f32),
  Op1(ast::Op1, String),
  Op2(ast::Op2, String, String),
  Var(String),
  Call(String, Vec<String>),
}

#[derive(Debug, Clone)]
pub enum Sent {
  Assign(String, Expr),
  IfElse(String, Vec<Sent>, Vec<Sent>),
  Return(Option<String>),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: String,
  pub args: Vec<String>,
  pub content: Vec<Sent>,
}

#[derive(Debug, Clone)]
pub struct Program {
  pub functions: Vec<Function>,
}

impl ast::Expr {
  fn to_knormal(self, x: String) -> Vec<Sent> {
    match self {
      ast::Expr::Void => vec![],
      ast::Expr::Int(i) => vec![Sent::Assign(x, Expr::Int(i))],
      ast::Expr::Float(f) => vec![Sent::Assign(x, Expr::Float(f))],
      ast::Expr::Op1(op, e) => {
        let (mut v, y) = e.expand();
        v.push(Sent::Assign(x, Expr::Op1(op, y)));
        v
      },
      ast::Expr::Op2(op, e1, e2) => {
        let (mut v1, y1) = e1.expand();
        let (mut v2, y2) = e2.expand();
        v1.append(&mut v2);
        v1.push(Sent::Assign(x, Expr::Op2(op, y1, y2)));
        v1
      },
      ast::Expr::Var(y) => vec![Sent::Assign(x, Expr::Var(y))],
      ast::Expr::Assign(y, e) => {
        let (mut v, z) = e.expand();
        v.push(Sent::Assign(y.clone(), Expr::Var(z)));
        v.push(Sent::Assign(x, Expr::Var(y)));
        v
      },
      ast::Expr::Call(f, v) => {
        let mut res: Vec<Sent> = Vec::new();
        let mut name_list: Vec<String> = Vec::new();
        for e in v.into_iter() {
          let (mut w, y) = e.expand();
          res.append(&mut w);
          name_list.push(y);
        }
        res.push(Sent::Assign(x, Expr::Call(f, name_list)));
        res
      }
    }
  }
  fn expand(self) -> (Vec<Sent>, String) {
    match self {
      Self::Var(x) => (Vec::new(), x),
      _ => {
        let x = util::id::generate();
        let v = self.to_knormal(x.clone());
        (v, x)
      }
    }
  }
}

impl ast::Sent {
  fn to_knormal(self) -> Vec<Sent> {
    match self {
      Self::Void => Vec::new(),
      Self::Expression(e) => match *e {
        ast::Expr::Assign(x, e) => e.to_knormal(x),
        _ => e.to_knormal(util::id::null()),
      },
      Self::Sentences(v) => {
        let mut res = Vec::<Sent>::new();
        for s in v {
          let mut t = s.to_knormal();
          res.append(&mut t);
        }
        res
      },
      Self::Return(e) => {
        match *e {
          ast::Expr::Void => vec![Sent::Return(None)],
          _ => {
            let (mut res, x) = e.expand();
            res.push(Sent::Return(Some(x)));
            res
          }
        }
      }
      Self::IfElse(e, s1, s2) => {
        let (mut v, x) = e.expand();
        v.push(Sent::IfElse(x, s1.to_knormal(), s2.to_knormal()));
        v
      }
      _ => unreachable!(),
    }
  }
}

impl ast::Function {
  fn to_knormal(self) -> Function {
    let args = self.args.into_iter().map(|x| x.1).collect();
    let content = self.content.to_knormal();
    Function {name: self.name, args, content}
  }
}

impl ast::Program {
  pub fn to_knormal(self) -> Program {
    Program {functions: self.functions.into_iter().map(|x| x.to_knormal()).collect()}
  }
}
