#include "cpu.hpp"
#include "entry.hpp"
#include <climits>
#include <cmath>
#include <map>
using std::string;

namespace CPU {
  Register regs;
  DataMem dmem;
  InstructionMem imem;

  int pc;
  long long cnt;

  void exec_next_instruction(){
    Instruction& inst = imem.get(pc);
    cnt++;
    inst.exec();
  }
  void nop(){
    pc++;
    cnt--;
  }
  void add(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x+y);
    pc++;
  }
  void sub(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x-y);
    pc++;
  }
  void mul(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x*y);
    pc++;
  }
  void div(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x/y);
    pc++;
  }
  void mod(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x%y);
    pc++;
  }
  void sll(string& r0, string& r1, string& r2){
    unsigned x = geti(r1);
    int y = geti(r2);
    regs.setu(r0, x << y);
    pc++;
  }
  void sra(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x >> y);
    pc++;
  }
  void fun_and(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x & y);
    pc++;
  }
  void fun_or(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    regs.seti(r0, x | y);
    pc++;
  }
  void seq(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    int z = (x == y);
    regs.seti(r0, z);
    pc++;
  }
  void sne(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    int z = (x != y);
    regs.seti(r0, z);
    pc++;
  }
  void slt(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    int z = (x < y);
    regs.seti(r0, z);
    pc++;
  }
  void sle(string& r0, string& r1, string& r2){
    int x = geti(r1);
    int y = geti(r2);
    int z = (x <= y);
    regs.seti(r0, z);
    pc++;
  }
  void fseq(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    int z = (x == y);
    regs.seti(r0, z);
    pc++;
  }
  void fsne(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    int z = (x != y);
    regs.seti(r0, z);
    pc++;
  }
  void fslt(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    int z = (x < y);
    regs.seti(r0, z);
    pc++;
  }
  void fsle(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    int z = (x <= y);
    regs.seti(r0, z);
    pc++;
  }
  void sw(string& rs, string& rt, string& n){
    int addr = regs.geti(rt) + stoll(n, nullptr, 0);
    dmem.set(addr, regs.getu(rs));
    pc++;
  }
  void lw(string& r0, string& r1, string& r2){
    int addr = geti(r1) + geti(r2);
    regs.setu(r0, dmem.get(addr));
    pc++;
  }
  void blt(string& r0, string& r1, string& imm){
    int x = geti(r0);
    int y = geti(r1);
    if(x < y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void ble(string& r0, string& r1, string& imm){
    int x = geti(r0);
    int y = geti(r1);
    if(x <= y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void beq(string& r0, string& r1, string& imm){
    int x = geti(r0);
    int y = geti(r1);
    if(x == y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void bne(string& r0, string& r1, string& imm){
    int x = geti(r0);
    int y = geti(r1);
    if(x != y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void fblt(string& r0, string& r1, string& imm){
    float x = getf(r0);
    float y = getf(r1);
    if(x < y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void fble(string& r0, string& r1, string& imm){
    float x = getf(r0);
    float y = getf(r1);
    if(x <= y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void fbeq(string& r0, string& r1, string& imm){
    float x = getf(r0);
    float y = getf(r1);
    if(x == y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void fbne(string& r0, string& r1, string& imm){
    float x = getf(r0);
    float y = getf(r1);
    if(x != y){
      pc += stoi(imm, nullptr, 0);
    }else{
      pc++;
    }
  }
  void j(string& d, string& n, string& s){
    regs.seti(d, stoll(n, nullptr, 0));
    pc = geti(s);
  }
  void fadd(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    float z = x + y;
    regs.setf(r0, z);
    pc++;
  }
  void fsub(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    float z = x - y;
    regs.setf(r0, z);
    pc++;
  }
  void fmul(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    float z = x * y;
    regs.setf(r0, z);
    pc++;
  }
  void fdiv(string& r0, string& r1, string& r2){
    float x = getf(r1);
    float y = getf(r2);
    float z = x / y;
    regs.setf(r0, z);
    pc++;
  }
  void fabs(string& r0, string& r1){
    float x = getf(r1);
    float y = std::fabs(x);
    regs.setf(r0, y);
    pc++;
  }
  void fsqrt(string& r0, string& r1){
    float x = getf(r1);
    regs.setf(r0, std::sqrt(x));
    pc++;
  }
  void floor(string& r0, string& r1){
    float x = getf(r1);
    regs.setf(r0, std::floor(x));
    pc++;
  }
  void ftoi(string& r0, string& r1){
    float x = getf(r1);
    regs.seti(r0, std::round(x));
    pc++;
  }
  void itof(string& r0, string& r1){
    int x = geti(r1);
    regs.setf(r0, (float)x);
    pc++;
  }
  void in(string& r0){
    string s = regs.read_token();
    if(s.find('.') == string::npos){
      regs.seti(r0, stoi(s));
    }else{
      regs.setf(r0, stof(s));
    }
    pc++;
  }
  void out(string& r0){
    int x = geti(r0);
    regs.print_int(x);
    pc++;
  }
  int geti(string& s){
    if(s[0] == '$'){
      return regs.geti(s);
    }else{
      return (int)stoll(s, nullptr, 0);
    }
  }
  unsigned int getu(string& s){
    if(s[0] == '$'){
      return regs.getu(s);
    }else{
      return (unsigned)stoll(s, nullptr, 0);
    }
  }
  float getf(string& s){
    if(s[0] == '$'){
      return regs.getf(s);
    }else{
      Entry x;
      x.u = (unsigned)stoll(s, nullptr, 0);
      return x.f;
    }
  }
}
