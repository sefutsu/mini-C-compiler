  jmp :main
fun:
  sub $sp, $sp, 6
  sw $ra, $sp, 0
  add $a6, $a0, $a1
  add $a7, $a6, $a2
  sw $a6, $sp, 1
  add $a6, $a7, $a3
  sw $a6, $sp, 4
  sw $a6, $sp, 3
  lw $a6, $sp, 4
  add $a6, $a6, $a4
  sw $a6, $sp, 4
  sw $a6, $sp, 5
  lw $a6, $sp, 3
  add $a6, $a6, $a5
  lw $a0, $sp, 5
  jmp :.fun.end
.fun.end:
  lw $ra, $sp, 0
  add $sp, $sp, 6
  ret
main:
  sub $sp, $sp, 4
  sw $ra, $sp, 0
  mv $a6, 1
  mv $a2, 2
  mv $a7, 3
  mv $a0, 4
  mv $a3, 5
  mv $a1, 6
  sw $a0, $sp, 1
  mv $a0, $a6
  sw $a1, $sp, 2
  mv $a1, $a2
  mv $a2, $a7
  sw $a3, $sp, 3
  lw $a3, $sp, 1
  lw $a4, $sp, 3
  lw $a5, $sp, 2
  jal :fun
  jal :print_int
  mv $a3, 0
  mv $a0, $a3
  jmp :.main.end
.main.end:
  lw $ra, $sp, 0
  add $sp, $sp, 4
  ret
