use clap::{Args, Parser, Subcommand};
use rand::prelude::IndexedRandom;
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// ANSI color codes for terminal output
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";

const EXIT_HINT: &str = "输入 q / quit / exit 可退出练习";
/// 平假名分组布局，对应 [5,5,5,5,5,5,5,3,5,2,1] = 46 个元素
/// 各组：a,o,k,s,t,n,h,m,y,r,w,n（每组首个罗马音）
const GROUP_LAYOUT: &[usize] = &[5, 5, 5, 5, 5, 5, 5, 3, 5, 2, 1];

#[derive(Parser)]
#[command(
    name = "keiko",
    version,
    about = "平假名与罗马音双向练习器（含 SQLite 统计）"
)]
struct Cli {
    #[command(flatten)]
    quiz_options: QuizOptions,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args, Clone, Copy, Debug, Default, Eq, PartialEq)]
struct QuizOptions {
    /// 在题库中加入促音
    #[arg(long = "sokuon", global = true)]
    include_sokuon: bool,
    /// 在题库中加入浊音
    #[arg(long = "dakuten", global = true)]
    include_dakuten: bool,
    /// 在题库中加入半浊音
    #[arg(long = "handakuten", global = true)]
    include_handakuten: bool,
    /// 在题库中加入拗音
    #[arg(long = "yoon", global = true)]
    include_yoon: bool,
    /// 一次性加入促音、浊音、半浊音、拗音
    #[arg(long = "all", global = true)]
    include_all: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 开始练习（默认）
    Quiz,
    /// 查看统计数据
    Stats,
    /// 打印全部平假名与罗马音（对照速览）
    Review,
    /// 清空本地统计库（删除 keiko_stats.db，需两次确认）
    Reset,
    /// 查看每个字的正确率详情
    Detail,
}

#[derive(Clone, Copy)]
struct KanaItem {
    hira: &'static str,
    roma: &'static str,
}

#[derive(Clone, Copy)]
struct KanaCategory {
    name: &'static str,
    items: &'static [KanaItem],
}

type AnswerStatsMap = HashMap<(String, String), (i64, i64)>;

#[derive(Clone, Copy)]
enum Direction {
    HiraToRoma,
    RomaToHira,
}

/// Groups items from a slice according to a layout specification.
///
/// The `layout` array specifies the number of items in each group.
/// Items are consumed sequentially from `items` and distributed into groups.
///
/// # Arguments
///
/// * `items` - The slice of items to be grouped
/// * `layout` - An array specifying how many items should be in each group
///
/// # Returns
///
/// A vector of slices, where each inner slice contains a group of item references.
///
/// # Example
///
/// ```
/// let items = [1, 2, 3, 4, 5, 6];
/// let layout = [3, 2, 1];
/// let groups = group_by_layout(&items, &layout);
/// // groups == [[1,2,3], [4,5], [6]]
/// ```
pub fn group_by_layout<'a, T>(items: &'a [T], layout: &[usize]) -> Vec<&'a [T]> {
    let mut groups = Vec::with_capacity(layout.len());
    let mut start = 0;
    for &count in layout {
        if start + count > items.len() {
            break;
        }
        let end = start + count;
        groups.push(&items[start..end]);
        start = end;
    }
    groups
}

