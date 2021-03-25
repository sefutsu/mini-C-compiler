  jmp :main
main:
  sub $sp, $sp, 2
  sw $ra, $sp, 0
  mv $a6, 1
  mv $a2, 1
  beq $a2, $zero, .else.1
  mv $a0, 2
  sw $a6, $sp, 1
  jal :print_int
  lw $a6, $sp, 1
  jmp :.cont.2
.else.1:
.cont.2:
  mv $a0, $a6
  jal :print_int
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 2
  ret
