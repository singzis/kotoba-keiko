use std::collections::HashMap;

/// 应用内部统一使用的返回结果类型。
///
/// 当前程序直接在终端面向最终用户展示错误，因此这里使用可读的 `String`
/// 来收敛错误信息，避免把存储层或解析层的细节暴露给上层调用方。
pub type AppResult<T> = Result<T, String>;

/// 控制题库范围的特性开关。
///
/// 这一组配置会同时被练习、对照表和统计视图复用，
/// 目的是让系统对“当前启用了哪些类别”只有一套解释。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QuizOptions {
    pub include_sokuon: bool,
    pub include_dakuten: bool,
    pub include_handakuten: bool,
    pub include_yoon: bool,
    pub include_all: bool,
}

impl QuizOptions {
    /// 返回是否应当纳入促音题目。
    pub fn includes_sokuon(self) -> bool {
        self.include_all || self.include_sokuon
    }

    /// 返回是否应当纳入浊音题目。
    pub fn includes_dakuten(self) -> bool {
        self.include_all || self.include_dakuten
    }

    /// 返回是否应当纳入半浊音题目。
    pub fn includes_handakuten(self) -> bool {
        self.include_all || self.include_handakuten
    }

    /// 返回是否应当纳入拗音题目。
    pub fn includes_yoon(self) -> bool {
        self.include_all || self.include_yoon
    }

    /// 返回是否应当纳入浊音/半浊音对应的拗音组合。
    pub fn includes_extended_yoon(self) -> bool {
        self.includes_yoon() && (self.includes_dakuten() || self.includes_handakuten())
    }

    /// 返回是否启用了任意基础清音之外的扩展类别。
    pub fn has_extra_categories(self) -> bool {
        self.includes_sokuon()
            || self.includes_dakuten()
            || self.includes_handakuten()
            || self.includes_yoon()
    }
}

/// 在练习、对照表和统计中复用的不可变假名条目。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KanaItem {
    pub hira: &'static str,
    pub roma: &'static str,
}

/// 用于分组统计输出的假名类别。
#[derive(Clone, Copy, Debug)]
pub struct KanaCategory {
    pub name: &'static str,
    pub items: &'static [KanaItem],
}

/// 以 `(平假名, 罗马音)` 为键的正确率统计映射。
pub type AnswerStatsMap = HashMap<(String, String), (i64, i64)>;

/// 一条已作答的练习记录。
#[derive(Clone, Copy, Debug)]
pub struct AnsweredKana {
    pub item: &'static KanaItem,
    pub is_correct: bool,
}

/// 可持久化的一轮练习摘要。
#[derive(Debug, Default)]
pub struct QuizSessionRecord {
    pub total: i64,
    pub correct: i64,
    pub answers: Vec<AnsweredKana>,
}

impl QuizSessionRecord {
    /// 返回本轮练习中的错误题数。
    pub fn incorrect(&self) -> i64 {
        self.total - self.correct
    }
}
