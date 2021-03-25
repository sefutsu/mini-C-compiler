use crate::ast::*;
use crate::util::id;
use std::collections::HashMap;

impl Op2 {
  fn get_type(&self) -> (Type, Type) {
    match self {
      Self::FAdd | Self::FSub | Self::FMul | Self::FDiv => (Type::Float, Type::Float),
      Self::FEq | Self::FNe | Self::FLt | Self::FLe => (Type::Float, Type::Int),
      _ => (Type::Int, Type::Int)
    }
  }
  fn to_float(self) -> Self {
    match self {
      Self::Add => Self::FAdd,
      Self::Sub => Self::FSub,
      Self::Mul => Self::FMul,
      Self::Div => Self::FDiv,
      Self::Eq => Self::FEq,
      Self::Ne => Self::FNe,
      Self::Lt => Self::FLt,
      Self::Le => Self::FLe,
      _ => self
    }
  }
  fn to_int(self) -> Self {
    match self {
      Self::FAdd => Self::Add,
      Self::FSub => Self::Sub,
      Self::FMul => Self::Mul,
      Self::FDiv => Self::Div,
      Self::FEq => Self::Eq,
      Self::FNe => Self::Ne,
      Self::FLt => Self::Lt,
      Self::FLe => Self::Le,
      _ => self
    }
  }
}

impl Expr {
  fn typing(self, env: &HashMap<String, Type>, funenv: &HashMap<String, FunType>) -> Result<(Self, Type), String> {
    match self {
      Self::Void => Ok((self, Type::Void)),
      Self::Int(_) => Ok((self, Type::Int)),
      Self::Float(_) => Ok((self, Type::Float)),
      Self::Var(ref x) => match env.get(x) {
        None => Err(format!("Undeclared variable: {}", x)),
        Some(t) => Ok((self, t.clone())),
      }
      Self::Op1(op, e) => {
        let (e, t) = e.typing(env, funenv)?;
        match t {
          Type::Void => Err(format!("void type")),
          Type::Int => match op {
            Op1::Neg | Op1::FNeg => 
              Ok((Self::Op1(Op1::Neg, Box::new(e)), Type::Int)),
            Op1::Ftoi => Ok((e, Type::Int)),
            Op1::Itof => Ok((Self::Op1(Op1::Itof, Box::new(e)), Type::Float)),
          },
          Type::Float => match op {
            Op1::Neg | Op1::FNeg => 
              Ok((Self::Op1(Op1::FNeg, Box::new(e)), Type::Float)),
            Op1::Ftoi => Ok((Self::Op1(Op1::Ftoi, Box::new(e)), Type::Int)),
            Op1::Itof => Ok((e, Type::Float)),
          },
        }
      },
      Self::Op2(op, e1, e2) => {
        let (e1, t1) = e1.typing(env, funenv)?;
        let (e2, t2) = e2.typing(env, funenv)?;
        match (&t1, &t2) {
          (Type::Int, Type::Int) => {
            let new_op = op.to_int();
            let (_, res_type) = new_op.get_type();
            Ok((Self::Op2(new_op, Box::new(e1), Box::new(e2)), res_type))
          },
          _ => {
            let new_op = op.to_float();
            let (_, res_type) = new_op.get_type();
            Ok((Self::Op2(
              new_op, 
              Box::new(e1.cast_to_float(&t1)?), 
              Box::new(e2.cast_to_float(&t2)?)
            ), res_type))
          },
        }
      },
      Self::Assign(x, e) => {
        let (_, t1) = Self::Var(x.clone()).typing(env, funenv)?;
        let (e, t2) = e.typing(env, funenv)?;
        Ok((Self::Assign(x, Box::new(e.cast_to_type(&t1, &t2)?)), t1))
      },
      Self::Call(f, v) => {
        match funenv.get(&f) {
          None => Err(format!("Undeclared function: {}", f)),
          Some(t) => {
            if t.args.len() != v.len() {
              return Err(format!("Wrong number of arguments when calling function {}", f))
            }
            let mut res: Vec<Self> = Vec::new();
            for (a, b) in t.args.iter().zip(v) {
              let (b, bt) = b.typing(env, funenv)?;
              res.push(b.cast_to_type(a, &bt)?);
            }
            Ok((Self::Call(f, res), t.ret.clone()))
          }
        }
      }
    }
  }
  fn cast_to_float(self, t: &Type) -> Result<Self, String> {
    match t {
      Type::Void => Err(format!("Type Error")),
      Type::Int => Ok(Self::Op1(Op1::Itof, Box::new(self))),
      _ => Ok(self),
    }
  }
  fn cast_to_int(self, t: &Type) -> Result<Self, String> {
    match t {
      Type::Void => Err(format!("Type Error")),
      Type::Float => Ok(Self::Op1(Op1::Ftoi, Box::new(self))),
      _ => Ok(self),
    }
  }
  fn cast_to_type(self, dt: &Type, st: &Type) -> Result<Self, String> {
    match dt {
      Type::Int => self.cast_to_int(st),
      Type::Float => self.cast_to_float(st),
      Type::Void => {
        match st {
          Type::Void => Ok(self),
          _ => Err(format!("Type Error"))
        }
      }
    }
  }
}

