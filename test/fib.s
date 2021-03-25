  jmp :main
fib:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a1, 2
  slt $a6, $a0, $a1
  beq $a6, $zero, .else.1
  jmp :.fib.end
  jmp :.cont.2
.else.1:
  mv $a1, 1
  sw $a0, $sp, 1
  sub $a0, $a0, $a1
  jal :fib
  mv $a2, 2
  lw $a4, $sp, 1
  sw $a0, $sp, 2
  sub $a0, $a4, $a2
  jal :fib
  lw $a6, $sp, 2
  add $a0, $a6, $a0
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
  mv $a0, 0
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
