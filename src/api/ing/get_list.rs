use crate::api::ing::{fmt_content, ing_star_tag_to_text, Ing, IngType};
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngEntry {
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "IsPrivate")]
    pub is_private: bool,
    #[serde(rename = "IsLucky")]
    pub is_lucky: bool,
    #[serde(rename = "CommentCount")]
    pub comment_count: usize,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
    #[serde(rename = "UserAlias")]
    pub user_alias: String,
    #[serde(rename = "UserDisplayName")]
    pub user_name: String,
    #[serde(rename = "UserIconUrl")]
    pub user_icon_url: String,
    #[serde(rename = "UserId")]
    pub user_id: usize,
    #[serde(rename = "UserGuid")]
    pub user_guid: String,
    #[serde(rename = "SendFrom")]
    pub send_from: usize,
    #[serde(rename = "Icons")]
    pub icons: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngCommentEntry {
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
    #[serde(rename = "StatusId")]
    pub status_id: usize,
    #[serde(rename = "UserAlias")]
    pub user_alias: String,
    #[serde(rename = "UserDisplayName")]
    pub user_name: String,
    #[serde(rename = "UserIconUrl")]
    pub user_icon_url: String,
    #[serde(rename = "UserId")]
    pub user_id: usize,
    #[serde(rename = "UserGuid")]
    pub user_guid: String,
}

impl Ing {
    pub async fn get_list(
        &self,
        skip: usize,
        take: usize,
        ing_type: &IngType,
    ) -> Result<Vec<(IngEntry, Vec<IngCommentEntry>)>> {
        let client = &reqwest::Client::new();

        let range = (skip + 1)..=(skip + take);
        let fut_iter = range.map(|i| async move {
            let req = {
                let url = openapi!("/statuses/@{}", ing_type.clone() as usize);
                let req = client.get(url);
                let queries = vec![("pageIndex", i), ("pageSize", 1)];
                setup_auth(req, &self.pat).query(&queries)
            };

            let resp = req.send().await?;

            let body = body_or_err(resp).await?;

            let entry_with_comment = {
                let entry = json::deserialize::<Vec<IngEntry>>(&body)?
                    .pop()
                    .ok_or(anyhow!("No item in response list"))?;

                let id = entry.id;
                (entry, self.get_comment_list(id).await?)
            };

            entry_with_comment.into_ok::<anyhow::Error>()
        });

        futures::future::join_all(fut_iter)
            .await
            .into_iter()
            .collect()
    }
}
