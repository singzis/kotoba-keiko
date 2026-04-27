//! `keiko` 命令行练习器的核心库。
//!
//! 二进制入口刻意保持轻量：参数解析和命令分发留在 `main.rs`，
//! 题库、练习、持久化和报表逻辑统一收敛到这里。
//! 这样既能降低 CLI 本身的复杂度，也为后续继续扩展其它前端留出稳定边界。

pub mod kana;
pub mod model;
pub mod quiz;
pub mod report;
pub mod storage;

pub use model::{AppResult, QuizOptions};
