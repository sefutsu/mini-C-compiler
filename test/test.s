  jmp :main
succ:
  sub $sp, $sp, 2
  sw $ra, $sp, 0
  mv $a4, 1
  sw $a0, $sp, 1
  add $a0, $a0, $a4
  lw $a4, $sp, 1
  add $a0, $a0, $a4
  jmp :.succ.end
.succ.end:
  lw $ra, $sp, 0
  add $sp, $sp, 2
  ret
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  mv $a0, 1
  jal :succ
  jal :print_int
  mv $a0, 0
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret
