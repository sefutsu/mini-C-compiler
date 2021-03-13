#pragma once

#include "instruction.hpp"
#include <vector>
#include <string>
#include <iostream>


class InstructionMem {
  std::vector<Instruction> instmem;
  public:
  InstructionMem() {}
  void initialize(std::istream& is){
    while(1){
      std::string op;
      is >> op;
      if(op == "") break;
      Instruction inst = get_instruction_class(op);
      inst.read_ops(is);
      instmem.push_back(inst);
    }
  }
  Instruction& get(int addr){
    return instmem[addr];
  }
};