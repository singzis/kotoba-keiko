use crate::model::{AnswerStatsMap, AppResult, QuizSessionRecord};
use rusqlite::{Connection, params};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// 打开应用数据库并确保表结构存在。
///
/// 调用方不需要关心数据库文件位于哪里、如何初始化，
/// 只需要拿到一个可直接使用的连接。
pub fn open_db() -> AppResult<Connection> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| format!("无法打开数据库：{e}"))?;
    init_db(&conn).map_err(|e| format!("初始化数据库失败：{e}"))?;
    Ok(conn)
}

/// 在两次明确确认后重置本地统计数据库。
///
/// 双重确认逻辑保留在存储层内部，
/// 这样调用方不会因为遗漏保护步骤而误做破坏性操作。
pub fn reset_db() -> AppResult<()> {
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

/// 持久化一轮已完成练习及其逐题作答记录。
pub fn save_quiz_session(conn: &Connection, record: &QuizSessionRecord) -> AppResult<()> {
    conn.execute(
        "INSERT INTO sessions (total, correct, incorrect) VALUES (?1, ?2, ?3)",
        params![record.total, record.correct, record.incorrect()],
    )
    .map_err(|e| format!("保存数据失败：{e}"))?;

    let session_id = conn.last_insert_rowid();
    let mut stmt = conn
        .prepare("INSERT INTO answers (session_id, hira, roma, correct) VALUES (?1, ?2, ?3, ?4)")
        .map_err(|e| format!("准备语句失败：{e}"))?;

    for answer in &record.answers {
        stmt.execute(params![
            session_id,
            answer.item.hira,
            answer.item.roma,
            if answer.is_correct { 1 } else { 0 }
        ])
        .map_err(|e| format!("保存答案失败：{e}"))?;
    }

    Ok(())
}

/// 读取每个假名条目的聚合正确率统计。
///
/// 这里直接返回映射结构，是为了让报表层聚焦在展示逻辑，
/// 而不是在多个地方重复拼接 SQL 结果。
pub fn load_answer_stats_map(conn: &Connection) -> AppResult<AnswerStatsMap> {
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
