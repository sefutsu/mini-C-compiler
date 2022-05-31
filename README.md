# mini-C-compiler
a small compiler from C to an original ISA assembly.

## Usage
C source code from stdin, compiled assembly to stdout.
```
cargo run < test/sample.c > test/sample.s
```
Rust compiler is required.

## Test
Compiled assembly can be executed using simulator.

Fisrt, compile the simulator. C++ compiler is required.
```
make -C simulator
```

Then, you can compile and run your C source code such as `test/hoge.c` by
```
./exec.sh hoge
```
