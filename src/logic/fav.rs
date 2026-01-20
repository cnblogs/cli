use anyhow::Result;

use crate::{
    api,
    commands::fav::{FaverateAction, FaverateCommand, ListArgs},
    context::Context,
};

pub async fn endpoint(cmd: FaverateCommand, ctx: &mut Context) -> Result<()> {
    match cmd.commands {
        FaverateAction::List(arg) => handle_list(arg, ctx).await,
    }
}
async fn handle_list(arg: ListArgs, ctx: &mut Context) -> Result<()> {
    api::fav::list_bookmarks(&ctx.client, arg)
        .await?
        .into_iter()
        .for_each(|x| ctx.terminal.writeln(x.into_format()).unwrap());
    Ok(())
}
