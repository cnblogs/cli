//!
//! 认证子命令
//!

use clap::{Args, Subcommand, builder::NonEmptyStringValueParser};

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub commands: UserAction,
}

/// 提供通过access token登录，状态查询，退出，显示当前token功能
#[derive(Debug, Subcommand)]
pub enum UserAction {
    Follower(FollowerArg),

    /// 用户登录，需提供access token。
    Login {
        #[clap(value_parser = NonEmptyStringValueParser::new())]
        token: String,
    },
    /// 用户退出
    Logout,
    /// 查看登录状态，登录后会显示用户信息
    Status,
    /// 显示当前登录token
    Token,
}

#[derive(serde::Serialize, Debug, Args)]
pub struct FollowerArg {
    /// 分页页码（从1开始）
    #[arg(long = "page-index", default_value_t = 1)]
    #[serde(rename = "pageIndex")]
    pub page_index: u64,

    /// 每页显示的条数，默认20
    #[arg(long = "page-size", default_value_t = 20)]
    #[serde(rename = "pageSize")]
    pub page_size: u64,
}
