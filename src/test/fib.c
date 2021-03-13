void print_int(int x);

int fib(int n) {
  if(n < 2) return n;
  else {
    return fib(n-1) + fib(n-2);
  }
}

void main(void) {
  int x = fib(10);
  print_int(x);
  return;
}
