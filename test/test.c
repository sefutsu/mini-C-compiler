int fib(int n) {
  if(n < 2) return n;
  else {
    return fib(n-1.0) + fib(n-2);
  }
}

int main(){
  int x = 1;
  ;
  float y = .2;
  x = 1 + x * -y;
  fib(10.0);
  return 0.0;
}