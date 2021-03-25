int print_int(int x);

int main(void){
  int x = 1;
  if(1) {
    int x = 2;
    print_int(x);
  } else {}
  print_int(x);
  return 0;
}