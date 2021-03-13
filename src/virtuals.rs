use crate::asm;
use crate::ast;
use crate::util::label;
use std::collections::HashMap;

pub type Register = &'static str;

#[derive(Debug, Clone)]
pub enum Inst {
  Li(Register, i32),
  FLi(Register, f32),
  Mv(Register, Register),
  Op1(Register, ast::Op1, Register),
  Op2(Register, ast::Op2, Register, Register),
  Jal(String),
  Ret,
  Save(Register, String),
  Restore(Register, String),
  IfElse(Register, Vec<Inst>, Vec<Inst>),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: String,
  pub content: Vec<Inst>,
}

#[derive(Debug, Clone)]
pub struct Program {
  pub functions: Vec<Function>,
}

impl Inst {
  fn to_asm(self, env: &HashMap<String, i32>, name: &str) -> Vec<asm::Inst> {
    match self {
      Inst::Li(r, s) => vec![asm::Inst::Mv(asm::Operand::Reg(r), asm::Operand::Int(s))],
      Inst::FLi(r, s) => vec![asm::Inst::Mv(asm::Operand::Reg(r), asm::Operand::Float(s))],
      Inst::Mv(r, s) => vec![asm::Inst::Mv(asm::Operand::Reg(r), asm::Operand::Reg(s))],
      Inst::Op1(d, op, s) => {
        let d = asm::Operand::Reg(d);
        let s = asm::Operand::Reg(s);
        match op {
          ast::Op1::Neg => vec![asm::Inst::Neg(d, s)],
          ast::Op1::FNeg => vec![asm::Inst::FNeg(d, s)],
          ast::Op1::Ftoi => vec![asm::Inst::Ftoi(d, s)],
          ast::Op1::Itof => vec![asm::Inst::Itof(d, s)],
        }
      },
      Inst::Op2(d, op, s, t) => {
        let d = asm::Operand::Reg(d);
        let s = asm::Operand::Reg(s);
        let t = asm::Operand::Reg(t);
        match op {
          ast::Op2::Add => vec![asm::Inst::Add(d, s, t)],
          ast::Op2::Sub => vec![asm::Inst::Sub(d, s, t)],
          ast::Op2::Mul => vec![asm::Inst::Mul(d, s, t)],
          ast::Op2::Div => vec![asm::Inst::Div(d, s, t)],
          ast::Op2::FAdd => vec![asm::Inst::FAdd(d, s, t)],
          ast::Op2::FSub => vec![asm::Inst::FSub(d, s, t)],
          ast::Op2::FMul => vec![asm::Inst::FMul(d, s, t)],
          ast::Op2::FDiv => vec![asm::Inst::FDiv(d, s, t)],
          ast::Op2::Eq => vec![asm::Inst::Seq(d, s, t)],
          ast::Op2::Ne => vec![asm::Inst::Sne(d, s, t)],
          ast::Op2::Lt => vec![asm::Inst::Slt(d, s, t)],
          ast::Op2::Le => vec![asm::Inst::Sle(d, s, t)],
          ast::Op2::FEq => vec![asm::Inst::FSeq(d, s, t)],
          ast::Op2::FNe => vec![asm::Inst::FSne(d, s, t)],
          ast::Op2::FLt => vec![asm::Inst::FSlt(d, s, t)],
          ast::Op2::FLe => vec![asm::Inst::FSle(d, s, t)],
        }
      },
      Inst::Jal(s) => vec![asm::Inst::Jal(s)],
      Inst::Ret => vec![asm::Inst::Jmp(label::end(name))],
      Inst::Save(r, x) => vec![asm::Inst::Sw(asm::Operand::Reg(r), asm::Operand::Sp, asm::Operand::Int(*env.get(&x).unwrap()))],
      Inst::Restore(r, x) => vec![asm::Inst::Lw(asm::Operand::Reg(r), asm::Operand::Sp, asm::Operand::Int(*env.get(&x).unwrap()))],
      Inst::IfElse(r, v, w) => {
        let mut res = Vec::new();
        let else_label = label::generate("else");
        let cont_label = label::generate("cont");
        res.push(asm::Inst::Beqz(asm::Operand::Reg(r), else_label.clone()));
        for i in v.into_iter() {
          let mut c = i.to_asm(env, name);
          res.append(&mut c);
        }
        res.push(asm::Inst::Jmp(cont_label.clone()));
        res.push(asm::Inst::Label(else_label));
        for i in w.into_iter() {
          let mut c = i.to_asm(env, name);
          res.append(&mut c);
        }
        res.push(asm::Inst::Label(cont_label));
        res
      }
    }
  }
}

fn flatten(v: Vec<Inst>) -> Vec<Inst> {
  let mut res = Vec::new();
  for i in v.into_iter() {
    match i {
      Inst::IfElse(x, s, t) => {
        res.append(&mut flatten(s));
        res.append(&mut flatten(t));
        res.push(Inst::IfElse(x, Vec::new(), Vec::new()));
      },
      x => res.push(x),
    }
  }
  res
}

impl Function {
  fn to_asm(self) -> Vec<asm::Inst> {
    let mut env: HashMap<String, i32> = HashMap::new();
    let mut ss: i32 = 1;
    for ist in flatten(self.content.clone()).iter() {
      match ist {
        Inst::Save(_, x) => {
          env.insert(x.clone(), ss);
          ss += 1;
        },
        _ => (),
      }
    }

    let mut insts: Vec<asm::Inst> = vec![
      asm::Inst::Label(self.name.clone()),
      asm::Inst::Sub(asm::Operand::Sp, asm::Operand::Sp, asm::Operand::Int(ss)),
      asm::Inst::Sw(asm::Operand::Ra, asm::Operand::Sp, asm::Operand::Int(0)),
    ];

    for ist in self.content.into_iter() {
      let mut conveterd = ist.to_asm(&env, &self.name);
      insts.append(&mut conveterd);
    }

    let mut fin = vec![
      asm::Inst::Label(label::end(&self.name)),
      asm::Inst::Lw(asm::Operand::Ra, asm::Operand::Sp, asm::Operand::Int(0)),
      asm::Inst::Add(asm::Operand::Sp, asm::Operand::Sp, asm::Operand::Int(ss)),
      asm::Inst::Ret,
    ];

    insts.append(&mut fin);
    insts
  }
}

impl Program {
  pub fn to_asm(self) -> asm::Program {
    let mut res: Vec<asm::Inst> = vec![asm::Inst::Jmp("main".to_string())];
    for fun in self.functions.into_iter() {
      let mut v = fun.to_asm();
      res.append(&mut v);
    }
    asm::Program{program: res}
  }
}