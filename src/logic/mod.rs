//! cli操作逻辑
//!
//! 此模块暂定封装操作逻辑，比如是闪存的curd，闪存评论的curd。
//!

use anyhow::Result;

use crate::{
    commands::{Cli, Commands},
};
use crate::context::Context;

pub mod ing;

pub async fn run(cli: Cli, ctx: &mut Context) -> Result<()> {
    match cli.commands {
        Commands::Auth(_a) => {}
        Commands::Fav => {}
        Commands::Ing { action } => ing::endpoint(action, ctx).await?,
        Commands::News => {}
        Commands::Post(_p) => {}
    }

    Ok(())
}
