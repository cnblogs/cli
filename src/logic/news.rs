use anyhow::Result;

use crate::{
    api,
    commands::news::{ListArgs, NewsAction, NewsCommand},
    context::Context,
};

pub async fn endpoint(cmd: NewsCommand, ctx: &mut Context) -> Result<()> {
    match cmd.commands {
        NewsAction::List(arg) => handle_list(arg, ctx).await,
    }
}

async fn handle_list(arg: ListArgs, ctx: &mut Context) -> Result<()> {
    api::news::list_news(&ctx.client, arg)
        .await?
        .into_iter()
        .for_each(|x| ctx.terminal.writeln(x.into_format()).unwrap());
    Ok(())
}
