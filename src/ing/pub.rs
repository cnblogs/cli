use std::ops::Not;
use crate::infra::http::{setup_auth, APPLICATION_JSON, unit_or_err};
use crate::infra::result::{IntoResult, ResultExt};
use crate::{openapi};
use anyhow::{bail, Result};
use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use crate::ing::Ing;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct IngPubErr {
    #[serde(rename = "Message")]
    message: String,
}

impl Ing {
    pub async fn r#pub(&self, content: &str) -> Result<()> {
        let url = openapi!("/statuses");

        let body = json!({
            "content": content,
            "isPrivate": false,
        })
            .to_string();

        let client = reqwest::Client::new().post(url);

        let req = setup_auth(client, &self.pat)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(body);

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
