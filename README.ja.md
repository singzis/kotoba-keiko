# kotoba-keiko（言葉稽古）

[中文](README.md) | [日本語](README.ja.md)

ターミナル上で使える**かな ↔ ローマ字**の双方向ミニクイズです。
デフォルトではひらがなを練習し、オプションでカタカナに切り替えられます。
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
cargo run -- quiz --katakana
cargo run -- quiz --katakana --dakuten
cargo run -- quiz --romaji-only
cargo run -- quiz --kana-only
```

`PATH` にインストールすれば、`keiko` を直接実行できます。

```bash
cargo install --path . --bin keiko
keiko quiz
```

注意：`cargo run` は現在のソースコードを使います。一方、ターミナルで直接実行する `keiko` はインストール済みの独立したバイナリです。ソースコード更新後に `keiko` コマンドへ新しい挙動を反映するには、再インストールしてください。

```bash
cargo install --path . --bin keiko --force
```

## サブコマンド

| コマンド | 説明 |
| --- | --- |
| `keiko` / `keiko quiz` | 練習を開始します。**かな**または**ローマ字**がランダムに出題されるので、対応する答えを入力します。 |
| `keiko stats` | 累計統計と直近のセッション情報を表示します。 |
| `keiko review` | 問題プール内のかなとローマ字の対応表を表示します（データベースは使用しません）。 |
| `keiko reset` | 統計を初期化します。ユーザーのホームディレクトリにある `~/.keiko_stats.db` を削除します（誤操作防止のため 2 回確認します）。 |

## オプション

- `--sokuon`：`quiz` / `review` に**促音**を追加します
- `--katakana`：**カタカナ**の問題プールを使用します。指定しない場合はひらがなです
- `--dakuten`：`quiz` / `review` に**濁音**を追加します
- `--handakuten`：`quiz` / `review` に**半濁音**を追加します
- `--yoon`：`quiz` / `review` に**拗音**を追加します
- `--all`：**促音・濁音・半濁音・拗音**をまとめて追加します
- `--romaji-only`：**ローマ字**だけを問題として表示し、対応するかなを入力します
- `--kana-only`：**かな**だけを問題として表示し、対応するローマ字を入力します
- `--romaji-only` と `--kana-only` は同時に指定できません。どちらも指定しない場合は、問題ごとにかなまたはローマ字がランダムに表示されます
- 問題プールのオプションは組み合わせて使用できます。たとえば `--dakuten --yoon` を指定すると濁音と拗音に加えて濁拗音も対象になります。`--katakana --dakuten` はカタカナ濁音を対象にします。`--all` はすべて有効にするのと同じです。

練習中に Ctrl-C を押すと `Press Ctrl-C again to exit` と表示されます。約 1.5 秒以内にもう一度 Ctrl-C を押すと終了します。

## データ保存

- デフォルトではユーザーのホームディレクトリに `~/.keiko_stats.db` を作成します。実行ディレクトリが異なっても同じ統計データを共有します
- 統計データをバックアップまたは移行する場合は、このファイルを対象にしてください
- 統計を最初からやり直したい場合は `keiko reset` を実行し、案内に従ってまず `yes`、次に `DELETE`（大文字）を入力してください。別の入力をするとキャンセルされます

## ライセンス

別途明記がない限り、リポジトリルートの定義に従います。
