//!
//! cli操作逻辑
//!
//! 此模块暂定封装操作逻辑，比如是闪存的curd，闪存评论的curd。
//!

pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use anyhow::Result;

use crate::commands::{Cli, Commands};
use crate::context::Context;

pub async fn run(cli: Cli, ctx: &mut Context) -> Result<()> {
    match cli.commands {
        Commands::User(action) => {
            user::endpoint(action, ctx).await?;
        }
        Commands::Fav(action) => fav::endpoint(action, ctx).await?,
        Commands::Ing { action } => ing::endpoint(action, ctx).await?,
        Commands::News(action) => news::endpoint(action, ctx).await?,
        Commands::Post(action) => post::endpoint(action, ctx).await?,
    }

    Ok(())
}
