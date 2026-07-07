use rusqlite::{Connection, OpenFlags};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

const INDEX_HTML: &str = include_str!("index.html");
const ADDR: &str = "127.0.0.1:7878";

fn main() {
    if let Err(err) = serve() {
        eprintln!("stats-viewer failed: {err}");
        std::process::exit(1);
    }
}

fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(ADDR)?;
    println!("stats-viewer: http://{ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(err) = handle_request(&mut stream) {
                    eprintln!("request failed: {err}");
                }
            }
            Err(err) => eprintln!("connection failed: {err}"),
        }
    }

    Ok(())
}

fn handle_request(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..size]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    match path {
        "/" | "/index.html" => {
            write_response(stream, "200 OK", "text/html; charset=utf-8", INDEX_HTML)
        }
        "/api/stats" => match render_stats_json() {
            Ok(json) => write_response(stream, "200 OK", "application/json; charset=utf-8", &json),
            Err(err) => write_response(
                stream,
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                &format!(r#"{{"error":"{}"}}"#, json_escape(&err.to_string())),
            ),
        },
        _ => write_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found",
        ),
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.as_bytes().len()
    )?;
    Ok(())
}

fn render_stats_json() -> Result<String, Box<dyn std::error::Error>> {
    let path = db_path();
    let conn = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    Ok(format!(
        r#"{{"dbPathRule":"$HOME/.keiko_stats.db","resolvedDbPath":"{}","sessions":{},"kanaStats":{},"latestWrong":{},"mismatchedSessions":{}}}"#,
        json_escape(&path.display().to_string()),
        query_sessions(&conn)?,
        query_kana_stats(&conn)?,
        query_latest_wrong(&conn)?,
        query_mismatched_sessions(&conn)?,
    ))
}

fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_owned());
    PathBuf::from(home).join(".keiko_stats.db")
}

fn query_sessions(conn: &Connection) -> rusqlite::Result<String> {
    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, total, correct, incorrect
         FROM sessions
         ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(format!(
            r#"{{"id":{},"started_at":"{}","ended_at":"{}","total":{},"correct":{},"incorrect":{}}}"#,
            row.get::<_, i64>(0)?,
            json_escape(&row.get::<_, String>(1)?),
            json_escape(&row.get::<_, String>(2)?),
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, i64>(5)?,
        ))
    })?;
    collect_json_array(rows)
}

fn query_kana_stats(conn: &Connection) -> rusqlite::Result<String> {
    let mut stmt = conn.prepare(
        "SELECT hira, roma, COUNT(*) AS attempts, SUM(correct) AS correct_count,
                COUNT(*) - SUM(correct) AS wrong_count,
                ROUND(SUM(correct) * 100.0 / COUNT(*), 2) AS accuracy_pct
         FROM answers
         GROUP BY hira, roma
         ORDER BY wrong_count DESC, attempts DESC, hira",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(format!(
            r#"{{"hira":"{}","roma":"{}","attempts":{},"correct":{},"wrong":{},"accuracy_pct":{:.2}}}"#,
            json_escape(&row.get::<_, String>(0)?),
            json_escape(&row.get::<_, String>(1)?),
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, f64>(5)?,
        ))
    })?;
    collect_json_array(rows)
}

fn query_latest_wrong(conn: &Connection) -> rusqlite::Result<String> {
    let mut stmt = conn.prepare(
        "SELECT hira, roma
         FROM answers
         WHERE session_id = (SELECT MAX(id) FROM sessions) AND correct = 0
         ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(format!(
            r#"{{"hira":"{}","roma":"{}"}}"#,
            json_escape(&row.get::<_, String>(0)?),
            json_escape(&row.get::<_, String>(1)?),
        ))
    })?;
    collect_json_array(rows)
}

fn query_mismatched_sessions(conn: &Connection) -> rusqlite::Result<String> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.total, COUNT(a.id) AS answer_rows
         FROM sessions s
         LEFT JOIN answers a ON a.session_id = s.id
         GROUP BY s.id
         HAVING s.total != COUNT(a.id)
         ORDER BY s.id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(format!(
            r#"{{"id":{},"total":{},"answer_rows":{}}}"#,
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    collect_json_array(rows)
}

fn collect_json_array(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<String>>,
) -> rusqlite::Result<String> {
    let mut values = Vec::new();
    for row in rows {
        values.push(row?);
    }
    Ok(format!("[{}]", values.join(",")))
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}
