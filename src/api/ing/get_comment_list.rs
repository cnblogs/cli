use crate::api::ing::get_list::IngCommentEntry;
use crate::api::ing::Ing;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::Result;

impl Ing {
    pub async fn get_comment_list(&self, ing_id: usize) -> Result<Vec<IngCommentEntry>> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/statuses/{}/comments", ing_id);
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };
        let resp = req.send().await?;

        let entry_vec = {
            let body = body_or_err(resp).await?;
            json::deserialize::<Vec<IngCommentEntry>>(&body)?
        };

        entry_vec.into_ok()
    }
}
