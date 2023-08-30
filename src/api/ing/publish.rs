use crate::api::ing::Ing;
use crate::infra::http::setup_auth;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::{bail, Result};
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::Not;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct IngPubErr {
    #[serde(rename = "Message")]
    message: String,
}

impl Ing {
    pub async fn publish(&self, content: &str) -> Result<()> {
        let url = openapi!("/statuses");

        let body = json!({
            "content": content,
            "isPrivate": false,
        })
        .to_string();

        let client = reqwest::Client::new();

        let req = {
            let req = client
                .post(url)
                .header(CONTENT_TYPE, APPLICATION_JSON.to_string())
                .body(body);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;
        let code = resp.status();

        if code.is_success().not() {
            let body = resp.text().await?;
            let err = serde_json::from_str::<IngPubErr>(&body)?;
            bail!(err.message)
        }

        ().into_ok()
    }
}
