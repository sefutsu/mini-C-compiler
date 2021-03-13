# C++シミュレータ

## 使い方
- `sim` オプションを付けてアセンブルしてシミュレーションソースを生成します  
`python assembler.py -sim file.asm`
- `make`
- `./simulator file.sim.s input.sld output.ppm`
- 第4引数に任意の文字列を渡すと1命令実行ごとにpcの値を出力する
