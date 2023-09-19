use crate::api::ing::Ing;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    pub async fn get_comment_list(&self, ing_id: usize) -> Result<Vec<IngCommentEntry>> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/statuses/{}/comments", ing_id);
            client.get(url).pat_auth(&self.pat)
        };
        let resp = req.send().await?;

        let entry_vec = {
            let body = body_or_err(resp).await?;
            json::deserialize::<Vec<IngCommentEntry>>(&body)?
        };

        entry_vec.into_ok()
    }
}
