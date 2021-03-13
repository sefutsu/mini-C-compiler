  jmp :main
fib:
  sub $sp, $sp, 9
  sw $ra, $sp, 0
  mv $a13, 2
  slt $a8, $a0, $a13
  sw $a8, $sp, 1
  sw $a0, $sp, 4
  beq $a8, $zero, .else.1
  lw $a0, $sp, 4
  jmp :.fib.end
  jmp :.cont.2
.else.1:
  mv $a4, 1
  lw $a3, $sp, 4
  sub $a12, $a3, $a4
  sw $a9, $sp, 7
  sw $a3, $sp, 4
  sw $a12, $sp, 5
  mv $a0, $a12
  jal :fib
  mv $a15, 2
  lw $a9, $sp, 4
  sub $a12, $a9, $a15
  sw $a12, $sp, 6
  sw $a0, $sp, 7
  sw $a10, $sp, 8
  mv $a0, $a12
  jal :fib
  lw $a6, $sp, 7
  add $a1, $a6, $a0
  mv $a0, $a1
  jmp :.fib.end
.cont.2:
.fib.end:
  lw $ra, $sp, 0
  add $sp, $sp, 9
  ret
main:
  sub $sp, $sp, 4
  sw $ra, $sp, 0
  mv $a9, 10
  sw $a9, $sp, 1
  sw $a12, $sp, 3
  mv $a0, $a9
  jal :fib
  sw $a0, $sp, 3
  jal :print_int
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 4
  ret
