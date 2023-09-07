use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

impl Post {
    pub async fn update(
        &self,
        id: usize,
        title: &Option<String>,
        body: &Option<String>,
        publish: &Option<bool>,
    ) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts");

            let json = {
                let mut json = self.get_one_raw(id).await?;
                if let Some(title) = title {
                    json["title"] = json!(title)
                }
                if let Some(body) = body {
                    json["postBody"] = json!(body)
                }
                if let Some(publish) = publish {
                    json["isPublished"] = json!(publish)
                }
                json
            };

            let req = client.post(url).json(&json);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;

        let id = {
            let json = body_or_err(resp).await?;
            #[derive(Serialize, Deserialize, Debug)]
            struct Body {
                pub id: usize,
            }
            let body = json::deserialize::<Body>(&json)?;
            body.id
        };

        id.into_ok()
    }
}