const KANA_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "あ",
        roma: "a",
    },
    KanaItem {
        hira: "い",
        roma: "i",
    },
    KanaItem {
        hira: "う",
        roma: "u",
    },
    KanaItem {
        hira: "え",
        roma: "e",
    },
    KanaItem {
        hira: "お",
        roma: "o",
    },
    KanaItem {
        hira: "か",
        roma: "ka",
    },
    KanaItem {
        hira: "き",
        roma: "ki",
    },
    KanaItem {
        hira: "く",
        roma: "ku",
    },
    KanaItem {
        hira: "け",
        roma: "ke",
    },
    KanaItem {
        hira: "こ",
        roma: "ko",
    },
    KanaItem {
        hira: "さ",
        roma: "sa",
    },
    KanaItem {
        hira: "し",
        roma: "shi",
    },
    KanaItem {
        hira: "す",
        roma: "su",
    },
    KanaItem {
        hira: "せ",
        roma: "se",
    },
    KanaItem {
        hira: "そ",
        roma: "so",
    },
    KanaItem {
        hira: "た",
        roma: "ta",
    },
    KanaItem {
        hira: "ち",
        roma: "chi",
    },
    KanaItem {
        hira: "つ",
        roma: "tsu",
    },
    KanaItem {
        hira: "て",
        roma: "te",
    },
    KanaItem {
        hira: "と",
        roma: "to",
    },
    KanaItem {
        hira: "な",
        roma: "na",
    },
    KanaItem {
        hira: "に",
        roma: "ni",
    },
    KanaItem {
        hira: "ぬ",
        roma: "nu",
    },
    KanaItem {
        hira: "ね",
        roma: "ne",
    },
    KanaItem {
        hira: "の",
        roma: "no",
    },
    KanaItem {
        hira: "は",
        roma: "ha",
    },
    KanaItem {
        hira: "ひ",
        roma: "hi",
    },
    KanaItem {
        hira: "ふ",
        roma: "fu",
    },
    KanaItem {
        hira: "へ",
        roma: "he",
    },
    KanaItem {
        hira: "ほ",
        roma: "ho",
    },
    KanaItem {
        hira: "ま",
        roma: "ma",
    },
    KanaItem {
        hira: "み",
        roma: "mi",
    },
    KanaItem {
        hira: "む",
        roma: "mu",
    },
    KanaItem {
        hira: "め",
        roma: "me",
    },
    KanaItem {
        hira: "も",
        roma: "mo",
    },
    KanaItem {
        hira: "や",
        roma: "ya",
    },
    KanaItem {
        hira: "ゆ",
        roma: "yu",
    },
    KanaItem {
        hira: "よ",
        roma: "yo",
    },
    KanaItem {
        hira: "ら",
        roma: "ra",
    },
    KanaItem {
        hira: "り",
        roma: "ri",
    },
    KanaItem {
        hira: "る",
        roma: "ru",
    },
    KanaItem {
        hira: "れ",
        roma: "re",
    },
    KanaItem {
        hira: "ろ",
        roma: "ro",
    },
    KanaItem {
        hira: "わ",
        roma: "wa",
    },
    KanaItem {
        hira: "を",
        roma: "wo",
    },
    KanaItem {
        hira: "ん",
        roma: "n",
    },
];

const DAKUON_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "が",
        roma: "ga",
    },
    KanaItem {
        hira: "ぎ",
        roma: "gi",
    },
    KanaItem {
        hira: "ぐ",
        roma: "gu",
    },
    KanaItem {
        hira: "げ",
        roma: "ge",
    },
    KanaItem {
        hira: "ご",
        roma: "go",
    },
    KanaItem {
        hira: "ざ",
        roma: "za",
    },
    KanaItem {
        hira: "じ",
        roma: "ji",
    },
    KanaItem {
        hira: "ず",
        roma: "zu",
    },
    KanaItem {
        hira: "ぜ",
        roma: "ze",
    },
    KanaItem {
        hira: "ぞ",
        roma: "zo",
    },
    KanaItem {
        hira: "だ",
        roma: "da",
    },
    KanaItem {
        hira: "ぢ",
        roma: "di",
    },
    KanaItem {
        hira: "づ",
        roma: "du",
    },
    KanaItem {
        hira: "で",
        roma: "de",
    },
    KanaItem {
        hira: "ど",
        roma: "do",
    },
    KanaItem {
        hira: "ば",
        roma: "ba",
    },
    KanaItem {
        hira: "び",
        roma: "bi",
    },
    KanaItem {
        hira: "ぶ",
        roma: "bu",
    },
    KanaItem {
        hira: "べ",
        roma: "be",
    },
    KanaItem {
        hira: "ぼ",
        roma: "bo",
    },
];

const HANDAKUON_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "ぱ",
        roma: "pa",
    },
    KanaItem {
        hira: "ぴ",
        roma: "pi",
    },
    KanaItem {
        hira: "ぷ",
        roma: "pu",
    },
    KanaItem {
        hira: "ぺ",
        roma: "pe",
    },
    KanaItem {
        hira: "ぽ",
        roma: "po",
    },
];

