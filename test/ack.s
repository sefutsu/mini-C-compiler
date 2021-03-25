  jmp :main
ack:
  sub $sp, $sp, 3
  sw $ra, $sp, 0
  mv $a3, 0
  sle $a4, $a0, $a3
  beq $a4, $zero, .else.1
  mv $a0, 1
  add $a0, $a1, $a0
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
  jmp :.cont.2
.else.1:
  mv $a3, 0
  sle $a4, $a1, $a3
  beq $a4, $zero, .else.3
  mv $a3, 1
  sub $a0, $a0, $a3
  mv $a1, 1
  jal :ack
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
  jmp :.cont.4
.else.3:
  mv $a3, 1
  sub $a4, $a0, $a3
  mv $a3, 1
  sub $a1, $a1, $a3
  sw $a4, $sp, 1
  jal :ack
  sw $a0, $sp, 2
  lw $a0, $sp, 1
  lw $a1, $sp, 2
  jal :ack
  lw $ra, $sp, 0
  add $sp, $sp, 3
  ret
.cont.4:
.cont.2:
main:
  sub $sp, $sp, 1
  sw $ra, $sp, 0
  mv $a0, 3
  mv $a1, 10
  jal :ack
  jal :print_int
  mv $a0, 0
  lw $ra, $sp, 0
  add $sp, $sp, 1
  ret

# out: 8189
# 実行命令数: 692819956
# スタック使用量: 24574
