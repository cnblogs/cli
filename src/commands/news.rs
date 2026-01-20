use clap::{Args, Subcommand};
use serde::Serialize;

#[derive(Debug, Args)]
pub struct NewsCommand {
    #[clap(subcommand)]
    pub commands: NewsAction,
}

#[derive(Debug, Subcommand)]
pub enum NewsAction {
    List(ListArgs),
}

#[derive(Debug, Args, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListArgs {
    /// 分页页码（从1开始）
    #[arg(long = "page-index", default_value_t = 1)]
    pub page_index: u64,

    /// 每页显示的条数，默认20
    #[arg(long = "page-size", default_value_t = 10)]
    pub page_size: u64,
}