const SOKUON_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "っか",
        roma: "kka",
    },
    KanaItem {
        hira: "っき",
        roma: "kki",
    },
    KanaItem {
        hira: "っく",
        roma: "kku",
    },
    KanaItem {
        hira: "っけ",
        roma: "kke",
    },
    KanaItem {
        hira: "っこ",
        roma: "kko",
    },
    KanaItem {
        hira: "っさ",
        roma: "ssa",
    },
    KanaItem {
        hira: "っし",
        roma: "sshi",
    },
    KanaItem {
        hira: "っす",
        roma: "ssu",
    },
    KanaItem {
        hira: "っせ",
        roma: "sse",
    },
    KanaItem {
        hira: "っそ",
        roma: "sso",
    },
    KanaItem {
        hira: "った",
        roma: "tta",
    },
    KanaItem {
        hira: "っち",
        roma: "cchi",
    },
    KanaItem {
        hira: "っつ",
        roma: "ttsu",
    },
    KanaItem {
        hira: "って",
        roma: "tte",
    },
    KanaItem {
        hira: "っと",
        roma: "tto",
    },
    KanaItem {
        hira: "っぱ",
        roma: "ppa",
    },
    KanaItem {
        hira: "っぴ",
        roma: "ppi",
    },
    KanaItem {
        hira: "っぷ",
        roma: "ppu",
    },
    KanaItem {
        hira: "っぺ",
        roma: "ppe",
    },
    KanaItem {
        hira: "っぽ",
        roma: "ppo",
    },
];

const YOON_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "きゃ",
        roma: "kya",
    },
    KanaItem {
        hira: "きゅ",
        roma: "kyu",
    },
    KanaItem {
        hira: "きょ",
        roma: "kyo",
    },
    KanaItem {
        hira: "しゃ",
        roma: "sha",
    },
    KanaItem {
        hira: "しゅ",
        roma: "shu",
    },
    KanaItem {
        hira: "しょ",
        roma: "sho",
    },
    KanaItem {
        hira: "ちゃ",
        roma: "cha",
    },
    KanaItem {
        hira: "ちゅ",
        roma: "chu",
    },
    KanaItem {
        hira: "ちょ",
        roma: "cho",
    },
    KanaItem {
        hira: "にゃ",
        roma: "nya",
    },
    KanaItem {
        hira: "にゅ",
        roma: "nyu",
    },
    KanaItem {
        hira: "にょ",
        roma: "nyo",
    },
    KanaItem {
        hira: "ひゃ",
        roma: "hya",
    },
    KanaItem {
        hira: "ひゅ",
        roma: "hyu",
    },
    KanaItem {
        hira: "ひょ",
        roma: "hyo",
    },
    KanaItem {
        hira: "みゃ",
        roma: "mya",
    },
    KanaItem {
        hira: "みゅ",
        roma: "myu",
    },
    KanaItem {
        hira: "みょ",
        roma: "myo",
    },
    KanaItem {
        hira: "りゃ",
        roma: "rya",
    },
    KanaItem {
        hira: "りゅ",
        roma: "ryu",
    },
    KanaItem {
        hira: "りょ",
        roma: "ryo",
    },
];

const EXTENDED_YOON_TABLE: &[KanaItem] = &[
    KanaItem {
        hira: "ぎゃ",
        roma: "gya",
    },
    KanaItem {
        hira: "ぎゅ",
        roma: "gyu",
    },
    KanaItem {
        hira: "ぎょ",
        roma: "gyo",
    },
    KanaItem {
        hira: "じゃ",
        roma: "ja",
    },
    KanaItem {
        hira: "じゅ",
        roma: "ju",
    },
    KanaItem {
        hira: "じょ",
        roma: "jo",
    },
    KanaItem {
        hira: "びゃ",
        roma: "bya",
    },
    KanaItem {
        hira: "びゅ",
        roma: "byu",
    },
    KanaItem {
        hira: "びょ",
        roma: "byo",
    },
    KanaItem {
        hira: "ぴゃ",
        roma: "pya",
    },
    KanaItem {
        hira: "ぴゅ",
        roma: "pyu",
    },
    KanaItem {
        hira: "ぴょ",
        roma: "pyo",
    },
];

