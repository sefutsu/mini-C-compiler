use crate::knormal;
use crate::virtuals::*;
use crate::alive;
use std::collections::HashMap;

fn all_regs() -> Vec<Register> {
  vec![
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13", "a14", "a15"
  ]
}

#[derive(Debug)]
struct RegAlloc {
  var: HashMap<String, Register>,
  reg: HashMap<Register, Option<String>>,
}

#[allow(dead_code)]
impl RegAlloc {
  fn new() -> Self {
    let mut regs: HashMap<Register, Option<String>> = HashMap::new();
    for r in all_regs() {
      regs.insert(r, None);
    }
    Self {
      var: HashMap::new(),
      reg: regs,
    }
  }
  fn reset(&mut self) {
    self.var = HashMap::new();
    self.reg = HashMap::new();
    for r in all_regs() {
      self.reg.insert(r, None);
    }
  }
  fn find_var(&self, v: &str) -> Option<Register> {
    self.var.get(v).map(|x| *x)
  }
  fn get_reg_content(&self, r: Register) -> Option<&str> {
    self.reg.get(r).unwrap().as_ref().map(|x| &x[..])
  }
  fn find_empty_reg(&self) -> Option<Register> {
    for (r, o) in self.reg.iter() {
      if let None = o {
        return Some(r)
      }
    }
    None
  }
  fn find_any_reg(&self) -> Register {
    self.reg.iter().next().unwrap().0
  }
  fn make_reg_empty(&mut self, r: Register) {
    match self.get_reg_content(r) {
      Some(x) => {
        let x = &x.to_string();
        self.var.remove(x);
        self.reg.insert(r, None);
      },
      None => (),
    }
  }
  // 生きている変数のリストを返す
  fn find_all_alives(&self, program:&[knormal::Sent]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for (v, _) in self.var.iter() {
      if alive::is_alive(v, program) {
        res.push(v.clone());
      }
    }
    res
  }
  // 生きている変数を退避する命令列を返す
  fn save_all_alives(&self, program: &[knormal::Sent]) -> Vec<Inst> {
    let mut res: Vec<Inst> = Vec::new();
    for (v, r) in self.var.iter() {
      if alive::is_alive(v, program) {
        res.push(Inst::Save(r, v.clone()));
      }
    }
    res
  }
  // xにrを割り当てる
  fn alloc_reg(&mut self, x: &str, r: Register) {
    self.var.insert(x.to_string(), r);
    self.reg.insert(r, Some(x.to_string()));
  }
  // xにrを割り当てて溢れた変数をSaveする
  fn alloc_reg_and_save(&mut self, x: &str, r: Register) -> Vec<Inst> {
    match self.get_reg_content(r) {
      None => {
        self.alloc_reg(x, r);
        Vec::new()
      },
      Some(y) => {
        if x == y {
          Vec::new()
        } else {
          let y = y.to_string();
          self.var.remove(&y.to_string());
          self.alloc_reg(x, r);
          vec![Inst::Save(r, y.to_string())]
        }
      },
    }
  }
  // xにrを割り当てて必要に応じてMove, Restoreする(rに入っていたものの退避はしない)
  fn alloc_reg_and_restore(&mut self, x: &str, r: Register) -> Vec<Inst> {
    match self.find_var(x) {
      None => {
        self.make_reg_empty(r);
        self.alloc_reg(x, r);
        vec![Inst::Restore(r, x.to_string())]
      },
      Some(s) => {
        if s == r {
          Vec::new()
        } else {
          self.make_reg_empty(r);
          self.make_reg_empty(s);
          self.alloc_reg(x, r);
          vec![Inst::Mv(r, s)]
        }
      }
    }
  }
  // 変数xにレジスタを割り当てる
  fn alloc_any_reg(&mut self, x: &str) -> (Register, Vec<Inst>) {
    match self.find_var(x) {
      Some(r) => (r, Vec::new()), // すでに割り当てられていた
      None => {
        match self.find_empty_reg() {
          Some(r) => { // 空いているレジスタがあった
            self.alloc_reg(x, r);
            (r, Vec::new())
          },
          None => { // 空いていなければ任意のレジスタに割り当てる
            let r = self.find_any_reg();
            (r, self.alloc_reg_and_save(x, r))
          }
        }
      }
    }
  }
  // 変数xにレジスタを割り当て必要ならRestoreする
  fn alloc_any_reg_and_restore(&mut self, x: &str) -> (Register, Vec<Inst>) {
    match self.find_var(x) {
      Some(r) => (r, Vec::new()), // すでに割り当てられていた
      None => {
        match self.find_empty_reg() {
          Some(r) => { // 空いているレジスタがあった
            self.alloc_reg(x, r);
            (r, vec![Inst::Restore(r, x.to_string())])
          },
          None => { // 空いていなければ任意のレジスタに割り当てる
            let r = self.find_any_reg();
            let mut ist = self.alloc_reg_and_save(x, r);
            ist.push(Inst::Restore(r, x.to_string()));
            (r, ist)
          },
        }
      }
    }
  }
  fn set_arguments(&mut self, args: Vec<String>) -> Vec<Inst> {
    let mut res = Vec::new();
    for (x, r) in args.into_iter().zip(all_regs()) {
      let mut v = self.alloc_reg_and_restore(&x, r);
      res.append(&mut v);
    }
    res
  }
  fn alloc_arguments(&mut self, args: Vec<String>) {
    for (x, r) in args.into_iter().zip(all_regs()) {
      self.alloc_reg(&x, r);
    }
  }
}

