def convert_instr(line, pc):
  op, *operands = line.replace(',', ' ').split()

  if op == "ret":
    return "j $zero 0 $ra"

  elif op == "li":
    return f"add {operands[0]} $zero {operands[1]}"

  elif op == "mv":
    return ' '.join(["add"] + operands + ["$zero"])

  elif op == "bgt":
    return ' '.join(["blt", operands[1], operands[0], operands[2]])

  elif op == "bge":
    return ' '.join(["ble", operands[1], operands[0], operands[2]])

  elif op == "fbgt":
    return ' '.join(["fblt", operands[1], operands[0], operands[2]])

  elif op == "fbge":
    return ' '.join(["fble", operands[1], operands[0], operands[2]])

  elif op == "fneg":
    return ' '.join(["add"] + operands + ["0x80000000"])

  elif op == "not":
    return ' '.join(["seq"] + operands + ["$zero"])

  elif op == "jmp":
    return f"j $zero 0 {operands[0]}"

  elif op == "jal":
    return f"j $ra {pc+1} {operands[0]}"

  else:
    return ' '.join([op] + operands)
