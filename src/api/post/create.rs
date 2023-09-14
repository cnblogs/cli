use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::{json, Value};

impl Post {
    pub async fn create(&self, title: &str, body: &str, publish: bool) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts");
            let body = json!({
                "postType": 1,
                "title": title,
                "postBody": body,
                "isPublished": publish,
                "displayOnHomePage": true
            });
            client.post(url).json(&body).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        let id = {
            let body = body_or_err(resp).await?;
            let json = json::deserialize::<Value>(&body)?;
            json["id"].as_u64().expect("as_u64 failed for `id`") as usize
        };

        id.into_ok()
    }
}
