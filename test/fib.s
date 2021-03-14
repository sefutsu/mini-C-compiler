  jmp :main
fib:
  sub $sp, $sp, 9
  sw $ra, $sp, 0
  mv $a2, 2
  slt $a3, $a0, $a2
  sw $a0, $sp, 5
  sw $a3, $sp, 2
  beq $a3, $zero, .else.1
  lw $a0, $sp, 5
  jmp :.fib.end
  jmp :.cont.2
.else.1:
  mv $a7, 1
  lw $a11, $sp, 5
  sub $a9, $a11, $a7
  sw $a9, $sp, 3
  sw $a3, $sp, 7
  sw $a11, $sp, 5
  mv $a0, $a9
  jal :fib
  mv $a3, 2
  lw $a1, $sp, 5
  sub $a14, $a1, $a3
  sw $a14, $sp, 6
  sw $a0, $sp, 7
  sw $a11, $sp, 8
  mv $a0, $a14
  jal :fib
  lw $a7, $sp, 7
  add $a3, $a7, $a0
  mv $a0, $a3
  jmp :.fib.end
.cont.2:
.fib.end:
  lw $ra, $sp, 0
  add $sp, $sp, 9
  ret
main:
  sub $sp, $sp, 5
  sw $ra, $sp, 0
  sw $a0, $sp, 2
  jal :read_int
  sw $a0, $sp, 2
  sw $a6, $sp, 4
  jal :fib
  sw $a0, $sp, 4
  jal :print_int
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 5
  ret
