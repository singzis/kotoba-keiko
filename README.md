# kotoba-keiko（言葉稽古）

[中文](README.md) | [日本語](README.ja.md)

终端里的**假名 ↔ 罗马音**双向小测验：默认练习平假名，也可切到片假名；随机出题、记录对错，统计写入本地 **SQLite**。

- **kotoba**（言葉）：词语、语言。  
- **keiko**（稽古）：练习、修习。  
- 可执行命令名：`**keiko`**（与 crate 名 `kotoba-keiko` 不同，crate 是工程名，CLI 是练习入口）。

## 环境

- [Rust](https://www.rust-lang.org/) **stable**（本仓库含 `rust-toolchain.toml`，默认跟随 stable）。
- 依赖：`clap`、`rand`、`rusqlite`（SQLite 使用 bundled，便于本机构建）。

## 构建与运行

```bash
cargo build --release
cargo run -- quiz      # 或直接 cargo run（默认即 quiz）
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

## 可视化预览

```bash
cargo run --bin stats-viewer
```

启动后打开 `http://127.0.0.1:7878`。页面会动态读取与 CLI 相同位置的 `$HOME/.keiko_stats.db`。

## 本地安装（懒人版）

在当前项目根目录复制执行下面两行，即可安装或覆盖旧版本并直接启动：

```bash
cargo install --path . --bin keiko --force
keiko
```

安装后，终端中的 `keiko` 是独立二进制；源码更新后，再执行一次上述命令即可同步最新版本。

## 子命令


| 命令                     | 说明                                                |
| ---------------------- | ------------------------------------------------- |
| `keiko` / `keiko quiz` | 开始练习；随机给出**假名**或**罗马音**，输入对应答案。                   |
| `keiko stats`          | 查看累计统计与最近若干次会话。                                   |
| `keiko review`         | 打印题库内全部假名与罗马音对照表（不访问数据库）。                         |
| `keiko reset`          | 清空统计：删除用户主目录下的 `~/.keiko_stats.db`（需两次交互确认，防止误删）。 |

## 可选参数

- `--sokuon`：在 `quiz` / `review` 中加入**促音**。
- `--katakana`：使用**片假名**题库；不加时默认使用平假名。
- `--dakuten`：在 `quiz` / `review` 中加入**浊音**。
- `--handakuten`：在 `quiz` / `review` 中加入**半浊音**。
- `--yoon`：在 `quiz` / `review` 中加入**拗音**。
- `--all`：一次性加入**促音、浊音、半浊音、拗音**。
- `--romaji-only`：只显示**罗马音**题面，要求输入对应假名。
- `--kana-only`：只显示**假名**题面，要求输入对应罗马音。
- `--romaji-only` 与 `--kana-only` 不能同时使用；二者都不指定时，每道题默认随机显示假名或罗马音。
- 题库参数可组合使用；例如 `--dakuten --yoon` 会加入浊音与拗音，并额外包含浊拗音；`--katakana --dakuten` 会练习片假名浊音；`--all` 则等价于全部开启。


退出练习：按下 Ctrl-C 后会显示 `Press Ctrl-C again to exit`；在约 1.5 秒内再次按下 Ctrl-C 即可退出。

## 数据存储

- 默认在用户主目录生成 `~/.keiko_stats.db`，不同运行目录会共享同一份统计数据。
- 如需备份或迁移统计数据，请备份该文件。
- 若要从头累计统计：运行 `keiko reset`，按提示先输入 `yes`，再输入 `DELETE`（全大写）；取消则输入其它内容或直接中止。

## 许可

若未另行声明，以仓库根目录为准。
