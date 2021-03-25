int print_int(int x);

int main(void){
  int x = 1;
  if(1) {
    int y = 2;
    print_int(y);
  } else {
    int y = 3;
    print_int(y);
  }
  print_int(x);
  return 0;
}