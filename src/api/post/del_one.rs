use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{setup_auth, unit_or_err};
use anyhow::Result;

impl Post {
    pub async fn del_one(&self, id: usize) -> Result<()> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts/{}", id);
            let req = client.delete(url);
            setup_auth(req, &self.pat)
        };
        let resp = req.send().await?;

        unit_or_err(resp).await
    }
}
