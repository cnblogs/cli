use anyhow::Result;
use owo_colors::OwoColorize;
use termimad::MadSkin;

use crate::{
    api,
    commands::post::{ListArgs, PostAction, PostCommand, ReplayArgs, ShowArgs},
    context::Context,
};

pub async fn endpoint(cmd: PostCommand, ctx: &mut Context) -> anyhow::Result<()> {
    match cmd.commands {
        // PostAction::Create => handle(ctx).await,
        PostAction::List(arg) => handle_list(arg, ctx).await,
        PostAction::Replay(arg) => handle_replay(arg, ctx).await,
        PostAction::Show(arg) => handle_show(arg, ctx).await,
        // PostAction::Update => handle(ctx).await,
    }
}

async fn handle_list(arg: ListArgs, ctx: &mut Context) -> Result<()> {
    let name = if let Some(n) = &arg.name {
        n
    } else {
        &ctx.cache.blog_app
    };

    let post = api::post::list_someone_post(&ctx.client, name, &arg).await?;
    for i in post {
        ctx.terminal.writeln(i.into_format())?;
    }
    Ok(())
}

async fn handle_replay(arg: ReplayArgs, ctx: &mut Context) -> Result<()> {
    let name = if let Some(n) = &arg.name {
        n
    } else {
        &ctx.cache.blog_app
    };
    let resp = api::post::raw_create_comment(&ctx.client, name, arg.id, &arg).await?;

    if resp.status().is_success() {
        ctx.terminal
            .writeln(format!("[Ok] {}", resp.status().bright_green()))
    } else {
        ctx.terminal.writeln(format!(
            "[Err] {}: {}",
            resp.status().red(),
            resp.text().await?.red()
        ))
    }
    // Ok(())
}

async fn handle_show(arg: ShowArgs, ctx: &mut Context) -> Result<()> {
    let resp = api::post::raw_show_post(&ctx.client, arg.id)
        .await?
        .text()
        .await?;

    // TODO: 优化
    // 处理快平台
    let resp: String = serde_json::from_str(&resp)?;
    let md = html2md::parse_html(&resp);
    let mds = MadSkin::default_light();
    mds.write_text_on(&mut ctx.terminal.stdout, &md)?;
    Ok(())
}

/// API 调用401,目前无法使用。
#[allow(unused)]
async fn handle_comments(arg: ShowArgs, ctx: &mut Context) -> Result<()> {
    let resp = api::post::raw_list_comments(&ctx.client, &ctx.cache.blog_app, arg.id, arg).await?;
    ctx.terminal.writeln(resp.status())?;
    ctx.terminal.writeln(resp.text().await?)
}
