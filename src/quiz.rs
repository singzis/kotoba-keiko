use crate::kana::{build_quiz_pool, selected_feature_labels};
use crate::model::{AnsweredKana, AppResult, QuizOptions, QuizSessionRecord};
use crate::storage::save_quiz_session;
use rand::prelude::IndexedRandom;
use rusqlite::Connection;
use std::io::{self, Write};

const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";
const EXIT_HINT: &str = "输入 q / quit / exit 可退出练习";

#[derive(Clone, Copy)]
enum Direction {
    HiraToRoma,
    RomaToHira,
}

/// 执行一轮交互式练习，并在需要时持久化结果。
///
/// 终端交互循环完全由这个函数接管，
/// 上层只负责流程编排，不需要理解逐题记账细节。
pub fn run_quiz(conn: &Connection, options: QuizOptions) -> AppResult<()> {
    println!("开始练习：随机给出平假名或罗马音，请输入对应答案。");
    let feature_labels = selected_feature_labels(options);
    if !feature_labels.is_empty() {
        println!("已启用题型：{}", feature_labels.join("；"));
    }
    println!("{EXIT_HINT}");

    let mut rng = rand::rng();
    let pool = build_quiz_pool(options);
    let mut session = QuizSessionRecord::default();

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
            Direction::HiraToRoma => print!("题目：{} -> ", item.hira),
            Direction::RomaToHira => print!("题目：{} -> ", item.roma),
        }
        io::stdout()
            .flush()
            .map_err(|e| format!("刷新输出失败：{e}"))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("读取输入失败：{e}"))?;
        let answer = input.trim();

        if is_exit(answer) {
            println!("\n已退出练习。");
            break;
        }

        if answer.is_empty() {
            println!("请输入有效内容。");
            continue;
        }

        session.total += 1;
        let is_correct = match direction {
            Direction::HiraToRoma => answer.eq_ignore_ascii_case(item.roma),
            Direction::RomaToHira => answer == item.hira,
        };

        if is_correct {
            session.correct += 1;
            println!("{COLOR_GREEN}正确{COLOR_RESET}");
        } else {
            println!(
                "{COLOR_RED}错误，正确答案：{} / {}{COLOR_RESET}",
                item.hira, item.roma
            );
        }

        session.answers.push(AnsweredKana { item, is_correct });
    }

    if session.total > 0 {
        let incorrect = session.incorrect();
        let accuracy = session.correct as f64 / session.total as f64 * 100.0;
        let fail_rate = incorrect as f64 / session.total as f64 * 100.0;
        println!(
            "本轮结束：总题数 {}，正确 {}，错误 {}，成功率 {:.2}%，失败率 {:.2}%",
            session.total, session.correct, incorrect, accuracy, fail_rate
        );
        save_quiz_session(conn, &session)?;
    } else {
        println!("本轮未作答，不记录数据。");
    }

    Ok(())
}

fn is_exit(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "q" | "quit" | "exit")
}
