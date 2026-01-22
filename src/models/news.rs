use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use serde::Deserialize;

use crate::tools::timer::DateFormatExt;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NewsInfo {
    pub id: u64,
    pub title: String,
    pub summary: String,
    pub topic_id: u64,
    pub topic_icon: Option<String>,
    pub view_count: u64,
    pub comment_count: u64,
    pub digg_count: u64,
    #[serde(with = "crate::tools::timer::rfc3339_or_naive")]
    pub date_added: DateTime<Utc>,
}

impl NewsInfo {
    pub fn into_format(self) -> String {
        format!(
            "{title}\n{summary}\n[#{id}][posted@ {date}][浏览：{vc}][评论：{cc}][点赞：{dc}]\n",
            title = self.title.bold().cyan(),
            summary = self.summary,
            id = self.id.bright_green(),
            date = self.date_added.as_time_age(),
            vc = self.view_count,
            cc = self.comment_count,
            dc = self.digg_count,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let a = r#"{
                "Id": 813264,
                "Title": "马斯克兑现承诺，开源X推荐算法！100% AI驱动，0人工规则",
                "Summary": "新智元报道 编辑：定慧 马斯克兑现承诺，X平台全新推荐算法正式开源！这套由 Grok 驱动的 AI 系统，完全取代了人工规则，通过 15 种行为预测精准计算每条帖子的命运。 1 月 11 日，马斯克在X平台上发了一条帖子，宣布将在 7 天内开源X平台全新的推荐算法。 他还承诺，此后每 4 周重复一次",
                "TopicId": 570,
                "TopicIcon": "https://images0.cnblogs.com/news_topic/20150702090016187.png",
                "ViewCount": 37,
                "CommentCount": 0,
                "DiggCount": 2,
                "DateAdded": "2026-01-20T15:40:00+08:00"
            }"#;

        let aa: NewsInfo = serde_json::from_str(a).unwrap();
        assert_eq!(aa.id, 813264);
    }
}
