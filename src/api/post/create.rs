use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

impl Post {
    pub async fn create(&self, title: &str, body: &str, publish: bool) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts");
            let body = json!({
                "postType": 1,
                "title": title,
                "postBody": body,
                "isPublished": publish
            });
            let req = client.post(url).json(&body);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;

        let id = {
            let json = body_or_err(resp).await?;
            #[derive(Serialize, Deserialize, Debug)]
            struct Body {
                pub id: usize,
            }
            let body = json::deserialize::<Body>(&json)?;
            body.id
        };

        id.into_ok()
    }
}
