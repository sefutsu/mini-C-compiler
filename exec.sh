#!/bin/zsh

cargo run < test/$1.c > test/$1.s
python assembler/assembler.py test/io.s test/$1.s
simulator/simulator test/$1.sim.s
