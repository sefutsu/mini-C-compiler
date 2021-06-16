#pragma once

#include "register.hpp"
#include "datamem.hpp"
#include "instmem.hpp"
#include "instruction.hpp"
#include <string>
#include <map>

namespace CPU {
  using std::string;
  extern Register regs;
  extern DataMem dmem;
  extern InstructionMem imem;
  extern int pc;
  extern long long cnt;
  void exec_next_instruction();
  void nop();
  void add(string&, string&, string&);
  void sub(string&, string&, string&);
  void mul(string&, string&, string&);
  void div(string&, string&, string&);
  void mod(string&, string&, string&);
  void sll(string&, string&, string&);
  void sra(string&, string&, string&);
  void fun_or(string&, string&, string&);
  void fun_and(string&, string&, string&);
  void seq(string&, string&, string&);
  void sne(string&, string&, string&);
  void slt(string&, string&, string&);
  void sle(string&, string&, string&);
  void fseq(string&, string&, string&);
  void fsne(string&, string&, string&);
  void fslt(string&, string&, string&);
  void fsle(string&, string&, string&);
  void j(string&, string&, string&);
  void blt(string&, string&, string&);
  void ble(string&, string&, string&);
  void beq(string&, string&, string&);
  void bne(string&, string&, string&);
  void fbeq(string&, string&, string&);
  void fbne(string&, string&, string&);
  void fblt(string&, string&, string&);
  void fble(string&, string&, string&);
  void lw(string&, string&, string&);
  void sw(string&, string&, string&);
  void fadd(string&, string&, string&);
  void fsub(string&, string&, string&);
  void fmul(string&, string&, string&);
  void fdiv(string&, string&, string&);
  void fabs(string&, string&);
  void fsqrt(string&, string&);
  void floor(string&, string&);
  void ftoi(string&, string&);
  void itof(string&, string&);
  void in(string&);
  void out(string&);
  int geti(string&);
  unsigned int getu(string&);
  float getf(string&);
};