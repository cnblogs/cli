use anyhow::Result;
use reqwest::{Client, Response};

use crate::{
    api::urls,
    models::user::{UserFollow, UserInfo},
    tools::IntoAnyhowResult,
};

pub async fn raw_user_info(c: &Client) -> Result<Response> {
    c.get(urls::USER).send().await.into_anyhow_result()
}

pub async fn user_info(c: &Client) -> Result<UserInfo> {
    raw_user_info(c).await?.json().await.into_anyhow_result()
}

/// 获取用户粉丝列表的原始响应
/// page - 分页参数，包含 pageIndex 和 pageSize
pub async fn raw_user_followers(
    c: &Client,
    page: impl serde::Serialize + Send + Sync,
) -> Result<Response> {
    c.get(urls::USER_FOLLOWERS)
        .query(&page)
        .send()
        .await
        .into_anyhow_result()
}

/// 获取用户粉丝列表
///
/// page - 分页参数，包含 pageIndex 和 pageSize
pub async fn user_followers(
    c: &Client,
    page: impl serde::Serialize + Send + Sync,
) -> Result<UserFollow> {
    raw_user_followers(c, page)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

/// 获取用户粉丝列表的原始响应
/// page - 分页参数，包含 pageIndex 和 pageSize
pub async fn raw_user_following(
    c: &Client,
    page: impl serde::Serialize + Send + Sync,
) -> Result<Response> {
    c.get(urls::USER_FOLLOWING)
        .query(&page)
        .send()
        .await
        .into_anyhow_result()
}

/// 获取用户粉丝列表
///
/// page - 分页参数，包含 pageIndex 和 pageSize
pub async fn user_following(
    c: &Client,
    page: impl serde::Serialize + Send + Sync,
) -> Result<UserFollow> {
    raw_user_following(c, page)
        .await?
        .json()
        .await
        .into_anyhow_result()
}
