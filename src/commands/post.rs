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
    Create,
    List(ListArgs),
    Replay,
    Show(ShowArgs),
    Update,
}

/// 随笔列表，可根据博客名称获取。
#[derive(Debug, Args, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListArgs {
    /// 博客名称，api接口的blog_app，默认当前用户，也可以指定。
    #[arg(
        value_name = "Blog name",
        help = "博客名称/标识符（输入时不能为空字符串）",
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

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[clap(value_parser = validate_non_zero_id)]
    pub id: u64,
    
    #[arg(long, defalut_value_t = false)]
    pub comment: bool,
}
