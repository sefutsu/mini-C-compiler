void print_int(int x);
int read_int(void);

int ack(int x, int y) {
  if (x <= 0) {
    return y + 1;
  } else if (y <= 0) {
    return ack(x - 1, 1);
  } else {
    return ack(x - 1, ack(x, y - 1));
  }
}

int main(void) {
  int x = ack(3, 10);
  print_int(x);
  return 0;
}
