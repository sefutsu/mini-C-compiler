#[derive(Debug, Clone)]
pub enum Op1 {
  Neg, FNeg,
  Cast(Type),
}

#[derive(Debug, Clone)]
pub enum Op2 {
  Add, Sub, Mul, Div, Mod,
  Lshift, Rshift,
  And, Or, Xor,
  FAdd, FSub, FMul, FDiv,
  Eq, Ne, Lt, Le,
  FEq, FNe, FLt, FLe,
}

#[derive(Debug, Clone)]
pub enum Type {
  Void,
  Int,
  Float,
}

#[derive(Debug, Clone)]
pub struct FunType {
  pub ret: Type,
  pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Void,
  Int(i32),
  Float(f32),
  Var(String),
  Op1(Op1, Box<Expr>),
  Op2(Op2, Box<Expr>, Box<Expr>),
  Assign(String, Box<Expr>),
  Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stat {
  Void,
  Expression(Box<Expr>),
  Decl(Type, String, Option<Expr>),
  Compound(Vec<Stat>),
  Return(Option<Expr>),
  IfElse(Box<Expr>, Box<Stat>, Box<Stat>),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub ret_type: Type,
  pub name: String,
  pub args: Vec<(Type, String)>,
  pub content: Stat,
}

#[derive(Debug, Clone)]
pub struct Program {
  pub functions: Vec<Function>,
}

// mod alpha;
// mod typing;
// mod to_knormal;