const KANA_CATEGORIES: &[KanaCategory] = &[
    KanaCategory {
        name: "清音",
        items: KANA_TABLE,
    },
    KanaCategory {
        name: "浊音",
        items: DAKUON_TABLE,
    },
    KanaCategory {
        name: "半浊音",
        items: HANDAKUON_TABLE,
    },
    KanaCategory {
        name: "促音",
        items: SOKUON_TABLE,
    },
    KanaCategory {
        name: "拗音",
        items: YOON_TABLE,
    },
    KanaCategory {
        name: "拗音（浊/半浊）",
        items: EXTENDED_YOON_TABLE,
    },
];

fn main() {
    if let Err(err) = run() {
        eprintln!("运行失败：{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Commands::Quiz);
    validate_quiz_options(&command, cli.quiz_options)?;

    match command {
        Commands::Review => {
            print_kana_chart(cli.quiz_options);
            Ok(())
        }
        Commands::Quiz => {
            let conn = open_db()?;
            run_quiz(&conn, cli.quiz_options)
        }
        Commands::Stats => {
            let conn = open_db()?;
            show_stats(&conn)
        }
        Commands::Reset => reset_db(),
        Commands::Detail => {
            let conn = open_db()?;
            show_detail(&conn)
        }
    }
}

fn validate_quiz_options(command: &Commands, quiz_options: QuizOptions) -> Result<(), String> {
    if (quiz_options.include_sokuon
        || quiz_options.include_dakuten
        || quiz_options.include_handakuten
        || quiz_options.include_yoon
        || quiz_options.include_all)
        && !matches!(command, Commands::Quiz | Commands::Review)
    {
        return Err(
            "`--sokuon`、`--dakuten`、`--handakuten`、`--yoon`、`--all` 仅可与 `quiz` 或 `review` 一起使用"
                .to_string(),
        );
    }
    Ok(())
}

fn include_sokuon(options: QuizOptions) -> bool {
    options.include_all || options.include_sokuon
}

fn include_dakuten(options: QuizOptions) -> bool {
    options.include_all || options.include_dakuten
}

fn include_handakuten(options: QuizOptions) -> bool {
    options.include_all || options.include_handakuten
}

fn include_yoon(options: QuizOptions) -> bool {
    options.include_all || options.include_yoon
}

fn reset_db() -> Result<(), String> {
    let path = db_path();
    if !path.exists() {
        println!("未找到 {}，无需清空。", path.display());
        return Ok(());
    }

    println!(
        "即将删除 {}（全部练习统计），此操作不可恢复。",
        path.display()
    );

    print!("第一次确认：请输入 yes 继续（其它输入取消）：");
    io::stdout()
        .flush()
        .map_err(|e| format!("刷新输出失败：{e}"))?;
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| format!("读取输入失败：{e}"))?;
    if !line.trim().eq_ignore_ascii_case("yes") {
        println!("已取消。");
        return Ok(());
    }

    print!("第二次确认：请输入 DELETE 确认清空（须全大写，其它输入取消）：");
    io::stdout()
        .flush()
        .map_err(|e| format!("刷新输出失败：{e}"))?;
    line.clear();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| format!("读取输入失败：{e}"))?;
    if line.trim() != "DELETE" {
        println!("已取消。");
        return Ok(());
    }

    fs::remove_file(&path).map_err(|e| format!("删除数据库失败：{e}"))?;
    println!("已清空：{} 已删除。", path.display());
    Ok(())
}

fn open_db() -> Result<Connection, String> {
    let db_path = db_path();
    let conn = Connection::open(&db_path).map_err(|e| format!("无法打开数据库：{e}"))?;
    init_db(&conn).map_err(|e| format!("初始化数据库失败：{e}"))?;
    Ok(conn)
}

fn build_quiz_pool(quiz_options: QuizOptions) -> Vec<&'static KanaItem> {
    let mut pool: Vec<&KanaItem> = KANA_TABLE.iter().collect();

    if include_dakuten(quiz_options) {
        pool.extend(DAKUON_TABLE.iter());
    }

    if include_handakuten(quiz_options) {
        pool.extend(HANDAKUON_TABLE.iter());
    }

    if include_sokuon(quiz_options) {
        pool.extend(SOKUON_TABLE.iter());
    }

    if include_yoon(quiz_options) {
        pool.extend(YOON_TABLE.iter());
        if include_dakuten(quiz_options) || include_handakuten(quiz_options) {
            pool.extend(EXTENDED_YOON_TABLE.iter());
        }
    }

    pool
}

