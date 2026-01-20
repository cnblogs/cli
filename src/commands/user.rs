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
