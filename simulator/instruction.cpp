#include "instruction.hpp"
#include "cpu.hpp"

void Instruction::exec(){
  switch (nop){
    case 0:
      f.op0();
      break;
    case 1:
      f.op1(op[0]);
      break;
    case 2:
      f.op2(op[0], op[1]);
      break;
    case 3:
      f.op3(op[0], op[1], op[2]);
      break;
    default:
      break;
  }
}

void Instruction::read_ops(std::istream& is){
  this->op = std::vector<std::string>(this->nop);
  for(int i = 0; i < this->nop; i++){
    is >> this->op[i];
  }
}

std::map<std::string, Instruction> instr{
  {"nop", Instruction(CPU::nop)},
  {"add", Instruction(CPU::add)},
  {"sub", Instruction(CPU::sub)},
  {"mul", Instruction(CPU::mul)},
  {"div", Instruction(CPU::div)},
  {"mod", Instruction(CPU::mod)},
  {"sll", Instruction(CPU::sll)},
  {"sra", Instruction(CPU::sra)},
  {"and", Instruction(CPU::fun_and)},
  {"or", Instruction(CPU::fun_or)},
  {"xor", Instruction(CPU::fun_xor)},
  {"seq", Instruction(CPU::seq)},
  {"sne", Instruction(CPU::sne)},
  {"slt", Instruction(CPU::slt)},
  {"sle", Instruction(CPU::sle)},
  {"fseq", Instruction(CPU::fseq)},
  {"fsne", Instruction(CPU::fsne)},
  {"fslt", Instruction(CPU::fslt)},
  {"fsle", Instruction(CPU::fsle)},
  {"lw", Instruction(CPU::lw)},
  {"sw", Instruction(CPU::sw)},
  {"j", Instruction(CPU::j)},
  {"blt", Instruction(CPU::blt)},
  {"ble", Instruction(CPU::ble)},
  {"beq", Instruction(CPU::beq)},
  {"bne", Instruction(CPU::bne)},
  {"fblt", Instruction(CPU::fblt)},
  {"fble", Instruction(CPU::fble)},
  {"fbeq", Instruction(CPU::fbeq)},
  {"fbne", Instruction(CPU::fbne)},
  {"fadd", Instruction(CPU::fadd)},
  {"fsub", Instruction(CPU::fsub)},
  {"fmul", Instruction(CPU::fmul)},
  {"fdiv", Instruction(CPU::fdiv)},
  {"fabs", Instruction(CPU::fabs)},
  {"fsqrt", Instruction(CPU::fsqrt)},
  {"floor", Instruction(CPU::floor)},
  {"itof", Instruction(CPU::itof)},
  {"ftoi", Instruction(CPU::ftoi)},
  {"in", Instruction(CPU::in)},
  {"out", Instruction(CPU::out)},
};

Instruction get_instruction_class(std::string op){
  if(!instr.count(op)){
    std::cerr << "Instruction \"" << op << "\" is not defined\n";
    exit(-1);
  }
  return instr[op];
}