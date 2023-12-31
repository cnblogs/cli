use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::iter::IntoIteratorExt;
use crate::infra::json;
use crate::infra::result::WrapResult;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashSet;
use std::iter;

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
                let url = blog_backend!("/posts/list");
                let query = [
                    ("t", 1.to_string()),
                    ("p", 1.to_string()),
                    ("s", 1.to_string()),
                    ("search", keyword.to_string()),
                ];
                client.get(url).query(&query).pat_auth(&self.pat)
            };
            let resp = req.send().await?;

            // total_count
            {
                let body = body_or_err(resp).await?;
                let json = json::deserialize::<Value>(&body)?;
                json["postsCount"]
                    .as_u64()
                    .expect("as_u64 failed for `postsCount`") as usize
            }
        };

        let range = (skip + 1)..=(skip + take).min(total_count);
        let id_list = range
            .map(|i| async move {
                let req = {
                    let url = blog_backend!("/posts/list");
                    let query = [
                        ("t", 1.to_string()),
                        ("p", i.to_string()),
                        ("s", 1.to_string()),
                        ("search", keyword.to_string()),
                    ];
                    client.get(url).query(&query).pat_auth(&self.pat)
                };
                let resp = req.send().await?;

                let id_list = {
                    let body = body_or_err(resp).await?;
                    let mut json = json::deserialize::<Value>(&body)?;
                    let post_id = {
                        let json = json["postList"].take();
                        let [post, ..] = serde_json::from_value::<[Value; 1]>(json)?;
                        post["id"].as_u64().expect("as_u64 failed for `id`") as usize
                    };
                    let zzk_post_id_list = {
                        let json = json["zzkSearchResult"]["postIds"].take();
                        serde_json::from_value::<Vec<usize>>(json)
                    }?;

                    zzk_post_id_list
                        .into_iter()
                        .chain(iter::once(post_id))
                        .collect::<Vec<usize>>()
                };

                id_list.wrap_ok::<anyhow::Error>()
            })
            .join_all()
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        (id_list, total_count).wrap_ok()
    }
}
