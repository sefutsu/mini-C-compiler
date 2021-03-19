int print_int(int x);

int succ(int x){
  int y = x + 1;
  y = y + x;
  return y;
}
int main(void){
  print_int(succ(1));
  return 0;
}