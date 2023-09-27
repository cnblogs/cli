use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::Value;

impl Post {
    pub async fn get_count(&self) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts/list");
            let query = [('t', 1), ('p', 1), ('s', 1)];
            client.get(url).query(&query).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        let count = {
            let body = body_or_err(resp).await?;
            let json = json::deserialize::<Value>(&body)?;
            json["postsCount"]
                .as_u64()
                .expect("as_u64 failed for `postsCount`") as usize
        };

        count.into_ok()
    }
}
