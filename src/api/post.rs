use anyhow::Result;
use reqwest::{Client, Response};
use serde::Serialize;

use crate::{
    api::urls::{BLOG_POST_PREFIX, POST_PREFIX},
    models::post::PostInfo,
    tools::IntoAnyhowResult,
};

pub async fn list_someone_post(
    c: &Client,
    blog_app: &String,
    params: impl Serialize + Send + Sync,
) -> Result<Vec<PostInfo>> {
    // let a = raw_list_someone_post(c, blog_app).await?;
    // a.json().await.into_anyhow_result()
    raw_list_someone_post(c, blog_app, params)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

pub async fn show_post_detail() {}

/// 获取指定用户的随笔列表（支持分页）
///
/// ## 参数
///
/// - `c`: 已初始化的HTTP客户端实例
/// - `blog_app`: 博客用户的唯一标识（博客园中的博客别名）
/// - `params`: 查询参数，pageIndex和pageIndex两个参数，需要能实现serilize。
///
/// ## 查询参数要求
///
/// 参数应实现 `Serialize` trait，通常包含：
/// - `pageIndex`: u64 - 页码（从0或1开始，取决于API规范）
/// - `pageSize`: u64 - 每页记录数
pub async fn raw_list_someone_post(
    c: &Client,
    blog_app: &String,
    params: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = format!("{}/{}/{}", POST_PREFIX, blog_app, "posts");
    c.get(url).query(&params).send().await.into_anyhow_result()
}

pub async fn raw_show_post(c: &Client, id: u64) -> Result<Response> {
    let url = format!("{}/{}/body", BLOG_POST_PREFIX, id);
    c.get(url).send().await.into_anyhow_result()
}

/// 获取指定id的随笔评论
///
/// ## 参数
///
/// - `c`: 已初始化的HTTP客户端实例
/// - `blog_app`: 博客园的博客名称
/// - `id`: 随笔ID
pub async fn raw_list_comments(
    c: &Client,
    blog_app: &String,
    id: u64,
    params: impl Serialize + Send + Sync,
) -> Result<Response> {
    c.get(gen_comments_url(blog_app, id))
        .query(&params)
        .send()
        .await
        .into_anyhow_result()
}

pub async fn raw_create_comment(
    c: &Client,
    blog_app: &String,
    id: u64,
    params: impl Serialize + Send + Sync,
) -> Result<Response> {
    c.post(gen_comments_url(blog_app, id))
        .json(&params)
        .send()
        .await
        .into_anyhow_result()
}

// pub async fn raw_post_detail(c: &Client, id: u64) -> Result<Response> {
//     let url = format!("{}/{}", BLOG_BACKEND_POST, id);
//     c.get(url).send().await.into_anyhow_result()
// }

fn gen_comments_url(blog_app: &String, id: u64) -> String {
    format!(
        "{}/{}/{}/{}/{}",
        POST_PREFIX, blog_app, "posts", id, "comments"
    )
}
