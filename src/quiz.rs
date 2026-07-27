use crate::kana::{build_quiz_pool, selected_feature_labels};
use crate::model::{AnsweredKana, AppResult, PromptMode, QuizOptions, QuizSessionRecord};
use crate::storage::save_quiz_session;
use rand::prelude::IndexedRandom;
use rusqlite::Connection;
use std::io::{self, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";
const CTRL_C_EXIT_HINT: &str = "Press Ctrl-C again to exit";
const DOUBLE_CTRL_C_WINDOW: Duration = Duration::from_millis(1_500);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    KanaToRomaji,
    RomajiToKana,
}

enum InputEvent {
    Line(String),
    Eof,
    Error(io::Error),
    CtrlC,
}

/// 执行一轮交互式练习，并在需要时持久化结果。
///
/// 终端交互循环完全由这个函数接管，
/// 上层只负责流程编排，不需要理解逐题记账细节。
pub fn run_quiz(conn: &Connection, options: QuizOptions, prompt_mode: PromptMode) -> AppResult<()> {
    let input_events = install_input_events()?;

    let script_name = if options.uses_katakana() {
        "片假名"
    } else {
        "平假名"
    };
    match prompt_mode {
        PromptMode::Random => {
            println!("开始练习：随机给出{script_name}或罗马音，请输入对应答案。");
        }
        PromptMode::KanaOnly => {
            println!("开始练习：仅给出{script_name}，请输入对应的罗马音。");
        }
        PromptMode::RomajiOnly => {
            println!("开始练习：仅给出罗马音，请输入对应的{script_name}。");
        }
    }
    let feature_labels = selected_feature_labels(options);
    if !feature_labels.is_empty() {
        println!("已启用题型：{}", feature_labels.join("；"));
    }

    let mut rng = rand::rng();
    let pool = build_quiz_pool(options);
    let mut session = QuizSessionRecord::default();

    loop {
        let item = pool
            .choose(&mut rng)
            .copied()
            .ok_or_else(|| "题库为空".to_string())?;
        let direction = choose_direction(prompt_mode, rand::random());

        let prompt = match direction {
            Direction::KanaToRomaji => format!("题目：{} -> ", item.hira),
            Direction::RomajiToKana => format!("题目：{} -> ", item.roma),
        };
        print!("{prompt}");
        io::stdout()
            .flush()
            .map_err(|e| format!("刷新输出失败：{e}"))?;

        let Some(input) = read_quiz_input(&input_events, &prompt)? else {
            println!("\n已退出练习。");
            break;
        };
        let answer = input.trim();

        if answer.is_empty() {
            println!("请输入有效内容。");
            continue;
        }

        session.total += 1;
        let is_correct = match direction {
            Direction::KanaToRomaji => answer.eq_ignore_ascii_case(item.roma),
            Direction::RomajiToKana => answer == item.hira,
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

fn install_input_events() -> AppResult<Receiver<InputEvent>> {
    let (sender, receiver) = mpsc::channel();
    let ctrl_c_sender = sender.clone();
    ctrlc::set_handler(move || {
        let _ = ctrl_c_sender.send(InputEvent::CtrlC);
    })
    .map_err(|e| format!("注册 Ctrl-C 处理器失败：{e}"))?;

    std::thread::spawn(move || {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    let _ = sender.send(InputEvent::Eof);
                    break;
                }
                Ok(_) => {
                    if sender.send(InputEvent::Line(input)).is_err() {
                        break;
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => {
                    let _ = sender.send(InputEvent::Error(error));
                    break;
                }
            }
        }
    });

    Ok(receiver)
}

fn read_quiz_input(events: &Receiver<InputEvent>, prompt: &str) -> AppResult<Option<String>> {
    loop {
        let event = events
            .recv()
            .map_err(|_| "输入事件通道已关闭".to_string())?;

        match event {
            InputEvent::Line(input) => return Ok(Some(input)),
            InputEvent::Eof => return Ok(None),
            InputEvent::Error(error) => return Err(format!("读取输入失败：{error}")),
            InputEvent::CtrlC => {
                println!("\n{CTRL_C_EXIT_HINT}");
                io::stdout()
                    .flush()
                    .map_err(|e| format!("刷新输出失败：{e}"))?;

                match events.recv_timeout(DOUBLE_CTRL_C_WINDOW) {
                    Ok(InputEvent::CtrlC) => std::process::exit(130),
                    Ok(InputEvent::Line(input)) => return Ok(Some(input)),
                    Ok(InputEvent::Eof) => return Ok(None),
                    Ok(InputEvent::Error(error)) => {
                        return Err(format!("读取输入失败：{error}"));
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        print!("{prompt}");
                        io::stdout()
                            .flush()
                            .map_err(|e| format!("刷新输出失败：{e}"))?;
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        return Err("输入事件通道已关闭".to_string());
                    }
                }
            }
        }
    }
}

fn choose_direction(prompt_mode: PromptMode, random_kana_prompt: bool) -> Direction {
    match prompt_mode {
        PromptMode::KanaOnly => Direction::KanaToRomaji,
        PromptMode::RomajiOnly => Direction::RomajiToKana,
        PromptMode::Random if random_kana_prompt => Direction::KanaToRomaji,
        PromptMode::Random => Direction::RomajiToKana,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kana_only_always_uses_kana_prompts() {
        assert_eq!(
            choose_direction(PromptMode::KanaOnly, false),
            Direction::KanaToRomaji
        );
        assert_eq!(
            choose_direction(PromptMode::KanaOnly, true),
            Direction::KanaToRomaji
        );
    }

    #[test]
    fn romaji_only_always_uses_romaji_prompts() {
        assert_eq!(
            choose_direction(PromptMode::RomajiOnly, false),
            Direction::RomajiToKana
        );
        assert_eq!(
            choose_direction(PromptMode::RomajiOnly, true),
            Direction::RomajiToKana
        );
    }

    #[test]
    fn random_mode_follows_random_direction() {
        assert_eq!(
            choose_direction(PromptMode::Random, true),
            Direction::KanaToRomaji
        );
        assert_eq!(
            choose_direction(PromptMode::Random, false),
            Direction::RomajiToKana
        );
    }
}
