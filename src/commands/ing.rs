use anyhow::anyhow;
use clap::{Args, Subcommand, ValueEnum, builder::NonEmptyStringValueParser};
use serde::Serialize;
use serde_json::json;

use crate::commands::validate_non_zero_id;

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum IngType {
    #[clap(name = "all", help = "查询所有闪存内容")]
    All,
    /// following 简写查询关注的人的闪存
    #[clap(name = "fo")]
    Following,
    /// 查询@我的闪存
    // #[clap(name = "mention")]
    // Mention,
    /// 查询我的闪存
    // #[clap(name = "my")]
    My,
    /// 查询回复我的闪存
    // #[clap(name = "comment")]
    // Comment,
    /// recent comment 简写 查询最近的评论闪存
    #[clap(name = "rc")]
    RecentComment,
    /// recent comment 简写 查询我发布的评论闪存
    #[clap(name = "myc")]
    MyComment,
    /// 按标签查询闪存（需配合 --tag 参数）
    #[clap(name = "tag")]
    Tag,
}

impl IngType {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Following => "following",
            // Self::Mention => "mention",
            Self::My => "my",
            // Self::Comment => "comment",
            Self::RecentComment => "recentcomment",
            Self::MyComment => "mycomment",
            Self::Tag => "tag",
        }
    }
}

/// 闪存的发布，查询，删除，评论等操作
#[derive(Debug, Subcommand)]
pub enum IngAction {
    Create(IngContent),
    Delete(IngDelete),
    List(IngListArg),
    Replay(IngReplayContent),
    Show(IngShowDetail),
}

/// 浏览闪存，可结合type和option进一步筛选。
#[derive(Debug, Args, Serialize)]
pub struct IngListArg {
    /// 闪存查询类型（可选值：all/following/mention/my/comments/recent-comments/my-comments/tag）
    #[clap(value_enum, default_value_t = IngType::All)]
    #[serde(skip)]
    pub r#type: IngType,

    /// 分页页码（从1开始）
    #[arg(long = "page-index", default_value_t = 1)]
    #[serde(rename = "pageIndex")]
    pub page_index: u64,

    /// 每页显示的条数，默认20
    #[arg(long = "page-size", default_value_t = 20)]
    #[serde(rename = "pageSize")]
    pub page_size: u64,

    /// 按照标签名查询，注意 仅当type=tag时，标签名必须。其余可选。
    #[arg(
        long,
        required_if_eq("type", "tag"),
        value_parser = NonEmptyStringValueParser::new(),
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// no commnet，关闭评论，只显示闪存。
    #[arg(
        default_value = "false",
        long = "no",
        // help = "no-comment，用于控制是否显示评论。"
    )]
    #[serde(skip)]
    pub no_comment: bool,
}

/// 发布闪存，操作示例：
///
/// cnb ing create "Hello World"
///
/// cnb ing create --tag "lucky"
///
/// cnb ing create "Hello World" --tag lucky --tag cnb
#[derive(Debug, Args)]
pub struct IngContent {
    /// 闪存正文，闪存内容空时，tag为必须
    #[clap(value_parser = NonEmptyStringValueParser::new())]
    pub content: Option<String>,

    /// content是否公开，默认公开
    #[arg(long = "is-private", short = 'p')]
    pub is_private: bool,

    /// tag标签，可选，如lucky，python
    #[arg(short = 't',
        long = "tag",
        // required_if_eq("content", "None"),
        requires_if("false", "content"),
        value_parser = NonEmptyStringValueParser::new())]
    pub tag: Vec<String>,
}

impl IngContent {
    pub fn to_json(self) -> impl Serialize {
        let mut content = String::new();
        self.tag.iter().for_each(|x| {
            if x.contains("[") && x.contains("]") {
                content.push_str(x);
            } else {
                content.push_str(&format!("[{}]", x));
            }
        });

        if let Some(cc) = self.content {
            content.push_str(&cc);
        }

        json!({
            "content": content,
            "isPrivate": false,
            "clientType": 13,
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.content.is_none() && self.tag.is_empty() {
            return Err(anyhow!("当 content 为空时，tag 必须提供"));
        }
        Ok(())
    }
}

/// 根据提供的闪存id删除对应闪存
/// 也可以同时提供闪存id和commentid删除对应的评论
#[derive(Debug, Args)]
pub struct IngDelete {
    #[arg(
        required = true,
        // value_delimiter = ',',
        value_parser = validate_non_zero_id,
        help = "要删除的ID列表（用逗号分隔），不能包含0"
    )]
    pub id: u64,

    #[arg(long = "cid", help = "根据闪存id和commentid删除评论。", value_parser = validate_non_zero_id)]
    pub comment_id: Option<u64>,
}

/// 评论回复，to实现@功能，
#[derive(Debug, Args)]
pub struct IngReplayContent {
    #[clap(value_parser = NonEmptyStringValueParser::new())]
    pub content: String,

    #[arg(long, value_parser = validate_non_zero_id, required = true)]
    pub id: u64,
    // #[arg(long = "to", short = 't', value_parser = NonEmptyStringValueParser::new())]
    // pub to: Vec<String>,
}

/// 根据ID展示闪存
#[derive(Debug, Args)]
pub struct IngShowDetail {
    /// 闪存ID
    #[clap(value_parser =validate_non_zero_id)]
    pub id: u64,

    /// 是否显示评论
    #[arg(long, short = 'c', default_value_t = false)]
    pub comments: bool,
}
