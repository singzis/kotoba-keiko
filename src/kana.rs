use crate::model::{KanaCategory, KanaItem, QuizOptions};

/// 基础平假名行分组布局。
///
/// 这些数字按五十音顺序描述每一行应当打印多少个元素。
pub const GROUP_LAYOUT: &[usize] = &[5, 5, 5, 5, 5, 5, 5, 3, 5, 2, 1];

/// 默认练习模式使用的基础平假名表。
pub const KANA_TABLE: &[KanaItem] = &[
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

/// 浊音条目。
pub const DAKUON_TABLE: &[KanaItem] = &[
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

/// 半浊音条目。
pub const HANDAKUON_TABLE: &[KanaItem] = &[
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

/// 用于单独训练的促音组合。
pub const SOKUON_TABLE: &[KanaItem] = &[
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

/// 基础拗音组合。
pub const YOON_TABLE: &[KanaItem] = &[
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

/// 依赖浊音或半浊音开关的扩展拗音组合。
pub const EXTENDED_YOON_TABLE: &[KanaItem] = &[
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

/// `stats` 与 `detail` 共用的统计类别定义。
pub const KANA_CATEGORIES: &[KanaCategory] = &[
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

/// 按固定布局把一维切片拆成连续分组。
///
/// 这个辅助函数的意义在于让对照表和详情视图共用同一套分组规则，
/// 避免行边界散落在多个位置各写一遍。
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

/// 根据练习选项构造最终题池。
///
/// 这里刻意把接口做窄：调用方只关心“我要一个完整题池”，
/// 不需要知道底层到底拼接了哪些子表。
pub fn build_quiz_pool(options: QuizOptions) -> Vec<&'static KanaItem> {
    let mut pool: Vec<&KanaItem> = KANA_TABLE.iter().collect();

    if options.includes_dakuten() {
        pool.extend(DAKUON_TABLE.iter());
    }
    if options.includes_handakuten() {
        pool.extend(HANDAKUON_TABLE.iter());
    }
    if options.includes_sokuon() {
        pool.extend(SOKUON_TABLE.iter());
    }
    if options.includes_yoon() {
        pool.extend(YOON_TABLE.iter());
    }
    if options.includes_extended_yoon() {
        pool.extend(EXTENDED_YOON_TABLE.iter());
    }

    pool
}

/// 生成当前启用题型的人类可读标签。
pub fn selected_feature_labels(options: QuizOptions) -> Vec<&'static str> {
    let mut labels = Vec::new();

    if options.includes_sokuon() {
        labels.push("促音");
    }
    if options.includes_dakuten() {
        labels.push("浊音");
    }
    if options.includes_handakuten() {
        labels.push("半浊音");
    }
    if options.includes_yoon() {
        labels.push(if options.includes_extended_yoon() {
            "拗音（含浊/半浊拗音）"
        } else {
            "拗音"
        });
    }

    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contains_item(items: &[&KanaItem], hira: &str, roma: &str) -> bool {
        items
            .iter()
            .any(|item| item.hira == hira && item.roma == roma)
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
}
