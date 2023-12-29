use crate::api::news::News;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::iter::IntoIteratorExt;
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewsEntry {
    pub id: usize,
    pub title: String,
    pub summary: String,
    pub topic_id: usize,
    #[serde(rename = "TopicIcon")]
    pub topic_icon_url: Option<String>,
    pub view_count: usize,
    pub comment_count: usize,
    pub digg_count: usize,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
}

impl News {
    pub async fn get_list(&self, skip: usize, take: usize) -> Result<Vec<NewsEntry>> {
        let client = &reqwest::Client::new();

        let range = (skip + 1)..=(skip + take);
        range
            .map(|i| async move {
                let req = {
                    let url = openapi!("/newsitems");
                    let query = [("pageIndex", i), ("pageSize", 1)];
                    client.get(url).query(&query).pat_auth(&self.pat)
                };

                let resp = req.send().await?;

                let entry = {
                    let body = body_or_err(resp).await?;
                    let [entry, ..] = json::deserialize::<[NewsEntry; 1]>(&body)?;
                    entry
                };

                entry.wrap_ok::<anyhow::Error>()
            })
            .join_all()
            .await
            .into_iter()
            .collect()
    }
}