fn selected_feature_labels(quiz_options: QuizOptions) -> Vec<&'static str> {
    let mut labels = Vec::new();
    if include_sokuon(quiz_options) {
        labels.push("促音");
    }
    if include_dakuten(quiz_options) {
        labels.push("浊音");
    }
    if include_handakuten(quiz_options) {
        labels.push("半浊音");
    }
    if include_yoon(quiz_options) {
        labels.push(
            if include_dakuten(quiz_options) || include_handakuten(quiz_options) {
                "拗音（含浊/半浊拗音）"
            } else {
                "拗音"
            },
        );
    }
    labels
}

fn print_group(name: &str, items: &[KanaItem]) {
    println!("\n【{name}】");
    for item in items {
        print!("{:>3} ({:<5}) ", item.hira, item.roma);
    }
    println!();
}

/// Prints the complete kana chart grouped by standard Japanese order.
/// Groups are: a,k,s,t,n,h,m,y,r,w,n corresponding to vowel, ka, sa, ta, na, ha, ma, ya, ra, wa rows plus n.
fn print_kana_chart(quiz_options: QuizOptions) {
    println!("平假名 ↔ 罗马音（练习用全表）");
    println!("{}", "—".repeat(48));

    let groups = group_by_layout(KANA_TABLE, GROUP_LAYOUT);
    for group in groups {
        let group_name = group.first().unwrap().roma.chars().next().unwrap();
        print_group(&group_name.to_uppercase().to_string(), group);
    }

    if include_dakuten(quiz_options) {
        print_group("浊音", DAKUON_TABLE);
    }
    if include_handakuten(quiz_options) {
        print_group("半浊音", HANDAKUON_TABLE);
    }
    if include_sokuon(quiz_options) {
        print_group("促音", SOKUON_TABLE);
    }

    if include_yoon(quiz_options) {
        print_group("拗音", YOON_TABLE);
        if include_dakuten(quiz_options) || include_handakuten(quiz_options) {
            print_group("拗音（浊/半浊）", EXTENDED_YOON_TABLE);
        }
    }
}

fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_owned());
    PathBuf::from(home).join(".keiko_stats.db")
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            ended_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            total INTEGER NOT NULL,
            correct INTEGER NOT NULL,
            incorrect INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS answers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id INTEGER NOT NULL,
            hira TEXT NOT NULL,
            roma TEXT NOT NULL,
            correct INTEGER NOT NULL,
            FOREIGN KEY(session_id) REFERENCES sessions(id)
        )",
        [],
    )?;
    Ok(())
}

