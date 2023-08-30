use crate::api::ing::get_list::IngCommentEntry;
use crate::api::ing::Ing;
use crate::infra::http::setup_auth;
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::{bail, Result};

impl Ing {
    pub async fn get_comment_list(&self, ing_id: usize) -> Result<Vec<IngCommentEntry>> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/statuses/{}/comments", ing_id);
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };
        let resp = req.send().await?;

        let code = resp.status();
        let body = resp.text().await?;

        if code.is_success() {
            let ing_entry_vec = json::deserialize::<Vec<IngCommentEntry>>(&body)?;
            ing_entry_vec.into_ok()
        } else {
            bail!("{}: {}", code, body)
        }
    }
}
