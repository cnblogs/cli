use crate::infra::http::setup_auth;
use crate::infra::result::IntoResult;
use crate::ing::Ing;
use crate::openapi;
use anyhow::Result;
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Body {
    #[serde(rename(serialize = "replyTo"))]
    reply_to: Option<usize>,
    #[serde(rename(serialize = "parentCommentId"))]
    parent_comment_id: Option<usize>,
    content: String,
}

impl Ing {
    pub async fn comment(
        &self,
        ing_id: usize,
        content: String,
        reply_to: Option<usize>,
        parent_comment_id: Option<usize>,
    ) -> Result<()> {
        let url = openapi!("/statuses/{}/comments", ing_id);

        let client = reqwest::Client::new();

        let req = {
            let req = client
                .post(url)
                .header(CONTENT_TYPE, APPLICATION_JSON.to_string());
            let body = Body {
                reply_to,
                parent_comment_id,
                content,
            };
            let body = serde_json::to_string(&body)?;
            let req = req.body(body);
            setup_auth(req, &self.pat)
        };

        req.send().await?;

        ().into_ok()
    }
}
