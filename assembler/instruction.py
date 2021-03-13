reg_name = [
  "$zero",
  "$ra",
  "$sp",
  "$hp",
  "$ca",
  "$a0",
  "$a1",
  "$a2",
  "$a3",
  "$a4",
  "$a5",
  "$a6",
  "$a7",
  "$a8",
  "$a9",
  "$a10",
  "$a11",
  "$a12",
  "$a13",
  "$a14",
  "$a15",
  "$s0",
  "$s1",
  "$s2",
  "$s3",
  "$s4",
  "$s5",
  "$s6",
  "$s7"
]
reg_idx = {}
for i, name in enumerate(reg_name):
  reg_idx[name] = i

OPF_dict = {
  "add": "00000",
  "sub": "00010",
  "mul4": "00100",
  "div2": "00110",
  "seq": "10000",
  "sne": "10010",
  "slt": "10100",
  "sle": "10110",
  "fseq": "11000",
  "fsne": "11010",
  "fslt": "11100",
  "fsle": "11110",

  "fadd": "00001",
  "fsub": "00011",
  "fmul": "00101",
  "fdiv": "00111",
  "finv": "01001",
  "fsqrt": "01011",
  "itof": "10001",
  "ftoi": "10011",
  "floor": "10101",
  "fabs": "10111",
  "exp2": "11001",
}

def n2b(n, l=6):
  if not (-(1 << (l-1)) <= n < (1 << l)):
    raise ValueError(f"{n} is too large to filled in length {l}")
  if n >= 0:
    res = bin(n)[2:]
    return '0' * (l - len(res)) + res
  else:
    return bin(((1 << l) - 1) & n)[2:]

def vint(n):
  if(type(n) == int): return n
  return int(n, 0)

def isreg(s):
  return type(s) == str and s[0] == '$'

class R_type:
  def __init__(self, op, operands):
    global reg_idx, reg_name
    self.op = op
    if op == "sw":
      s, t, n = operands
      if t[0] != '$':
        print("Wrong Order for instruction sw")
        s, n, t = operands
      self.rd = 0
      self.rs = reg_idx[s]
      self.rt = reg_idx[t]
      self.n = vint(n)
      self.IS = 1
      self.IT = 0
    else:
      operands = operands[:]
      if op == "out": operands = [reg_name[0]] + operands # rd = 0
      operands += [reg_name[0]] * 2 # 0埋め
      d, s, t, *_ = operands
      self.rd = reg_idx[d]
      if not isreg(s) and not isreg(t):
        raise ValueError(f"CANNOT use 2 immediate in instruction {op}")
      if not isreg(s):
        self.rs = 0
        self.rt = reg_idx[t]
        self.n = vint(s)
        self.IS = 1
        self.IT = 0
      elif not isreg(t):
        self.rs = reg_idx[s]
        self.rt = 0
        self.n = vint(t)
        self.IS = 0
        self.IT = 1
      else:
        self.rs = reg_idx[s]
        self.rt = reg_idx[t]
        self.n = 0
        self.IS = 0
        self.IT = 0
  def get_code(self):
    global OPF_dict
    if self.op in OPF_dict.keys():
      OPF = OPF_dict[self.op]
    else:
      OPF = "00000"
    B = '0'
    LD = '1' if self.op == "lw" else '0'
    ST = '1' if self.op == "sw" else '0'
    IN = '1' if self.op == "in" else '0'
    OUT = '1' if self.op == "out" else '0'
    PAD = '0'
    J = '0'
    return B + OPF + LD + ST + IN + OUT + PAD + J
  def binary(self):
    return (
      str(self.IS) + str(self.IT) + self.get_code() + \
        n2b(self.rd) + n2b(self.rs) + n2b(self.rt),
      n2b(self.n, 32)
    )

class B_type:
  def __init__(self, op, operands):
    self.op = op
    s, t, lab = operands
    self.lab = vint(lab)
    if not isreg(s) and not isreg(t):
      raise ValueError(f"CANNOT use 2 immediate in instruction {op}")
    if not isreg(s):
      self.rs = 0
      self.rt = reg_idx[t]
      self.n = vint(s)
      self.IS = 1
      self.IT = 0
    elif not isreg(t):
      self.rs = reg_idx[s]
      self.rt = 0
      self.n = vint(t)
      self.IS = 0
      self.IT = 1
    else:
      self.rs = reg_idx[s]
      self.rt = reg_idx[t]
      self.n = 0
      self.IS = 0
      self.IT = 0
  def binary(self):
    global OPF_dict
    code = OPF_dict[self.op.replace('b', 's')][:4]
    return (
      str(self.IS) + str(self.IT) + '1' + code + \
        n2b(self.lab, 13) + n2b(self.rs) + n2b(self.rt),
      n2b(self.n, 32)
    )

class J_type:
  def __init__(self, operands):
    d, n, s = operands
    global reg_idx
    self.rd = reg_idx[d]
    self.n = vint(n)
    if isreg(s):
      self.rs = reg_idx[s]
      self.s = 0
      self.IS = 0
    else:
      self.rs = 0
      self.s = vint(s)
      self.IS = 1
  def binary(self):
    sb = n2b(self.s, 16)
    return (
      str(self.IS) + "10" + \
      sb[:10] + "1" + n2b(self.rd) + n2b(self.rs) + sb[10:],
      n2b(self.n, 32)
    )


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

  elif op == "reti":
    return f"j $a0 {operands[0]} $ra"

  else:
    return ' '.join([op] + operands)


if __name__ == '__main__':
  c = B_type("fblt", ["$a0", "0xf3", "-1"])
  print(c.binary())
  c = R_type("sw", ["$a0", "$a0", "-1"])
  print(c.binary())

