  jmp :main
fib:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a3, 2
  slt $a1, $a0, $a3
  beq $a1, $zero, .else.1
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
  jmp :.cont.2
.else.1:
  mv $a3, 1
  sw $a0, $sp, 1
  sub $a0, $a0, $a3
  jal :fib
  mv $a6, 2
  lw $a1, $sp, 1
  sw $a0, $sp, 2
  sub $a0, $a1, $a6
  jal :fib
  lw $a6, $sp, 2
  add $a0, $a6, $a0
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
.cont.2:
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  jal :read_int
  jal :fib
  jal :print_int
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
