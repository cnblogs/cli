//! 闪存评论相关
//!

use anyhow::{Ok, Result};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::{api::ing::get_comment_list::IngCommentEntry, infra::http::RequestBuilderExt, openapi};

/// 闪存评论及评论回复
///
/// replay_to: 在web端有一个ReplyToUserId，这里盲猜是这个
/// parent_comment_id: 0 是对某条闪存评论，如果对闪存评论要回应，这里则是闪存评论的id
/// content: 评论内容。 如果是对闪存评论回应，则应加上`@用户名称`
///
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct StatusComment {
    #[serde(skip)]
    pub status_id: String,
    pub replay_to: u64,
    pub parent_comment_id: u64,
    pub content: String,
}

/// 根据闪存ID发表一个评论
pub async fn post(token: String, sc: StatusComment) -> Result<Response> {
    let r = Client::new()
        .post(openapi!("/statuses/{}/comments", sc.parent_comment_id))
        .pat_auth(token.as_str())
        .form(&sc)
        .send()
        .await?
        .error_for_status()?;
    Ok(r)
}

/// 根据闪存ID获取评论
pub async fn get(token: &str, status_id: &str) -> Result<Vec<IngCommentEntry>> {
    let r = Client::new()
        .get(openapi!("/statuses/{}/comments", status_id))
        .pat_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(r)
}

/// 根据闪存ID和commentid删除评论
pub async fn delete(token: &str, status_id: &str, comment_id: &str) -> Result<()> {
    Client::new()
        .delete(openapi!("/statuses/{}/comments/{}", status_id, comment_id))
        .pat_auth(token)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
