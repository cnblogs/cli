//!
//! 闪存API
//!
//! raw_*类型直接返回anyhow::Result<reqwest::Response>，提供原始的reponse，供自定义处理逻辑。
//!

use anyhow::{Ok, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use serde_json::json;

use crate::{
    api::urls::{COMMENTS_PATH, STATUS},
    models::ing::{IngComment, IngDetail, IngInfo},
    tools::IntoAnyhowResult,
};

/// 创建闪存
///
/// content:
pub async fn create_statuses(
    c: &Client,
    content: impl Serialize + Send + Sync,
) -> Result<Response> {
    raw_create_status(c, content).await
}

pub async fn get_status_with_comment(c: &Client, status: IngInfo) -> Result<IngDetail> {
    if status.comment_count < 1 {
        return Ok(IngDetail {
            status,
            comments: vec![],
        });
    }
    let comments = list_comments(c, status.id).await?;
    Ok(IngDetail { status, comments })
}

pub async fn get_status(c: &Client, id: u64) -> Result<IngInfo> {
    raw_get_status(c, id)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

pub async fn list_comments(c: &Client, id: u64) -> Result<Vec<IngComment>> {
    raw_list_comments(c, id)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

pub async fn list_statuses(
    c: &Client,
    path: &str,
    params: impl Serialize + Send + Sync,
) -> Result<Vec<IngInfo>> {
    let resp = raw_list_statuses(c, path, params).await?;
    resp.error_for_status()?.json().await.into_anyhow_result()
}

pub async fn raw_list_comments(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}/{}", STATUS, id, COMMENTS_PATH);
    c.get(url).send().await.into_anyhow_result()
}

pub async fn raw_list_statuses(
    c: &Client,
    path: &str,
    params: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = format!("{}@{}", STATUS, path);
    c.get(url).query(&params).send().await.into_anyhow_result()
}

pub async fn raw_create_status(
    c: &Client,
    content: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = STATUS.to_string();
    c.post(url).json(&content).send().await.into_anyhow_result()
}

pub async fn raw_create_comment(c: &Client, id: u64, content: String) -> Result<Response> {
    let url = format!("{}{}/{}", STATUS, id, COMMENTS_PATH);
    let res = json!({"content": content});
    c.post(url).json(&res).send().await.into_anyhow_result()
}

pub async fn raw_delete_status(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}", STATUS, id);
    c.delete(url).send().await.into_anyhow_result()
}

pub async fn raw_delete_status_comment(
    c: &Client,
    status_id: u64,
    comment_id: u64,
) -> Result<Response> {
    let url = format!("{}{}/{}/{}", STATUS, status_id, COMMENTS_PATH, comment_id);
    c.delete(url).send().await.into_anyhow_result()
}

pub async fn raw_get_status(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}", STATUS, id);
    c.get(url).send().await.into_anyhow_result()
}
