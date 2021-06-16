#[macro_use]
extern crate lalrpop_util;
use std::io::Read;

mod util;
mod ast;
// mod knormal;
// mod virtuals;
// mod asm;

lalrpop_mod!(pub c);

// fn compile(prog: ast::Program) -> Result<asm::Program, String> {
//   let alphaed = prog.alpha()?;
//   eprintln!("Alphaed: {:#?}", alphaed);
//   let typed = alphaed.typing()?;
//   eprintln!("Typed: {:#?}", typed);
//   let knormaled = typed.to_knormal();
//   eprintln!("Knormaled: {:#?}", knormaled);
//   let virtualized = knormaled.to_virtual();
//   eprintln!("Virtualized: {:#?}", virtualized);
//   let assembly = virtualized.to_asm();
//   // eprintln!("Assembly: {:#?}\n\n", assembly);
//   Ok(assembly)
// }

fn main() {
  let parser = c::ProgramParser::new();
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf).unwrap();
  match parser.parse(&buf) {
    Ok(s) => {
      eprintln!("Parsed: {:#?}", s);
      // match compile(s) {
      //   Ok(a) => print!("{}", a),
      //   Err(e) => eprintln!("Compile Error: {}", e),
      // }
    }
    Err(e) => eprintln!("Parse error: {:#?}", e),
  }
}
