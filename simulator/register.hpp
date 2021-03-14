#pragma once

#include <map>
#include <string>
#include <vector>
#include <tuple>
#include <iostream>
#include <iomanip>

#include "entry.hpp"
#include "print.hpp"

class Register{
  using string = std::string;
  std::vector<string> reg_names;
  std::map<string, Entry> regs;
  std::istream *is;
  std::ostream *os;
  public:
  Register(){
    reg_names = std::vector<string>({
      "$zero",
      "$ra",
      "$sp",
      "$hp",
      "$ca",
      "$a0",
      "$a1",
      "$a2",
      "$a3",
      "$a4",
      "$a5",
      "$a6",
      "$a7",
      "$a8",
      "$a9",
      "$a10",
      "$a11",
      "$a12",
      "$a13",
      "$a14",
      "$a15",
      "$s0",
      "$s1",
      "$s2",
      "$s3",
      "$s4",
      "$s5",
      "$s6",
      "$s7"
    });
    Entry zero;
    zero.i = 0;
    for(string s: reg_names){
      regs[s] = zero;
    }
  }
  string read_token(){
    string res;
    std::cin >> res;
    return res;
  }
  void print_int(int b){
    std::cout << "out: " << b << std::endl;
  }
  int geti(string name){
    return regs[name].i;
  }
  unsigned int getu(string name){
    return regs[name].u;
  }
  float getf(string name){
    return regs[name].f;
  }
  Entry get(string name){
    return regs[name];
  }
  void seti(string name, int v){
    regs[name].i = v;
  }
  void setu(string name, unsigned int v){
    regs[name].u = v;
  }
  void setf(string name, float v){
    regs[name].f = v;
  }
  void dump(string name, char base = 16, bool showbase = true){
    print32(regs[name], base, showbase);
  }
  void dumpall(){
    for(string name: reg_names){
      std::cout << name << ": ";
      print32(regs[name], 16, true);
      std::cout << ' ';
      print32(regs[name], 10, true);
      std::cout << ' ';
      print32(regs[name], 'f', true);
      std::cout << std::endl;
    }
  }
};