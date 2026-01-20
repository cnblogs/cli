use anyhow::Result;
use reqwest::{Client, Response};
use serde::Serialize;

use crate::{api::urls::OPENAPI, models::fav::FavInfo, tools::IntoAnyhowResult};

pub async fn list_bookmarks(
    c: &Client,
    page: impl Serialize + Send + Sync,
) -> Result<Vec<FavInfo>> {
    raw_list_bookmarks(c, page)
        .await?
        .json()
        .await
        .into_anyhow_result()
}

pub async fn raw_list_bookmarks(
    c: &Client,
    page: impl Serialize + Send + Sync,
) -> Result<Response> {
    let url = format!("{}/{}", OPENAPI, "Bookmarks");
    c.get(url).query(&page).send().await.into_anyhow_result()
}