fn sents_to_virtual(program: Vec<knormal::Sent>, alloc: &mut RegAlloc) -> Vec<Inst> {
  let mut insts: Vec<Inst> = Vec::new();
  for (i, sent) in program.clone().into_iter().enumerate() {
    let prog: &[knormal::Sent] = &program[i..];

    let mut v = sent.to_virtual(alloc, prog);
    insts.append(&mut v);
  }
  insts
}

impl knormal::Sent {
  fn to_virtual(
    self, 
    alloc: &mut RegAlloc,
    program: &[knormal::Sent]
  ) -> Vec<Inst> {
    let mut insts: Vec<Inst> = Vec::new();    
    match self {
      knormal::Sent::Assign(x, e) => {
        match e {
          knormal::Expr::Int(i) => {
            let (r, mut v) = alloc.alloc_any_reg(&x);
            insts.append(&mut v);
            insts.push(Inst::Li(r, i));
          },
          knormal::Expr::Float(f) => {
            let (r, mut v) = alloc.alloc_any_reg(&x);
            insts.append(&mut v);
            insts.push(Inst::FLi(r, f));
          },
          knormal::Expr::Var(y) => {
            let (r, mut v) = alloc.alloc_any_reg(&x);
            insts.append(&mut v);
            let (ry, mut v) = alloc.alloc_any_reg_and_restore(&y);
            insts.append(&mut v);
            insts.push(Inst::Mv(r, ry));
          },
          knormal::Expr::Op1(op, y) => {
            let (r, mut v) = alloc.alloc_any_reg(&x);
            insts.append(&mut v);
            let (ry, mut v) = alloc.alloc_any_reg_and_restore(&y);
            insts.append(&mut v);
            insts.push(Inst::Op1(r, op, ry));
          },
          knormal::Expr::Op2(op, y, z) => {
            let (r, mut v) = alloc.alloc_any_reg(&x);
            insts.append(&mut v);
            let (ry, mut v) = alloc.alloc_any_reg_and_restore(&y);
            insts.append(&mut v);
            let (rz, mut v) = alloc.alloc_any_reg_and_restore(&z);
            insts.append(&mut v);
            insts.push(Inst::Op2(r, op, ry, rz));
          },
          knormal::Expr::Call(f, args) => {
            eprintln!("{} {:?}", f, alloc.find_all_alives(&program[1..]));
            let mut v = alloc.save_all_alives(&program[1..]);
            insts.append(&mut v);
            let mut v = alloc.set_arguments(args);
            insts.append(&mut v);
            insts.push(Inst::Jal(f));
            alloc.reset();
            alloc.alloc_reg(&x, "a0");
          }
        }
      },
      knormal::Sent::IfElse(x, s, t) => {
        let (rx, mut v) = alloc.alloc_any_reg_and_restore(&x);
        insts.append(&mut v);
        let mut v = alloc.save_all_alives(program);
        insts.append(&mut v);
        alloc.reset();
        let mut sv = sents_to_virtual(s, alloc);
        let mut v = alloc.save_all_alives(&program[1..]); // if文以降で生きている変数
        sv.append(&mut v);

        alloc.reset();
        let mut tv = sents_to_virtual(t, alloc);
        let mut v = alloc.save_all_alives(&program[1..]);
        tv.append(&mut v);

        insts.push(Inst::IfElse(rx, sv, tv));
      },
      knormal::Sent::Return(o) => {
        if let Some(x) = o {
          insts = alloc.alloc_reg_and_restore(&x, "a0");
        }
        insts.push(Inst::Ret);
      },
    }
    insts
  }
}

impl knormal::Function {
  fn to_virtual(self) -> Function {
    // 引数の割り当てを設定
    let mut alloc: RegAlloc = RegAlloc::new();
    alloc.alloc_arguments(self.args);

    let insts: Vec<Inst> = sents_to_virtual(self.content, &mut alloc);
    
    Function{name: self.name, content: insts}
  }
}

impl knormal::Program {
  pub fn to_virtual(self) -> Program {
    Program{functions: self.functions.into_iter().map(|x| x.to_virtual()).collect()}
  }
}
