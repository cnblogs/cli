//!
//! 认证子命令
//!

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Authenticate {
    #[clap(subcommand)]
    pub commands: AuthenticateSubCommands,
}

/// 认证子命令
///
///
#[derive(Debug, Subcommand)]
pub enum AuthenticateSubCommands {
    Login { token: String },
    Logout,
    Status,
    Token,
}
