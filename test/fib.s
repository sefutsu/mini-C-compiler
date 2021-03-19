  jmp :main
fib:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a4, 2
  slt $a5, $a0, $a4
  sw $a0, $sp, 1
  beq $a5, $zero, .else.1
  jmp :.fib.end
  jmp :.cont.2
.else.1:
  mv $a4, 1
  sub $a5, $a0, $a4
  sw $a0, $sp, 1
  mv $a0, $a5
  jal :fib
  mv $a7, 2
  lw $a4, $sp, 1
  sub $a6, $a4, $a7
  sw $a0, $sp, 2
  mv $a0, $a6
  jal :fib
  lw $a1, $sp, 2
  add $a2, $a1, $a0
  mv $a0, $a2
  jmp :.fib.end
.cont.2:
.fib.end:
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  jal :read_int
  jal :fib
  jal :print_int
  mv $a2, 0
  mv $a0, $a2
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
