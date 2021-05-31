  jmp :main
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  mv $a7, 1
  mv $a5, 1
  add $a2, $a7, $a5
  mv $a7, $a2
  mv $a0, $a7
  jal :print_int
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
