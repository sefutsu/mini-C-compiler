#include "print.hpp"
#include <iostream>
#include <iomanip>
#include <cstdio>
#include "entry.hpp"

void print32(Entry n, char base, bool showbase){
  if(base == 'f'){
    printf("%6f", n.f);
    if(showbase){
      std::cout << 'f';
    }
  }else{
    print32(n.i, base, showbase);
  }
}

void print32(int n, char base, bool showbase){
  using namespace std;
  if(base == 16){
    if(showbase){
      cout << "0x";
    }
    printf("%08x", n);
  }else{
    printf("%d", n);
  }
}