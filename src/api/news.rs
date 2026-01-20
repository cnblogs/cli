use anyhow::Result;
use reqwest::{Client, Response};
use serde::Serialize;

use crate::{api::urls::OPENAPI, models::news::NewsInfo, tools::IntoAnyhowResult};

pub async fn list_news(c: &Client, page: impl Serialize + Send + Sync) -> Result<Vec<NewsInfo>> {
    raw_list_news(c, page)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

pub async fn raw_list_news(c: &Client, page: impl Serialize + Send + Sync) -> Result<Response> {
    let url = format!("{}/{}", OPENAPI, "NewsItems");
    c.get(url).query(&page).send().await.into_anyhow_result()
}
