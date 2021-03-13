#pragma once

#include <string>
#include <vector>
#include <iostream>
#include <map>

class Instruction {
  public:
  int nop;
  std::vector<std::string> op;
  union {
    unsigned long n;
    void (*op0)();
    void (*op1)(std::string&);
    void (*op2)(std::string&, std::string&);
    void (*op3)(std::string&, std::string&, std::string&);
  } f;
  Instruction () {}
  Instruction(void (*g)()) {
    f.op0 = g;
    nop = 0;
  }
  Instruction(void (*g)(std::string&)) {
    f.op1 = g;
    nop = 1;
  }
  Instruction(void (*g)(std::string&, std::string&)) {
    f.op2 = g;
    nop = 2;
  }
  Instruction(void (*g)(std::string&, std::string&, std::string&)) {
    f.op3 = g;
    nop = 3;
  }
  unsigned long get_func(){
    return f.n;
  }
  void exec();
  void read_ops(std::istream& is);
};

Instruction get_instruction_class(std::string);

extern std::map<std::string, Instruction> instr;