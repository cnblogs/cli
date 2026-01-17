//!
//! 认证模块
//!

use anyhow::Result;
use owo_colors::OwoColorize;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use reqwest::{ClientBuilder, StatusCode};

use crate::commands::auth::{Authenticate, AuthenticateSubCommands};
use crate::context::Context;
use crate::tools::http::IntoNoParseResult;
use crate::{api, models};

pub async fn endpoint(cmd: Authenticate, ctx: &mut Context) -> anyhow::Result<()> {
    match cmd.commands {
        AuthenticateSubCommands::Login { token } => handle_login(token, ctx).await,
        AuthenticateSubCommands::Logout => handle_logout(ctx),
        AuthenticateSubCommands::Status => user_info(ctx).await,
        AuthenticateSubCommands::Token => handle_print_token(ctx),
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
        ctx.update_token(token)?;
        let p = resp.json::<models::user::UserInfo>().await?;

        ctx.terminal
            .writeln(format!("🎉 欢迎，{}！", p.display_name.bright_green()))?;
    } else {
        let r = resp.into_no_parse_result().await?;
        ctx.terminal.writeln(r.into_format())?;
    }
    Ok(())
}

fn handle_print_token(ctx: &mut Context) -> Result<()> {
    ctx.terminal
        .writeln(format!("[Token]: {}", ctx.token.bright_green()))
}

async fn user_info(ctx: &mut Context) -> Result<()> {
    let user = api::user::user_info(&ctx.client).await?;
    ctx.terminal.writeln(user.format_user_info())
}

fn handle_logout(ctx: &mut Context) -> Result<()> {
    ctx.clean()
}