impl Sent {
  fn typing(self, env: &mut HashMap<String, Type>, funenv: &HashMap<String, FunType>) -> Result<Self, String> {
    match self {
      Self::Void => Ok(self),
      Self::Decl(t, x) => {
        env.insert(x, t);
        Ok(Self::Void)
      },
      Self::Expression(e) => {
        let (res, _) = e.typing(env, funenv)?;
        Ok(Self::Expression(Box::new(res)))
      },
      Self::DeclAssign(t, x, e) => {
        env.insert(x.clone(), t);
        let (res, _) = Expr::Assign(x, e).typing(env, funenv)?;
        Ok(Self::Expression(Box::new(res)))
      }
      Self::Sentences(v) => {
        let mut res = Vec::new();
        for s in v {
          let t = s.typing(env, funenv)?;
          res.push(t);
        }
        Ok(Self::Sentences(res))
      },
      Self::IfElse(e, s1, s2) => {
        let (e, t) = e.typing(env, funenv)?;
        let e = e.cast_to_int(&t)?;
        let s1 = s1.typing(&mut env.clone(), funenv)?;
        let s2 = s2.typing(&mut env.clone(), funenv)?;
        Ok(Self::IfElse(Box::new(e), Box::new(s1), Box::new(s2)))
      },
      Self::Return(e) => {
        let (e, t) = e.typing(env, funenv)?;
        let res = e.cast_to_type(&env.get(&id::ret_val()).unwrap(), &t)?;
        Ok(Self::Return(Box::new(res)))
      },
    }
  }
}

impl Function {
  fn typing(self, funenv: &HashMap<String, FunType>) -> Result<Self, String> {
    let mut env: HashMap<String, Type> = HashMap::new();
    env.insert(id::ret_val(), self.ret_type.clone());
    for (t, x) in self.args.iter() {
      env.insert(x.clone(), t.clone());
    }
    let res = self.content.typing(&mut env, funenv)?;
    Ok(Self {ret_type: self.ret_type, name: self.name, args: self.args, content: res})
  }
}

impl Program {
  pub fn typing(self) -> Result<Self, String> {
    let mut funenv: HashMap<String, FunType> = HashMap::new();
    let mut res = Self {functions: Vec::new()};
    for fun in self.functions.into_iter() {
      let t: FunType = FunType {
        ret: fun.ret_type.clone(),
        args: fun.args.clone().into_iter().map(|x| x.0).collect()
      };
      funenv.insert(fun.name.clone(), t);
      match fun.content {
        Sent::Void => (),
        _ => {
          let typed = fun.typing(&funenv)?;
          res.functions.push(typed);
        },
      }
    }
    Ok(res)
  }
}
