# アセンブラ

### 更新情報
libmincaml.sを追加しました  
`-l`オプションが使えます

### 使い方
`python assembler.py [OPTIONS] [filename]`
- filenameは*.sまたは*.asmにすると良いです
- *.disas.s(人間用)とprogram.bit(機械語)が生成されます

#### OPTIONS
- `-s`: シンボルテーブルをstdoutに出力します
- `-pre`: *.pre.sを生成します(削除しません)
- `-d`: globals.sを読み込んでシンボルテーブルへの登録とdata.bitの生成を行います
- `-l`: libmincaml.sをリンクします
- `-nolab`: disas.sのジャンプ先にラベルを表示しません
- `-sim`: シミュレータ用ファイルを生成します

#### minrtのアセンブル例
`python assembler.py -l -d minrt.asm`

### エラー処理について
エラー検出機構は以下の2つの誤りを検出します
- labelの2重定義
- 命令の即値フィールドに収まらない即値

これら以外の誤りが入力ファイルにあった場合の動作は保証されません  
エラーが発生した行番号を表示する機能は実装されていませんが、出力ファイルを見ることでこれを特定することができます  
*.pre.s, program.bit において最後に出力された行の次の行がエラーが発生した行になります
