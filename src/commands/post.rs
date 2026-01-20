use clap::{Args, Subcommand, builder::NonEmptyStringValueParser};
use serde::Serialize;

use crate::commands::validate_non_zero_id;

#[derive(Debug, Args)]
pub struct PostCommand {
    #[clap(subcommand)]
    pub commands: PostAction,
}

#[derive(Debug, Subcommand)]
pub enum PostAction {
    // Create,
    List(ListArgs),
    Replay(ReplayArgs),
    Show(ShowArgs),
    // Update,
}

/// 随笔列表，可根据博客名称获取。
#[derive(Debug, Args, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListArgs {
    /// 博客名称，api接口的blog_app，默认当前用户，也可以指定。
    #[arg(
        value_name = "Blog name",
        help = "博客名称，api接口的blog_app，默认当前用户，也可以指定。",
        value_parser = NonEmptyStringValueParser::new()
    )]
    #[serde(skip)]
    pub name: Option<String>,

    /// 分页页码（从1开始）
    #[arg(long = "page-index", default_value_t = 1)]
    pub page_index: u64,

    /// 每页显示的条数，默认20
    #[arg(long = "page-size", default_value_t = 10)]
    pub page_size: u64,
}

/// 展示随笔内容
#[derive(Debug, Args, Serialize)]
pub struct ShowArgs {
    /// 随笔ID，必传
    #[serde(skip)]
    #[clap(value_parser = validate_non_zero_id, required = true)]
    pub id: u64,

    /// 展示评论，默认关闭评论，选择显示评论，则不展示随笔内容
    ///
    /// 目前API调用存在问题，未实现此功能，占位。
    #[serde(skip)]
    #[arg(long, short = 'c', default_value_t = false)]
    pub comments: bool,

    /// 分页页码（从1开始）
    #[arg(
        default_value_t = 1,
        long = "page-index",
        required_if_eq("comments", "true"),
        value_parser = validate_non_zero_id)]
    pub page_index: u64,

    /// 每页显示的条数，默认20
    #[arg(
        long = "page-size",
        default_value_t = 20,
        required_if_eq("comments", "true"),
        value_parser = validate_non_zero_id
    )]
    pub page_size: u64,
}

/// 博文评论回复
#[derive(Debug, Args, Serialize)]
pub struct ReplayArgs {
    /// 评论内容，必传
    #[serde(rename = "body")]
    #[clap(value_parser = NonEmptyStringValueParser::new(), required = true)]
    pub content: String,

    /// 随笔ID，必传
    #[serde(skip)]
    #[arg(long, value_parser = validate_non_zero_id, required = true)]
    pub id: u64,

    #[serde(skip)]
    #[arg(
        long,
        value_name = "Blog name",
        help = "博客名称，api接口的blog_app，默认当前用户，也可以指定。",
        value_parser = NonEmptyStringValueParser::new()
    )]
    /// 回复博客园名称，不传为当前登录账号的博客名称。
    pub name: Option<String>,
}

/// 博文评论回复
#[derive(Debug, Args, Serialize)]
pub struct CreateArgs {}
