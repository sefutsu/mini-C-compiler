#include <iostream>
#include <fstream>
#include "register.hpp"
#include "datamem.hpp"
#include "cpu.hpp"
#include "instruction.hpp"
#include "entry.hpp"
using namespace std;
#define DATA_MEM_WIDTH 18

void exec_all(){
  int minsp = (1 << DATA_MEM_WIDTH);
  int maxhp = 0;
  while(CPU::pc >= 0){
    CPU::exec_next_instruction();
    minsp = min(minsp, CPU::regs.geti("$sp"));
    maxhp = max(maxhp, CPU::regs.geti("$hp"));
  }
  cout << "実行命令数: " << CPU::cnt << endl;
  cout << "スタック使用量: " << (1 << DATA_MEM_WIDTH) - minsp << endl;
  cout << "ヒープ使用量: " << maxhp << endl;
  cout << "メモリ使用量: " << maxhp + (1 << DATA_MEM_WIDTH) - minsp << endl;
}

void exec(){
  int minsp = (1 << DATA_MEM_WIDTH);
  int maxhp = 0;
  while(CPU::pc >= 0){
    cout << CPU::pc << endl;
    CPU::exec_next_instruction();
    minsp = min(minsp, CPU::regs.geti("$sp"));
    maxhp = max(maxhp, CPU::regs.geti("$hp"));
  }
  cout << "実行命令数: " << CPU::cnt << endl;
  cout << "スタック使用量: " << (1 << DATA_MEM_WIDTH) - minsp << endl;
  cout << "ヒープ使用量: " << maxhp << endl;
  cout << "メモリ使用量: " << maxhp + (1 << DATA_MEM_WIDTH) - minsp << endl;
  CPU::regs.dumpall();
}


int main(int argc, char *argv[]){
  std::ifstream sim_file(argv[1], std::ios::in);
  CPU::imem.initialize(sim_file);
  sim_file.close();
  CPU::dmem.set_size((1 << DATA_MEM_WIDTH));
  CPU::regs.seti("$sp", (1 << DATA_MEM_WIDTH));
  CPU::regs.seti("$ra", -1);

  if(argc <= 2){
    exec_all();
  }else{
    exec();
  }
  return 0;
}
