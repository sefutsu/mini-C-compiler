#pragma once

#include "print.hpp"
#include <vector>
#include <iostream>

class DataMem {
  int size;
  std::vector<unsigned> mem;
  public:
  DataMem(int size = 1024){
    this->size = size;
    this->mem = std::vector<unsigned>(size);
  }
  void set_size(int size){
    this->size = size;
    this->mem = std::vector<unsigned>(size);
  }
  void check_addr(int addr){
    if(addr < 0 || addr >= this->size){
      std::cerr << "Invalid Address " << addr << std::endl;
      exit(-1);
    }
  }
  int get(int addr){
    check_addr(addr);
    return mem[addr];
  }
  void set(int addr, int v){
    check_addr(addr);
    mem[addr] = v;
  }
  void dump(int start, int length=1, char base=16){
    start = std::max(start, 0);
    int end = std::min(start + length, this->size);
    for(int addr = start; addr < end; addr++){
      print32(addr);
      std::cout << ": ";
      print32(mem[addr], base, true);
      std::cout << std::endl;
    }
  }
};