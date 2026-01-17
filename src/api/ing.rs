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
    Ok(raw_get_status(c, id).await?.json().await?)
}

pub async fn list_comments(c: &Client, id: u64) -> Result<Vec<IngComment>> {
    Ok(raw_list_comments(c, id).await?.json().await?)
}

pub async fn list_statuses(
    c: &Client,
    path: &str,
    params: impl Serialize + Send + Sync,
) -> Result<Vec<IngInfo>> {
    let resp = raw_list_statuses(c, path, params).await?;
    Ok(resp.error_for_status()?.json().await?)
}

pub async fn raw_list_comments(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}/{}", STATUS, id, COMMENTS_PATH);
    Ok(c.get(url).send().await?)
}

pub async fn raw_list_statuses(
    c: &Client,
    path: &str,
    params: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = format!("{}@{}", STATUS, path);
    Ok(c.get(url).query(&params).send().await?)
}

pub async fn raw_create_status(
    c: &Client,
    content: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = STATUS.to_string();
    Ok(c.post(url).json(&content).send().await?)
}

pub async fn raw_create_comment(c: &Client, id: u64, content: String) -> Result<Response> {
    let url = format!("{}/{}/{}", STATUS, id, COMMENTS_PATH);
    let res = json!({"content": content});
    Ok(c.post(url).json(&res).send().await?)
}

pub async fn raw_delete_status(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}", STATUS, id);
    Ok(c.delete(url).send().await?)
}

pub async fn raw_delete_status_comment(
    c: &Client,
    status_id: u64,
    comment_id: u64,
) -> Result<Response> {
    let url = format!("{}{}/{}/{}", STATUS, status_id, COMMENTS_PATH, comment_id);
    Ok(c.delete(url).send().await?)
}

pub async fn raw_get_status(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}{}", STATUS, id);
    Ok(c.get(url).send().await?)
}
