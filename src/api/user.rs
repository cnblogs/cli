use anyhow::Result;
use reqwest::{Client, Response};

use crate::{api::urls, models::user::UserInfo, tools::IntoAnyhowResult};

pub async fn raw_user_info(c: &Client) -> Result<Response> {
    c.get(urls::USER).send().await.into_anyhow_result()
}

pub async fn user_info(c: &Client) -> Result<UserInfo> {
    raw_user_info(c).await?.json().await.into_anyhow_result()
}
