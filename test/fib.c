void print_int(int x);
int read_int(void);

int fib(int n) {
  if(n < 2) return n;
  else {
    return fib(n-1) + fib(n-2);
  }
}

void main(void) {
  int n = read_int();
  int x = fib(n);
  print_int(x);
  return;
}
