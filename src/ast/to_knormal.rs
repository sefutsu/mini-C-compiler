use crate::ast;
use crate::knormal::*;
use crate::util;

impl ast::Expr {
  fn to_knormal(self, x: String) -> Vec<Stat> {
    match self {
      ast::Expr::Void => vec![],
      ast::Expr::Int(i) => vec![Stat::Assign(x, Expr::Int(i))],
      ast::Expr::Float(f) => vec![Stat::Assign(x, Expr::Float(f))],
      ast::Expr::Op1(op, e) => {
        let (mut v, y) = e.expand();
        v.push(Stat::Assign(x, Expr::Op1(op, y)));
        v
      },
      ast::Expr::Op2(op, e1, e2) => {
        let (mut v1, y1) = e1.expand();
        let (mut v2, y2) = e2.expand();
        v1.append(&mut v2);
        v1.push(Stat::Assign(x, Expr::Op2(op, y1, y2)));
        v1
      },
      ast::Expr::Var(y) => vec![Stat::Assign(x, Expr::Var(y))],
      ast::Expr::Assign(y, e) => {
        let (mut v, z) = e.expand();
        v.push(Stat::Assign(y.clone(), Expr::Var(z)));
        v.push(Stat::Assign(x, Expr::Var(y)));
        v
      },
      ast::Expr::Call(f, v) => {
        let mut res: Vec<Stat> = Vec::new();
        let mut name_list: Vec<String> = Vec::new();
        for e in v.into_iter() {
          let (mut w, y) = e.expand();
          res.append(&mut w);
          name_list.push(y);
        }
        res.push(Stat::Assign(x, Expr::Call(f, name_list)));
        res
      }
    }
  }
  fn expand(self) -> (Vec<Stat>, String) {
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

impl ast::Stat {
  fn to_knormal(self) -> Vec<Stat> {
    match self {
      Self::Void => Vec::new(),
      Self::Expression(e) => match *e {
        ast::Expr::Assign(x, e) => e.to_knormal(x),
        _ => e.to_knormal(util::id::null()),
      },
      Self::Statements(v) => {
        let mut res = Vec::<Stat>::new();
        for s in v {
          let mut t = s.to_knormal();
          res.append(&mut t);
        }
        res
      },
      Self::Return(e) => {
        match *e {
          ast::Expr::Void => vec![Stat::Return(None)],
          _ => {
            let (mut res, x) = e.expand();
            res.push(Stat::Return(Some(x)));
            res
          }
        }
      }
      Self::IfElse(e, s1, s2) => {
        let (mut v, x) = e.expand();
        v.push(Stat::IfElse(x, s1.to_knormal(), s2.to_knormal()));
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
