use crate::api::news::News;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use anyhow::Result;

impl News {
    pub async fn get_body(&self, id: usize) -> Result<String> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("newsitems/{}/body", id);
            client.get(url).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        body_or_err(resp).await
    }
}
