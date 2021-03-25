use crate::ast;

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

mod alive;
pub mod reg_alloc;
