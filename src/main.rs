use clap::{Args, Parser, Subcommand};
use kotoba_keiko::{AppResult, QuizOptions, quiz, report, storage};

#[derive(Parser)]
#[command(
    name = "keiko",
    version,
    about = "平假名与罗马音双向练习器（含 SQLite 统计）"
)]
struct Cli {
    #[command(flatten)]
    quiz_options: CliQuizOptions,
    #[command(subcommand)]
    command: Option<Commands>,
}

/// 仅供 CLI 使用的参数解析结构。
///
/// 核心库使用纯粹的 `QuizOptions`，这样未来即使不是 clap 驱动的前端，
/// 也可以复用同一套业务配置结构。
#[derive(Args, Clone, Copy, Debug, Default, Eq, PartialEq)]
struct CliQuizOptions {
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

impl From<CliQuizOptions> for QuizOptions {
    fn from(value: CliQuizOptions) -> Self {
        Self {
            include_sokuon: value.include_sokuon,
            include_dakuten: value.include_dakuten,
            include_handakuten: value.include_handakuten,
            include_yoon: value.include_yoon,
            include_all: value.include_all,
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("运行失败：{err}");
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Commands::Quiz);
    let options = QuizOptions::from(cli.quiz_options);
    validate_quiz_options(&command, options)?;

    match command {
        Commands::Quiz => {
            let conn = storage::open_db()?;
            quiz::run_quiz(&conn, options)
        }
        Commands::Stats => {
            let conn = storage::open_db()?;
            print!("{}", report::render_stats(&conn)?);
            Ok(())
        }
        Commands::Review => {
            print!("{}", report::render_kana_chart(options));
            Ok(())
        }
        Commands::Reset => storage::reset_db(),
        Commands::Detail => {
            let conn = storage::open_db()?;
            print!("{}", report::render_detail(&conn)?);
            Ok(())
        }
    }
}

/// 限制题型参数只能用于真正消费题库类别的命令。
///
/// 这样可以把命令级别的约束留在 CLI 边界，
/// 避免这类知识扩散进核心库内部。
fn validate_quiz_options(command: &Commands, options: QuizOptions) -> AppResult<()> {
    if options.has_extra_categories() && !matches!(command, Commands::Quiz | Commands::Review) {
        return Err(
            "`--sokuon`、`--dakuten`、`--handakuten`、`--yoon`、`--all` 仅可与 `quiz` 或 `review` 一起使用"
                .to_string(),
        );
    }

    Ok(())
}
