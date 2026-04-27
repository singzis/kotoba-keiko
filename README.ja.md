# kotoba-keiko（言葉稽古）

[中文](README.md) | [日本語](README.ja.md)

ターミナル上で使える**ひらがな ↔ ローマ字**の双方向ミニクイズです。  
問題をランダムに出題し、正誤を記録し、統計情報をローカルの **SQLite** に保存します。

- **kotoba**（言葉）：ことば・言語  
- **keiko**（稽古）：練習・習得  
- 実行コマンド名は `keiko` です（crate 名の `kotoba-keiko` とは異なり、crate はプロジェクト名、CLI は練習用の入口です）。

## 動作環境

- [Rust](https://www.rust-lang.org/) **stable**（このリポジトリには `rust-toolchain.toml` が含まれており、デフォルトで stable を使用します）
- 依存ライブラリ：`clap`、`rand`、`rusqlite`（SQLite は bundled 構成なので、ローカル環境でもビルドしやすくしています）

## ビルドと実行

```bash
cargo build --release
cargo run -- quiz      # または単に cargo run（デフォルトで quiz）
cargo run -- quiz --sokuon
cargo run -- quiz --dakuten
cargo run -- quiz --handakuten
cargo run -- quiz --yoon
cargo run -- quiz --all
cargo run -- quiz --dakuten --yoon
```

`PATH` にインストールすれば、`keiko` を直接実行できます。

```bash
cargo install --path .
keiko quiz
```

## サブコマンド

| コマンド | 説明 |
| --- | --- |
| `keiko` / `keiko quiz` | 練習を開始します。**ひらがな**または**ローマ字**がランダムに出題されるので、対応する答えを入力します。 |
| `keiko stats` | 累計統計と直近のセッション情報を表示します。 |
| `keiko review` | 問題プール内のひらがなとローマ字の対応表を表示します（データベースは使用しません）。 |
| `keiko reset` | 統計を初期化します。現在のディレクトリにある `keiko_stats.db` を削除します（誤操作防止のため 2 回確認します）。 |

## オプション

- `--sokuon`：`quiz` / `review` に**促音**を追加します
- `--dakuten`：`quiz` / `review` に**濁音**を追加します
- `--handakuten`：`quiz` / `review` に**半濁音**を追加します
- `--yoon`：`quiz` / `review` に**拗音**を追加します
- `--all`：**促音・濁音・半濁音・拗音**をまとめて追加します
- これらのオプションは組み合わせて使用できます。たとえば `--dakuten --yoon` を指定すると濁音と拗音に加えて濁拗音も対象になります。`--all` はすべて有効にするのと同じです。

練習中に `q`、`quit`、`exit` のいずれかを入力すると、そのラウンドを終了します。  
回答が 1 問以上ある場合のみ、結果がデータベースに保存されます。

## データ保存

- デフォルトでは現在の作業ディレクトリに `keiko_stats.db` を作成します
- できるだけ固定ディレクトリで実行するか、必要に応じて自分のバックアップ運用に合わせてください
- 統計を最初からやり直したい場合は `keiko reset` を実行し、案内に従ってまず `yes`、次に `DELETE`（大文字）を入力してください。別の入力をするとキャンセルされます

## ライセンス

別途明記がない限り、リポジトリルートの定義に従います。
