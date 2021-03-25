  jmp :main
fib:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a7, 2
  slt $a4, $a0, $a7
  beq $a4, $zero, .else.1
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
  jmp :.cont.2
.else.1:
  mv $a7, 1
  sw $a0, $sp, 1
  sub $a0, $a0, $a7
  jal :fib
  mv $a2, 2
  lw $a6, $sp, 1
  sw $a0, $sp, 2
  sub $a0, $a6, $a2
  jal :fib
  lw $a1, $sp, 2
  add $a0, $a1, $a0
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
