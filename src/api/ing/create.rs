use crate::api::ing::Ing;
use crate::infra::http::{unit_or_err, RequestBuilderExt};
use crate::openapi;
use anyhow::Result;
use serde_json::json;

impl Ing {
    pub async fn create(&self, content: &str) -> Result<()> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/statuses");
            let body = json!({
                "content": content,
                "isPrivate": false,
            });

            client.post(url).json(&body).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        unit_or_err(resp).await
    }
}
