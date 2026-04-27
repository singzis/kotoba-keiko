use crate::kana::{
    DAKUON_TABLE, EXTENDED_YOON_TABLE, GROUP_LAYOUT, HANDAKUON_TABLE, KANA_CATEGORIES, KANA_TABLE,
    SOKUON_TABLE, YOON_TABLE, group_by_layout,
};
use crate::model::{AnswerStatsMap, AppResult, KanaItem, QuizOptions};
use crate::storage::load_answer_stats_map;
use rusqlite::Connection;
use std::fmt::Write as _;

/// 渲染当前配置下的假名对照表。
///
/// 对照表属于展示职责，因此由本模块负责分组和格式化，
/// 而 `kana` 模块只提供底层数据。
pub fn render_kana_chart(options: QuizOptions) -> String {
    let mut output = String::new();
    writeln!(output, "平假名 ↔ 罗马音（练习用全表）").unwrap();
    writeln!(output, "{}", "—".repeat(48)).unwrap();

    let groups = group_by_layout(KANA_TABLE, GROUP_LAYOUT);
    for group in groups {
        let group_name = group.first().unwrap().roma.chars().next().unwrap();
        write_group(&mut output, &group_name.to_uppercase().to_string(), group);
    }

    if options.includes_dakuten() {
        write_group(&mut output, "浊音", DAKUON_TABLE);
    }
    if options.includes_handakuten() {
        write_group(&mut output, "半浊音", HANDAKUON_TABLE);
    }
    if options.includes_sokuon() {
        write_group(&mut output, "促音", SOKUON_TABLE);
    }
    if options.includes_yoon() {
        write_group(&mut output, "拗音", YOON_TABLE);
    }
    if options.includes_extended_yoon() {
        write_group(&mut output, "拗音（浊/半浊）", EXTENDED_YOON_TABLE);
    }

    output
}

/// 渲染全局练习统计以及类别级正确率。
pub fn render_stats(conn: &Connection) -> AppResult<String> {
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
        return Ok(
            "暂无统计数据，先运行 keiko 或 keiko quiz 开始练习；可用 keiko review 查看全表。\n"
                .to_string(),
        );
    }

    let mut output = String::new();
    let accuracy = correct as f64 / total as f64 * 100.0;
    let fail_rate = incorrect as f64 / total as f64 * 100.0;

    writeln!(output, "累计会话：{sessions}").unwrap();
    writeln!(output, "累计题数：{total}").unwrap();
    writeln!(output, "累计正确：{correct}").unwrap();
    writeln!(output, "累计错误：{incorrect}").unwrap();
    writeln!(output, "累计成功率：{accuracy:.2}%").unwrap();
    writeln!(output, "累计失败率：{fail_rate:.2}%").unwrap();

    let stats_map = load_answer_stats_map(conn)?;
    let category_stats = build_category_stats(&stats_map);
    if !category_stats.is_empty() {
        writeln!(output, "\n类别汇总:").unwrap();
        for (name, correct_count, total) in category_stats {
            let accuracy = correct_count as f64 / total as f64 * 100.0;
            let bar_len = (accuracy / 10.0) as usize;
            let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
            writeln!(
                output,
                "  {:<12} [{}] {:5.1}% ({}/{})",
                name, bar, accuracy, correct_count, total
            )
            .unwrap();
        }
    }

    writeln!(output, "\n最近 5 次会话:").unwrap();
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
        writeln!(
            output,
            "{}. {} ~ {} | 总 {} | 对 {} | 错 {} | 成功率 {:.2}%",
            idx + 1,
            started_at,
            ended_at,
            total,
            correct,
            incorrect,
            accuracy
        )
        .unwrap();
    }

    Ok(output)
}

/// 渲染类别级与逐字级的正确率详情。
pub fn render_detail(conn: &Connection) -> AppResult<String> {
    let stats_map = load_answer_stats_map(conn)?;
    if stats_map.is_empty() {
        return Ok("暂无详细数据，先运行 keiko 或 keiko quiz 开始练习。\n".to_string());
    }

    let mut output = String::new();
    writeln!(output, "类别汇总:\n").unwrap();
    for (name, correct_count, total) in build_category_stats(&stats_map) {
        let accuracy = correct_count as f64 / total as f64 * 100.0;
        let bar_len = (accuracy / 10.0) as usize;
        let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
        writeln!(
            output,
            "  {:<12} [{}] {:5.1}% ({}/{})",
            name, bar, accuracy, correct_count, total
        )
        .unwrap();
    }

    writeln!(output, "\n各字正确率统计:\n").unwrap();
    for group in group_by_layout(KANA_TABLE, GROUP_LAYOUT) {
        let group_name = group.first().unwrap().roma.chars().next().unwrap();
        write_detail_group(
            &mut output,
            &group_name.to_uppercase().to_string(),
            group,
            &stats_map,
        );
    }

    write_detail_group(&mut output, "浊音", DAKUON_TABLE, &stats_map);
    write_detail_group(&mut output, "半浊音", HANDAKUON_TABLE, &stats_map);
    write_detail_group(&mut output, "促音", SOKUON_TABLE, &stats_map);
    write_detail_group(&mut output, "拗音", YOON_TABLE, &stats_map);
    write_detail_group(
        &mut output,
        "拗音（浊/半浊）",
        EXTENDED_YOON_TABLE,
        &stats_map,
    );

    Ok(output)
}

/// 把逐字统计聚合到较粗粒度的类别上。
///
/// 这个辅助函数把类别语义收敛到一个地方，
/// 这样以后新增题型时，`stats` 和 `detail` 不会各自偏离。
pub(crate) fn build_category_stats(stats_map: &AnswerStatsMap) -> Vec<(&'static str, i64, i64)> {
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

fn write_group(output: &mut String, name: &str, items: &[KanaItem]) {
    writeln!(output, "\n【{name}】").unwrap();
    for item in items {
        write!(output, "{:>3} ({:<5}) ", item.hira, item.roma).unwrap();
    }
    writeln!(output).unwrap();
}

fn write_detail_group(
    output: &mut String,
    group_name: &str,
    items: &[KanaItem],
    stats_map: &AnswerStatsMap,
) {
    let mut printed = false;

    for item in items {
        if let Some(&(correct_count, total)) =
            stats_map.get(&(item.hira.to_string(), item.roma.to_string()))
        {
            if !printed {
                writeln!(output, "[{group_name}]").unwrap();
                printed = true;
            }

            let accuracy = correct_count as f64 / total as f64 * 100.0;
            let bar_len = (accuracy / 10.0) as usize;
            let bar = "#".repeat(bar_len) + &"-".repeat(10 - bar_len);
            writeln!(
                output,
                "  {} ({:>5}): [{}] {:5.1}% ({}/{})",
                item.hira, item.roma, bar, accuracy, correct_count, total
            )
            .unwrap();
        }
    }

    if printed {
        writeln!(output).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AnswerStatsMap;

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
