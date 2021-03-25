int print_int(int x);

int f(int x, float x){
  return x;
}

int main(void){
  print_int(f(1, 1));
  return 0;
}