fn run_quiz(conn: &Connection, quiz_options: QuizOptions) -> Result<(), String> {
    println!("开始练习：随机给出平假名或罗马音，请输入对应答案。");
    let feature_labels = selected_feature_labels(quiz_options);
    if !feature_labels.is_empty() {
        println!("已启用题型：{}", feature_labels.join("；"));
    }
    println!("{EXIT_HINT}");

    let mut rng = rand::rng();
    let mut total = 0_i64;
    let mut correct = 0_i64;
    let mut answers: Vec<(&KanaItem, bool)> = Vec::new();
    let pool = build_quiz_pool(quiz_options);

    loop {
        let item = pool
            .choose(&mut rng)
            .copied()
            .ok_or_else(|| "题库为空".to_string())?;
        let direction = if rand::random::<bool>() {
            Direction::HiraToRoma
        } else {
            Direction::RomaToHira
        };

        match direction {
            Direction::HiraToRoma => {
                print!("题目：{} -> ", item.hira);
            }
            Direction::RomaToHira => {
                print!("题目：{} -> ", item.roma);
            }
        }
        io::stdout()
            .flush()
            .map_err(|e| format!("刷新输出失败：{e}"))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("读取输入失败：{e}"))?;
        let answer = input.trim();

        // 优先检查退出命令，不计入统计
        if is_exit(answer) {
            println!("\n已退出练习。");
            break;
        }

        if answer.is_empty() {
            println!("请输入有效内容。");
            continue;
        }

        total += 1;
        let ok = match direction {
            Direction::HiraToRoma => answer.eq_ignore_ascii_case(item.roma),
            Direction::RomaToHira => answer == item.hira,
        };
        if ok {
            correct += 1;
            println!("{COLOR_GREEN}正确{COLOR_RESET}");
        } else {
            println!(
                "{COLOR_RED}错误，正确答案：{} / {}{COLOR_RESET}",
                item.hira, item.roma
            );
        }
        answers.push((item, ok));
    }

    let incorrect = total - correct;
    if total > 0 {
        let accuracy = correct as f64 / total as f64 * 100.0;
        let fail_rate = incorrect as f64 / total as f64 * 100.0;
        println!(
            "本轮结束：总题数 {total}，正确 {correct}，错误 {incorrect}，成功率 {accuracy:.2}%，失败率 {fail_rate:.2}%"
        );
        conn.execute(
            "INSERT INTO sessions (total, correct, incorrect) VALUES (?1, ?2, ?3)",
            params![total, correct, incorrect],
        )
        .map_err(|e| format!("保存数据失败：{e}"))?;

        // 保存每题的答案记录
        let mut stmt = conn
            .prepare(
                "INSERT INTO answers (session_id, hira, roma, correct) VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(|e| format!("准备语句失败：{e}"))?;
        let session_id = conn.last_insert_rowid();
        for (item, ok) in answers {
            stmt.execute(params![
                session_id,
                item.hira,
                item.roma,
                if ok { 1 } else { 0 }
            ])
            .map_err(|e| format!("保存答案失败：{e}"))?;
        }
    } else {
        println!("本轮未作答，不记录数据。");
    }

    Ok(())
}

