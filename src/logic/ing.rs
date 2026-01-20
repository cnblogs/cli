//!
//! 闪存相关逻辑
//!

use anyhow::{Ok, Result};

use crate::context::Context;
use crate::tools::http::IntoNoParseResult;
use crate::{
    api,
    commands::ing::{
        IngAction, IngContent, IngDelete, IngListArg, IngReplayContent, IngShowDetail,
    },
};

pub async fn endpoint(cmd: IngAction, ctx: &mut Context) -> Result<()> {
    match cmd {
        IngAction::List(action) => handle_list_action(action, ctx).await?,
        IngAction::Create(action) => handle_create_action(action, ctx).await?,
        IngAction::Delete(action) => handle_delete_action(action, ctx).await?,
        IngAction::Replay(action) => handle_replay_action(action, ctx).await?,
        IngAction::Show(action) => handle_show_action(action, ctx).await?,
    };
    Ok(())
}

/// 查看实现
async fn handle_list_action(action: IngListArg, ctx: &mut Context) -> Result<()> {
    let resp = api::ing::list_statuses(&ctx.client, action.r#type.as_str(), &action).await?;

    match (action.no_comment, ctx.json) {
        // 不显示评论，json格式输出。
        (true, true) => {
            ctx.terminal.writeln(serde_json::to_string_pretty(&resp)?)?;
        }
        // 不显示评论，pretty输出。
        (true, false) => {
            resp.into_iter()
                .for_each(|x| ctx.terminal.writeln(x.format()).unwrap());
        }
        // 显示评论，json格式输出。
        (false, true) => {
            for i in resp {
                let ing = api::ing::get_status_with_comment(&ctx.client, i).await?;
                ctx.terminal.writeln(serde_json::to_string_pretty(&ing)?)?;
            }
        }
        // 显示评论，pretty输出。
        (false, false) => {
            for v in resp {
                ctx.terminal.writeln(
                    api::ing::get_status_with_comment(&ctx.client, v)
                        .await?
                        .format(),
                )?;
            }
        }
    }

    Ok(())
}

/// 删除实现
async fn handle_delete_action(action: IngDelete, ctx: &mut Context) -> Result<()> {
    let a = if let Some(cid) = action.comment_id {
        api::ing::raw_delete_status_comment(&ctx.client, action.id, cid)
            .await?
            .into_no_parse_result()
            .await?
    } else {
        api::ing::raw_delete_status(&ctx.client, action.id)
            .await?
            .into_no_parse_result()
            .await?
    };
    ctx.terminal.writeln(a.into_format())
}

/// 发布闪存实现
async fn handle_create_action(action: IngContent, ctx: &mut Context) -> Result<()> {
    action.validate()?;
    let a = api::ing::raw_create_status(&ctx.client, action.to_json())
        .await?
        .into_no_parse_result()
        .await?;

    ctx.terminal.writeln(a.into_format())
}

/// 回复功能实现
async fn handle_replay_action(action: IngReplayContent, ctx: &mut Context) -> Result<()> {
    let a = api::ing::raw_create_comment(&ctx.client, action.id, action.content)
        .await?
        .into_no_parse_result()
        .await?;

    ctx.terminal.writeln(a.into_format())
}

/// 处理单条展示信息
async fn handle_show_action(action: IngShowDetail, ctx: &mut Context) -> Result<()> {
    let res = api::ing::get_status(&ctx.client, action.id).await?;

    // 显示评论
    if action.comments {
        let r = api::ing::get_status_with_comment(&ctx.client, res).await?;
        ctx.terminal.writeln(r.format())
    } else {
        ctx.terminal.writeln(res.format())
    }
}
