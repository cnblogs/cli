use crate::api::ing::Ing;
use crate::infra::http::{unit_or_err, RequestBuilderExt};
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

impl Ing {
    pub async fn comment(
        &self,
        ing_id: usize,
        content: String,
        reply_to: Option<usize>,
        parent_comment_id: Option<usize>,
    ) -> Result<()> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/statuses/{}/comments", ing_id);
            let body = {
                #[serde_with::skip_serializing_none]
                #[derive(Clone, Debug, Serialize, Deserialize)]
                struct Body {
                    #[serde(rename(serialize = "replyTo"))]
                    reply_to: Option<usize>,
                    #[serde(rename(serialize = "parentCommentId"))]
                    parent_comment_id: Option<usize>,
                    content: String,
                }
                Body {
                    reply_to,
                    parent_comment_id,
                    content,
                }
            };
            client.post(url).json(&body).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        unit_or_err(resp).await
    }
}
