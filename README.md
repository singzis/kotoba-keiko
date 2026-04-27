# kotoba-keiko（言葉稽古）

[中文](README.md) | [日本語](README.ja.md)

终端里的**平假名 ↔ 罗马音**双向小测验：随机出题、记录对错，统计写入本地 **SQLite**。

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
```

安装到本机 `PATH` 后可直接敲 `keiko`：

```bash
cargo install --path .
keiko quiz
```

## 子命令


| 命令                     | 说明                                                |
| ---------------------- | ------------------------------------------------- |
| `keiko` / `keiko quiz` | 开始练习；随机给出**平假名**或**罗马音**，输入对应答案。                  |
| `keiko stats`          | 查看累计统计与最近若干次会话。                                   |
| `keiko review`         | 打印题库内全部平假名与罗马音对照表（不访问数据库）。                        |
| `keiko reset`          | 清空统计：删除当前目录下的 `**keiko_stats.db`**（需两次交互确认，防止误删）。 |

## 可选参数

- `--sokuon`：在 `quiz` / `review` 中加入**促音**。
- `--dakuten`：在 `quiz` / `review` 中加入**浊音**。
- `--handakuten`：在 `quiz` / `review` 中加入**半浊音**。
- `--yoon`：在 `quiz` / `review` 中加入**拗音**。
- `--all`：一次性加入**促音、浊音、半浊音、拗音**。
- 这些参数都可组合使用；例如 `--dakuten --yoon` 会加入浊音与拗音，并额外包含浊拗音；`--all` 则等价于全部开启。


退出练习：输入 `**q`**、`**quit**` 或 `**exit**` 后结束本轮；若本轮有作答，会写入数据库。

## 数据存储

- 默认在当前工作目录生成 `**keiko_stats.db**`。  
- 请在固定目录下运行，或自行把该文件与备份策略对齐。  
- 若要从头累计统计：运行 `**keiko reset**`，按提示先输入 `**yes**`，再输入 `**DELETE**`（全大写）；取消则输入其它内容或直接中止。

## 许可

若未另行声明，以仓库根目录为准。
