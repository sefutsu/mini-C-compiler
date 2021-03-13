import instruction as ist
import writer
import sys
import re
import os

def convert_label(s):
  if s[0] != ':':
    return s
  s = s[1:]
  if '+' in s:
    lab, imm = s.split('+')
    imm = int(imm, 0)
  else:
    lab, imm = s, 0
  global symbol_table
  return symbol_table[lab] + imm
  

if len(sys.argv) < 2:
  print(f"usage: python assembler.py [OPTIONS] [filename]", file=sys.stderr)
  sys.exit()

source_file_names = [sys.argv[-1]] + [x for x in sys.argv[1:-1] if x[0] != '-']
raw_file_name = (sys.argv[-1]).replace(".s", "").replace(".asm", "")
pre_file_name = raw_file_name + ".pre.s"
disas_file_name = raw_file_name + ".disas.s"
sim_file_name = raw_file_name + ".sim.s"

SYM = "-s" in sys.argv
PRE = "-pre" in sys.argv
DISAS = "-disas" in sys.argv

#### *.s -> *.pre.s

pre_file = open(pre_file_name, "w")
line_cnt = 0

symbol_table = {}

for source_file_name in source_file_names:
  source_file = open(source_file_name, "r")
  while True:
    line = source_file.readline()
    if not line: break
    line = re.sub("#.*\n", "", line) #remove comment
    line = line.rstrip().lstrip() #\n will also be removed
    if not line: continue # only space line
    if line[-1] == ':': # label
      symbol = line.rstrip(':')
      if symbol in symbol_table.keys():
        raise NameError(f"label \"{symbol}\" is used twice in {source_file_name}")
      symbol_table[symbol] = line_cnt
    else:
      converted_line = ist.convert_instr(line, line_cnt)
      pre_file.write(converted_line + '\n')
      line_cnt += 1 + converted_line.count('\n')
  source_file.close()

pre_file.close()

if SYM: #ouput symbol table
  print(symbol_table)

#### *.pre.s -> *.sim.s

dw = writer.DisasWriter(disas_file_name)
sw = writer.SimWriter(sim_file_name)
pre_file = open(pre_file_name, "r")

line_cnt = 0
while True:
  line = pre_file.readline()
  if not line: break
  parsed_line = line.split()
  
  op, *operands = parsed_line
  
  label = None
  if op == "j":
    if operands[2][0] == ':': label = operands[2][1:]
    operands[2] = convert_label(operands[2])
    
  elif op in ["beq", "bne", "blt", "ble", "fbeq", "fbne", "fblt", "fble"]:
    label = operands[2]
    operands[2] = symbol_table[operands[2]] - line_cnt

  else:
    operands = list(map(convert_label, operands))

  dw(op, operands, label)
  sw(op, operands)
  line_cnt += 1

pre_file.close()
del dw
del sw

if not PRE:
  os.remove(pre_file_name)
if not DISAS:
  os.remove(disas_file_name)
