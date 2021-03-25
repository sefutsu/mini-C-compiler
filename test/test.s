  jmp :main
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  mv $a5, 1
  itof $a7, $a5
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