fn show_stats(conn: &Connection) -> Result<(), String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                COUNT(*) AS sessions,
                COALESCE(SUM(total), 0) AS total,
                COALESCE(SUM(correct), 0) AS correct,
                COALESCE(SUM(incorrect), 0) AS incorrect
            FROM sessions",
        )
        .map_err(|e| format!("读取统计失败：{e}"))?;

    let (sessions, total, correct, incorrect): (i64, i64, i64, i64) = stmt
        .query_row([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| format!("读取汇总数据失败：{e}"))?;

    if total == 0 {
        println!("暂无统计数据，先运行 keiko 或 keiko quiz 开始练习；可用 keiko review 查看全表。");
        return Ok(());
    }

    let accuracy = correct as f64 / total as f64 * 100.0;
    let fail_rate = incorrect as f64 / total as f64 * 100.0;

    println!("累计会话：{sessions}");
    println!("累计题数：{total}");
    println!("累计正确：{correct}");
    println!("累计错误：{incorrect}");
    println!("累计成功率：{accuracy:.2}%");
    println!("累计失败率：{fail_rate:.2}%");

    let stats_map = load_answer_stats_map(conn)?;
    let category_stats = build_category_stats(&stats_map);
    if !category_stats.is_empty() {
        println!("\n类别汇总:");
        for (name, correct_count, total) in category_stats {
            let accuracy = correct_count as f64 / total as f64 * 100.0;
            let bar_len = (accuracy / 10.0) as usize;
            let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
            println!(
                "  {:<12} [{}] {:5.1}% ({}/{})",
                name, bar, accuracy, correct_count, total
            );
        }
    }

    println!("\n最近 5 次会话:");
    let mut recent = conn
        .prepare(
            "SELECT started_at, ended_at, total, correct, incorrect
             FROM sessions
             ORDER BY id DESC
             LIMIT 5",
        )
        .map_err(|e| format!("读取最近会话失败：{e}"))?;

    let rows = recent
        .query_map([], |row| {
            let started_at: String = row.get(0)?;
            let ended_at: String = row.get(1)?;
            let total: i64 = row.get(2)?;
            let correct: i64 = row.get(3)?;
            let incorrect: i64 = row.get(4)?;
            Ok((started_at, ended_at, total, correct, incorrect))
        })
        .map_err(|e| format!("映射最近会话失败：{e}"))?;

    for (idx, row) in rows.enumerate() {
        let (started_at, ended_at, total, correct, incorrect) =
            row.map_err(|e| format!("解析最近会话失败：{e}"))?;
        let accuracy = if total > 0 {
            correct as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        println!(
            "{}. {} ~ {} | 总 {} | 对 {} | 错 {} | 成功率 {:.2}%",
            idx + 1,
            started_at,
            ended_at,
            total,
            correct,
            incorrect,
            accuracy
        );
    }

    Ok(())
}

/// Shows detailed accuracy statistics for each kana item grouped by standard order.
fn show_detail(conn: &Connection) -> Result<(), String> {
    let stats_map = load_answer_stats_map(conn)?;
    if stats_map.is_empty() {
        println!("暂无详细数据，先运行 keiko 或 keiko quiz 开始练习。");
        return Ok(());
    }

    // Use the same layout-based grouping as print_kana_chart
    let groups = group_by_layout(KANA_TABLE, GROUP_LAYOUT);

    println!("类别汇总:\n");
    for (name, correct_count, total) in build_category_stats(&stats_map) {
        let accuracy = correct_count as f64 / total as f64 * 100.0;
        let bar_len = (accuracy / 10.0) as usize;
        let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
        println!(
            "  {:<12} [{}] {:5.1}% ({}/{})",
            name, bar, accuracy, correct_count, total
        );
    }

    println!("\n各字正确率统计:\n");
    for group in groups {
        let group_name = group.first().unwrap().roma.chars().next().unwrap();
        print_detail_group(&group_name.to_uppercase().to_string(), group, &stats_map);
    }

    print_detail_group("浊音", DAKUON_TABLE, &stats_map);
    print_detail_group("半浊音", HANDAKUON_TABLE, &stats_map);
    print_detail_group("促音", SOKUON_TABLE, &stats_map);
    print_detail_group("拗音", YOON_TABLE, &stats_map);
    print_detail_group("拗音（浊/半浊）", EXTENDED_YOON_TABLE, &stats_map);

    Ok(())
}

fn build_category_stats(stats_map: &AnswerStatsMap) -> Vec<(&'static str, i64, i64)> {
    KANA_CATEGORIES
        .iter()
        .filter_map(|category| {
            let mut correct_sum = 0_i64;
            let mut total_sum = 0_i64;

            for item in category.items {
                if let Some(&(correct_count, total)) =
                    stats_map.get(&(item.hira.to_string(), item.roma.to_string()))
                {
                    correct_sum += correct_count;
                    total_sum += total;
                }
            }

            if total_sum > 0 {
                Some((category.name, correct_sum, total_sum))
            } else {
                None
            }
        })
        .collect()
}

fn load_answer_stats_map(conn: &Connection) -> Result<AnswerStatsMap, String> {
    let mut stmt = conn
        .prepare(
            "SELECT hira, roma, SUM(correct) AS correct_count, COUNT(*) AS total
             FROM answers
             GROUP BY hira, roma
             ORDER BY ROWID",
        )
        .map_err(|e| format!("读取答案统计失败：{e}"))?;

    let rows = stmt
        .query_map([], |row| {
            let hira: String = row.get(0)?;
            let roma: String = row.get(1)?;
            let correct_count: i64 = row.get(2)?;
            let total: i64 = row.get(3)?;
            Ok(((hira, roma), (correct_count, total)))
        })
        .map_err(|e| format!("查询答案统计失败：{e}"))?;

    let mut stats_map = AnswerStatsMap::new();
    for row in rows {
        let ((hira, roma), stats) = row.map_err(|e| format!("解析答案统计失败：{e}"))?;
        stats_map.insert((hira, roma), stats);
    }

    Ok(stats_map)
}

fn print_detail_group(group_name: &str, items: &[KanaItem], stats_map: &AnswerStatsMap) {
    let mut printed = false;

    for item in items {
        if let Some(&(correct_count, total)) =
            stats_map.get(&(item.hira.to_string(), item.roma.to_string()))
        {
            if !printed {
                println!("[{group_name}]");
                printed = true;
            }
            let accuracy = correct_count as f64 / total as f64 * 100.0;
            let bar_len = (accuracy / 10.0) as usize;
            let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
            println!(
                "  {} ({:>5}): [{}] {:5.1}% ({}/{})",
                item.hira, item.roma, bar, accuracy, correct_count, total
            );
        }
    }

    if printed {
        println!();
    }
}

fn is_exit(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "q" | "quit" | "exit")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contains_item(items: &[&KanaItem], hira: &str, roma: &str) -> bool {
        items
            .iter()
            .any(|item| item.hira == hira && item.roma == roma)
    }

    fn build_stats_map(entries: &[(&str, &str, i64, i64)]) -> AnswerStatsMap {
        entries
            .iter()
            .map(|(hira, roma, correct_count, total)| {
                (
                    ((*hira).to_string(), (*roma).to_string()),
                    (*correct_count, *total),
                )
            })
            .collect()
    }

    #[test]
    fn default_pool_contains_only_base_kana() {
        let pool = build_quiz_pool(QuizOptions::default());

        assert_eq!(pool.len(), 46);
        assert!(contains_item(&pool, "あ", "a"));
        assert!(!contains_item(&pool, "が", "ga"));
        assert!(!contains_item(&pool, "っか", "kka"));
        assert!(!contains_item(&pool, "きゃ", "kya"));
    }

    #[test]
    fn split_flags_add_requested_groups() {
        let pool = build_quiz_pool(QuizOptions {
            include_sokuon: true,
            include_dakuten: true,
            include_handakuten: true,
            include_yoon: false,
            include_all: false,
        });

        assert!(contains_item(&pool, "が", "ga"));
        assert!(contains_item(&pool, "ぱ", "pa"));
        assert!(contains_item(&pool, "っか", "kka"));
        assert!(!contains_item(&pool, "きゃ", "kya"));
        assert!(!contains_item(&pool, "ぎゃ", "gya"));
    }

    #[test]
    fn yoon_pool_adds_base_yoon_only() {
        let pool = build_quiz_pool(QuizOptions {
            include_sokuon: false,
            include_dakuten: false,
            include_handakuten: false,
            include_yoon: true,
            include_all: false,
        });

        assert!(contains_item(&pool, "きゃ", "kya"));
        assert!(contains_item(&pool, "しゅ", "shu"));
        assert!(!contains_item(&pool, "が", "ga"));
        assert!(!contains_item(&pool, "ぎゃ", "gya"));
    }

    #[test]
    fn combined_pool_adds_extended_yoon_too() {
        let pool = build_quiz_pool(QuizOptions {
            include_sokuon: true,
            include_dakuten: true,
            include_handakuten: true,
            include_yoon: true,
            include_all: false,
        });

        assert!(contains_item(&pool, "が", "ga"));
        assert!(contains_item(&pool, "っち", "cchi"));
        assert!(contains_item(&pool, "きょ", "kyo"));
        assert!(contains_item(&pool, "ぎゃ", "gya"));
        assert!(contains_item(&pool, "ぴょ", "pyo"));
    }

    #[test]
    fn all_flag_enables_everything() {
        let pool = build_quiz_pool(QuizOptions {
            include_sokuon: false,
            include_dakuten: false,
            include_handakuten: false,
            include_yoon: false,
            include_all: true,
        });

        assert!(contains_item(&pool, "っち", "cchi"));
        assert!(contains_item(&pool, "が", "ga"));
        assert!(contains_item(&pool, "ぽ", "po"));
        assert!(contains_item(&pool, "きゅ", "kyu"));
        assert!(contains_item(&pool, "びょ", "byo"));
    }

    #[test]
    fn category_stats_are_grouped_by_kana_type() {
        let stats_map = build_stats_map(&[
            ("あ", "a", 2, 3),
            ("が", "ga", 1, 2),
            ("ぱ", "pa", 3, 4),
            ("っか", "kka", 1, 1),
            ("きゃ", "kya", 2, 2),
            ("ぎゃ", "gya", 1, 3),
        ]);

        assert_eq!(
            build_category_stats(&stats_map),
            vec![
                ("清音", 2, 3),
                ("浊音", 1, 2),
                ("半浊音", 3, 4),
                ("促音", 1, 1),
                ("拗音", 2, 2),
                ("拗音（浊/半浊）", 1, 3),
            ]
        );
    }
}
