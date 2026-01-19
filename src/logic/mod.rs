//!
//! cli操作逻辑
//!
//! 此模块暂定封装操作逻辑，比如是闪存的curd，闪存评论的curd。
//!

pub mod auth;
pub mod ing;
pub mod post;

use anyhow::Result;

use crate::commands::{Cli, Commands};
use crate::context::Context;

pub async fn run(cli: Cli, ctx: &mut Context) -> Result<()> {
    match cli.commands {
        Commands::Auth(action) => {
            auth::endpoint(action, ctx).await?;
        }
        Commands::Fav => {}
        Commands::Ing { action } => ing::endpoint(action, ctx).await?,
        Commands::News => {}
        Commands::Post(action) => post::endpoint(action, ctx).await?,
    }

    Ok(())
}
