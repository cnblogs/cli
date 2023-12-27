use crate::api::ing::Ing;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngCommentEntry {
    pub id: usize,
    pub content: String,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
    pub status_id: usize,
    pub user_alias: String,
    #[serde(rename = "UserDisplayName")]
    pub user_name: String,
    pub user_icon_url: String,
    pub user_id: usize,
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

        entry_vec.wrap_ok()
    }
}
