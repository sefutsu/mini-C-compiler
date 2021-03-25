  jmp :main
main:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a1, 1
  mv $a3, 1
  beq $a3, $zero, .else.1
  mv $a0, 2
  sw $a1, $sp, 2
  jal :print_int
  lw $a1, $sp, 2
  jmp :.cont.2
.else.1:
  mv $a0, 3
  sw $a1, $sp, 2
  jal :print_int
  lw $a1, $sp, 2
.cont.2:
  mv $a0, $a1
  jal :print_int
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
