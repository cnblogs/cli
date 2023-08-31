use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, cons_query_string, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};

impl Post {
    pub async fn get_count(&self) -> Result<usize> {
        let client = reqwest::Client::new();

        let req = {
            let url = {
                let query = vec![('t', 1), ('p', 1), ('s', 1)];
                let query = cons_query_string(query);
                blog_backend!("/posts/list?{}", query)
            };

            let req = client.get(url);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;

        let count = {
            let json = body_or_err(resp).await?;
            #[derive(Serialize, Deserialize, Debug)]
            struct Body {
                #[serde(rename = "postsCount")]
                pub total_count: usize,
            }
            let body = json::deserialize::<Body>(&json)?;

            body.total_count
        };

        count.into_ok()
    }
}
