use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::{json, Value};

impl Post {
    pub async fn update(
        &self,
        id: usize,
        title: &Option<String>,
        body: &Option<String>,
        publish: &Option<bool>,
    ) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts");

            let json = {
                let mut json = self.get_one_raw(id).await?;
                if let Some(title) = title {
                    json["title"] = json!(title)
                }
                if let Some(body) = body {
                    json["postBody"] = json!(body)
                }
                if let Some(publish) = publish {
                    json["isPublished"] = json!(publish)
                }
                json
            };

            client.post(url).json(&json).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        let id = {
            let body = body_or_err(resp).await?;
            let json = json::deserialize::<Value>(&body)?;
            json["id"].as_u64().unwrap() as usize
        };

        id.into_ok()
    }
}
