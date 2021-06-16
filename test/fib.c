void print_int(int x);
int read_int(void);

int fib(int n) {
  if(n < 2 && n >= 0) return n;
  else {
    return fib(n-1) + fib(n-2);
  }
}

int main(void) {
  int n = read_int();
  int x = fib(n);
  print_int(x);
  return 0;
}
