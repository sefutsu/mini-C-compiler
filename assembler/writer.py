class DisasWriter:
  def __init__(self, filename, LAB=False):
    self.f = open(filename, "w")
    self.LAB = LAB
  def __call__(self, op, operands, label = None):
    operands = list(map(str, operands))
    self.f.write(op + ' ' + ', '.join(operands))
    if label and self.LAB:
      self.f.write(f" <{label}>")
    self.f.write("\n")
  def __del__(self):
    self.f.close()

class SimWriter:
  def __init__(self, filename):
    self.f = open(filename, "w")
  def __call__(self, op, operands):
    instr = [op] + list(map(str, operands))
    self.f.write(' '.join(map(str, instr)) + '\n')
  def __del__(self):
    self.f.close()
