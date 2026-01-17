//!
//! 认证子命令
//!

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Authenticate {
    #[clap(subcommand)]
    pub commands: AuthenticateSubCommands,
}

/// 提供通过access token登录，状态查询，退出，显示当前token功能
#[derive(Debug, Subcommand)]
pub enum AuthenticateSubCommands {
    /// 用户登录，需提供access token。
    Login { token: String },
    /// 用户退出
    Logout,
    /// 查看登录状态，登录后会显示用户信息
    Status,
    /// 显示当前登录token
    Token,
}
