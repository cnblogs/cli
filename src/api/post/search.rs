use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt, VecExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

impl Post {
    pub async fn search(
        &self,
        skip: usize,
        take: usize,
        keyword: &str,
    ) -> Result<(Vec<usize>, usize)> {
        let client = &reqwest::Client::new();

        // total_count is used for patch the buggy blog backend API
        // If index is greater than the max page index, API will still return the last page
        let total_count = {
            let req = {
                let query = vec![
                    ("t", "1".to_string()),
                    ("p", 1.to_string()),
                    ("s", 1.to_string()),
                    ("search", keyword.to_string()),
                ]
                .into_query_string();
                let url = blog_backend!("/posts/list?{}", query);
                client.get(url).pat_auth(&self.pat)
            };
            let resp = req.send().await?;

            // total_count
            {
                #[derive(Serialize, Deserialize, Debug)]
                struct Body {
                    #[serde(rename = "postsCount")]
                    pub total_count: usize,
                }
                let body = body_or_err(resp).await?;
                let body = json::deserialize::<Body>(&body)?;
                body.total_count
            }
        };

        let range = (skip + 1)..=(skip + take).min(total_count);
        let fut_iter = range.map(|i| async move {
            let req = {
                let query = vec![
                    ("t", "1".to_string()),
                    ("p", i.to_string()),
                    ("s", 1.to_string()),
                    ("search", keyword.to_string()),
                ]
                .into_query_string();
                let url = blog_backend!("/posts/list?{}", query);
                client.get(url).pat_auth(&self.pat)
            };
            let resp = req.send().await?;

            let id_list = {
                let body = body_or_err(resp).await?;
                let mut json = json::deserialize::<Value>(&body)?;
                let post_id_list = {
                    let json = json["postList"].take();
                    serde_json::from_value::<Vec<usize>>(json)
                }?;
                let zzk_post_id_list = {
                    let json = json["zzkSearchResult"]["postIds"].take();
                    serde_json::from_value::<Vec<usize>>(json)
                }?;

                post_id_list
                    .into_iter()
                    .chain(zzk_post_id_list.into_iter())
                    .collect::<Vec<usize>>()
            };

            id_list.into_ok::<anyhow::Error>()
        });

        let id_list = futures::future::join_all(fut_iter)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        (id_list, total_count).into_ok()
    }
}
