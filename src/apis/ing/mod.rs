//! cnblogs 闪存接口模块
//!
//! 实现封装[cnblogs Api](https://api.cnblogs.com/Help#0aee001a01835c83a3277a500ffc9040)中的`Statuses`。
//!
//! - 获取自己最新一条闪存内容    https://api.cnblogs.com/api/statuses/recent      
//! - 发布闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments   
//! - 获取闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments
//! - 删除闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments/{id}
//! - 发布闪存               https://api.cnblogs.com/api/statuses
//! - 删除闪存               https://api.cnblogs.com/api/statuses/{id}
//! - 根据类型获取闪存列表    https://api.cnblogs.com/api/statuses/@{type}?pageIndex={pageIndex}&pageSize={pageSize}&tag={tag}
//! - 根据Id获取闪存         https://api.cnblogs.com/api/statuses/{id}
//!

pub mod comment;

use anyhow::{Ok, Result};
use clap::{ValueEnum, Parser};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{infra::http::RequestBuilderExt, openapi};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct IngContent {
    pub content: String,
    pub is_private: bool,
    pub lucky: bool,
    pub client_type: IngSendFrom,
}

impl Default for IngContent {
    fn default() -> Self {
        IngContent {
            content: "".to_string(),
            is_private: true,
            lucky: false,
            client_type: IngSendFrom::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IngSendFrom {
    None = 0,
    Ms = 1,
    GTalk = 2,
    Qq = 3,
    Sms = 5,
    CellPhone = 6,
    Web = 8,
    VsCode = 9,
    Cli = 13,
}

impl Default for IngSendFrom {
    fn default() -> Self {
        return IngSendFrom::Cli;
    }
}

/// 查询条件，用于根据类别查询
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct QeurySet {
    #[serde(skip)]
    pub r#type: QueryIngType,
    pub page_index: u64,
    pub page_size: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tag: String,
}

impl Default for QeurySet {
    fn default() -> Self {
        return Self {
            r#type: QueryIngType::default(),
            page_index: 1,
            page_size: 10,
            tag: "".to_string(),
        };
    }
}

///
/// Follow = 1, 关注
/// Myself = 4, 我的
/// Public = 5,
/// RecentComment = 6, //新回应
/// MyComment = 7, 我回应
/// Tag = 10,  tag 必填
/// Comment = 13 回复我
/// Mention = 14,
#[derive(Debug, Clone, ValueEnum, Parser)]
pub enum QueryIngType {
    Following = 1,
    My = 4,
    All = 5,
    RecentComment = 6,
    MyComment = 7,
    Tag = 10,
    Comment = 13,
    Mention = 14,
}

impl Default for QueryIngType {
    fn default() -> Self {
        return Self::All;
    }
}

impl From<u8> for QueryIngType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Following,
            _ => Self::All,
        }
    }
}

impl QueryIngType {
    fn as_u8(&self) -> u8 {
        match self {
            QueryIngType::Following => 1,
            QueryIngType::My => 4,
            QueryIngType::All => 5,
            QueryIngType::RecentComment => 6,
            QueryIngType::MyComment => 7,
            QueryIngType::Tag => 10,
            QueryIngType::Mention => 14,
            QueryIngType::Comment => 13,
        }
    }
}

/// 闪存详细内容。
///
/// 用于根据ID查询闪存的结果解析。
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngEntry {
    pub id: u64,
    pub content: String,
    pub is_private: bool,
    pub is_lucky: bool,
    pub comment_count: u64,
    pub date_added: String,
    pub user_alias: String,
    pub user_display_name: String,
    pub user_icon_url: String,
    pub user_id: u64,
    pub user_guid: String,
    pub send_from: u8,
    pub icons: String,
}

pub async fn lastest(token: &str) -> Result<Response> {
    let c = Client::new()
        .get(openapi!("/statuses/recent"))
        .pat_auth(token)
        .send()
        .await?
        .error_for_status()?;
    Ok(c)
}

/// 根据条件查询
///
/// 如果是tag是，一定要传入Tag,tag是自己想查询的比如Linux，Debian，Python等等。
/// 页数是从1开始的
pub async fn query(token: &str, q: &QeurySet) -> Result<Vec<IngEntry>> {
    let r = Client::new()
        .get(openapi!("/statuses/@{}", q.r#type.as_u8()))
        .pat_auth(token)
        .query(&q)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<IngEntry>>()
        .await?;
    Ok(r)
}

/// 根据ID查询
pub async fn query_by_id(token: &str, id: &u64) -> Result<IngEntry> {
    let r = Client::new()
        .get(openapi!("/statuses/{}", id))
        .pat_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<IngEntry>()
        .await?;
    Ok(r)
}

/// 发布一条闪存
pub async fn post(token: &str, c: &IngContent) -> Result<Response> {
    let r = Client::new()
        .post(openapi!("/statuses"))
        .pat_auth(token)
        .json(c)
        .send()
        .await?;
    Ok(r)
}

/// 删除一条闪存
pub async fn delete(token: &str, id: String) -> Result<Response> {
    let r = Client::new()
        .post(openapi!("/statuses/{}", id))
        .pat_auth(token)
        .send()
        .await?;
    Ok(r)
}
