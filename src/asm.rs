#![allow(unused_must_use)]

#[derive(Debug, Clone)]
pub enum Operand {
  Reg(&'static str),
  Sp,
  Ra,
  Int(i32),
  Float(f32),
}

#[derive(Debug, Clone)]
pub enum Inst {
  Label(String),
  Mv(Operand, Operand),
  Add(Operand, Operand, Operand),
  Sub(Operand, Operand, Operand),
  Mul(Operand, Operand, Operand),
  Div(Operand, Operand, Operand),
  FAdd(Operand, Operand, Operand),
  FSub(Operand, Operand, Operand),
  FMul(Operand, Operand, Operand),
  FDiv(Operand, Operand, Operand),
  Seq(Operand, Operand, Operand),
  Sne(Operand, Operand, Operand),
  Slt(Operand, Operand, Operand),
  Sle(Operand, Operand, Operand),
  FSeq(Operand, Operand, Operand),
  FSne(Operand, Operand, Operand),
  FSlt(Operand, Operand, Operand),
  FSle(Operand, Operand, Operand),
  Neg(Operand, Operand),
  FNeg(Operand, Operand),
  Beqz(Operand, String),
  Jmp(String),
  Jal(String),
  Lw(Operand, Operand, Operand),
  Sw(Operand, Operand, Operand),
  Itof(Operand, Operand),
  Ftoi(Operand, Operand),
  Ret,
}

#[derive(Debug, Clone)]
pub struct Program {
  pub program: Vec<Inst>,
}

impl std::fmt::Display for Operand {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Reg(r) => write!(f, "${}", r),
      Self::Sp => write!(f, "$sp"),
      Self::Ra => write!(f, "$ra"),
      Self::Int(i) => write!(f, "{}", i),
      Self::Float(i) => write!(f, "{}", i.to_bits()),
    }
  }
}

impl std::fmt::Display for Inst {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Label(s) => write!(f, "{}:", s),
      Self::Mv(a, b) => write!(f, "  mv {}, {}", a, b),
      Self::Add(a, b, c) => write!(f, "  add {}, {}, {}", a, b, c),
      Self::Sub(a, b, c) => write!(f, "  sub {}, {}, {}", a, b, c),
      Self::Mul(a, b, c) => write!(f, "  mul {}, {}, {}", a, b, c),
      Self::Div(a, b, c) => write!(f, "  div {}, {}, {}", a, b, c),
      Self::FAdd(a, b, c) => write!(f, "  fadd {}, {}, {}", a, b, c),
      Self::FSub(a, b, c) => write!(f, "  fsub {}, {}, {}", a, b, c),
      Self::FMul(a, b, c) => write!(f, "  fmul {}, {}, {}", a, b, c),
      Self::FDiv(a, b, c) => write!(f, "  fdiv {}, {}, {}", a, b, c),
      Self::Seq(a, b, c) => write!(f, "  seq {}, {}, {}", a, b, c),
      Self::Sne(a, b, c) => write!(f, "  sne {}, {}, {}", a, b, c),
      Self::Slt(a, b, c) => write!(f, "  slt {}, {}, {}", a, b, c),
      Self::Sle(a, b, c) => write!(f, "  sle {}, {}, {}", a, b, c),
      Self::FSeq(a, b, c) => write!(f, "  fseq {}, {}, {}", a, b, c),
      Self::FSne(a, b, c) => write!(f, "  fsne {}, {}, {}", a, b, c),
      Self::FSlt(a, b, c) => write!(f, "  fslt {}, {}, {}", a, b, c),
      Self::FSle(a, b, c) => write!(f, "  fsle {}, {}, {}", a, b, c),
      Self::Neg(a, b) => write!(f, "  sub {}, $zero, {}", a, b),
      Self::FNeg(a, b) => write!(f, "  fneg {}, {}", a, b),
      Self::Beqz(a, l) => write!(f, "  beq {}, $zero, {}", a, l),
      Self::Jmp(l) => write!(f, "  jmp :{}", l),
      Self::Jal(l) => write!(f, "  jal :{}", l),
      Self::Lw(a, b, c) => write!(f, "  lw {}, {}, {}", a, b, c),
      Self::Sw(a, b, c) => write!(f, "  sw {}, {}, {}", a, b, c),
      Self::Itof(a, b) => write!(f, "  itof {}, {}", a, b),
      Self::Ftoi(a, b) => write!(f, "  ftoi {}, {}", a, b),
      Self::Ret => write!(f, "  ret"),
    }
  }
}

impl std::fmt::Display for Program {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for i in self.program.iter() {
      write!(f, "{}\n", i);
    }
    Ok(())
  }
}

#[test]
fn test_operand() {
  let x: u32 = 0x3f800000;
  assert_eq!(Operand::Float(1.0).to_string(), format!("{}", x));
}
#[test]
fn test_inst() {
  assert_eq!(
    Inst::Add(Operand::Reg("a0"), Operand::Reg("a1"), Operand::Int(2)).to_string(),
    "  add $a0, $a1, 2".to_string()
  )
}