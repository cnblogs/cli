//!
//! 认证模块
//!

use anyhow::Result;
use owo_colors::OwoColorize;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use reqwest::{ClientBuilder, StatusCode};

use crate::commands::user::{FollowArg, UserAction, UserCommand};
use crate::context::Context;
use crate::context::config::Cache;
use crate::tools::http::IntoNoParseResult;
use crate::{api, models};

pub async fn endpoint(cmd: UserCommand, ctx: &mut Context) -> anyhow::Result<()> {
    match cmd.commands {
        UserAction::Follower(arg) => handle_followers(ctx, arg).await,
        UserAction::Following(arg) => handle_following(ctx, arg).await,
        UserAction::Login { token } => handle_login(token, ctx).await,
        UserAction::Logout => handle_logout(ctx),
        UserAction::Status => user_info(ctx).await,
        UserAction::Token => handle_print_token(ctx),
    }
}

async fn handle_login(token: String, ctx: &mut Context) -> Result<()> {
    let header_value = format!("Bearer {}", token);
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, header_value.parse()?);

    let client = ClientBuilder::new().default_headers(header).build()?;
    let resp = api::user::raw_user_info(&client).await?;

    if resp.status().eq(&StatusCode::UNAUTHORIZED) {
        let _ = ctx
            .terminal
            .writeln(format!("Token `{}`错误。请输入正确的token。", token).red());
    }

    if resp.status().is_success() {
        let p = resp.json::<models::user::UserInfo>().await?;
        let name = p.display_name.clone();

        let mut c: Cache = p.into();
        c.token = token;
        ctx.save_cache(c)?;

        ctx.terminal
            .writeln(format!("🎉 欢迎，{}！", name.bright_green()))?;
    } else {
        let r = resp.into_no_parse_result().await?;
        ctx.terminal.writeln(r.into_format())?;
    }
    Ok(())
}

fn handle_print_token(ctx: &mut Context) -> Result<()> {
    ctx.terminal
        .writeln(format!("[Token]: {}", ctx.cache.token.bright_green()))
}

async fn user_info(ctx: &mut Context) -> Result<()> {
    let user = api::user::user_info(&ctx.client).await?;
    let c: Cache = user.clone().into();
    ctx.save_cache(c)?;
    ctx.terminal.writeln(user.format_user_info())
}

fn handle_logout(ctx: &Context) -> Result<()> {
    ctx.clean()
}

async fn handle_followers(ctx: &mut Context, arg: FollowArg) -> Result<()> {
    let followers = api::user::user_followers(&ctx.client, &arg).await?;

    ctx.terminal.writeln(
        format!(
            "{} 人关注了您, 当前{}页，每页数量{}",
            followers.total_count, arg.page_index, arg.page_size
        )
        .bright_green(),
    )?;

    followers
        .items
        .iter()
        .for_each(|f| ctx.terminal.writeln(f.as_format()).unwrap());
    Ok(())
}

async fn handle_following(ctx: &mut Context, arg: FollowArg) -> Result<()> {
    let following = api::user::user_following(&ctx.client, &arg).await?;
    ctx.terminal.writeln(
        format!(
            "您关注了 {} 人, 当前{}页，每页数量{}",
            following.total_count, arg.page_index, arg.page_size
        )
        .bright_green(),
    )?;
    following
        .items
        .iter()
        .for_each(|f| ctx.terminal.writeln(f.as_format()).unwrap());
    Ok(())
}
