  jmp :main
fib:
  sub $sp, $sp, 9
  sw $ra, $sp, 0
  mv $a15, 2
  slt $a1, $a0, $a15
  sw $a1, $sp, 1
  sw $a0, $sp, 4
  beq $a1, $zero, .else.1
  lw $a0, $sp, 4
  jmp :.fib.end
  jmp :.cont.2
.else.1:
  mv $a11, 1
  lw $a6, $sp, 4
  sub $a15, $a6, $a11
  sw $a15, $sp, 3
  sw $a6, $sp, 4
  sw $a9, $sp, 6
  mv $a0, $a15
  jal :fib
  mv $a3, 2
  lw $a7, $sp, 4
  sub $a15, $a7, $a3
  sw $a0, $sp, 6
  sw $a15, $sp, 7
  sw $a5, $sp, 8
  mv $a0, $a15
  jal :fib
  lw $a12, $sp, 6
  add $a13, $a12, $a0
  mv $a0, $a13
  jmp :.fib.end
.cont.2:
.fib.end:
  lw $ra, $sp, 0
  add $sp, $sp, 9
  ret
main:
  sub $sp, $sp, 4
  sw $ra, $sp, 0
  mv $a5, 10
  sw $a5, $sp, 1
  sw $a4, $sp, 3
  mv $a0, $a5
  jal :fib
  sw $a0, $sp, 3
  jal :print_int
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 4
  ret
