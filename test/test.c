void print_int(int x);

int fun(int a, int b, int c, int d, int e, int f){
  return a + b + c + d + e + f;
}

int main(void){
  int x = fun(1, 2, 3, 4, 5, 6);
  print_int(x);
  return 0;
}
