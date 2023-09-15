use crate::api::post::get_one::PostEntry;
use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, RequestBuilderExt, VecExt};
use crate::infra::iter::IntoIteratorExt;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::Value;

/*
Fields only available over blog_backend!("/posts/list?{}", query):
  aggCount: number
  feedBackCount: number
  isInSiteCandidate: boolean
  isInSiteHome: boolean
  postConfig: number
  viewCount: number
  webCount: number
*/

impl Post {
    pub async fn get_meta_list(&self, skip: usize, take: usize) -> Result<(Vec<PostEntry>, usize)> {
        // WRN:
        // This impl has low performance but robust
        // Current API of blog backend is buggy
        // It's not worth to design a more efficient impl
        let client = &reqwest::Client::new();

        // total_count is used for patch the buggy blog backend API
        // If index is greater than the max page index, API will still return the last page
        let total_count = self.get_count().await?;

        let range = (skip + 1)..=(skip + take).min(total_count);
        let vec = range
            .map(|i| async move {
                let req = {
                    let url = {
                        let query = vec![('t', 1), ('p', i), ('s', 1)].into_query_string();
                        blog_backend!("/posts/list?{}", query)
                    };

                    client.get(url).pat_auth(&self.pat)
                };

                let resp = req.send().await?;

                let entry = {
                    let body = body_or_err(resp).await?;
                    let json = json::deserialize::<Value>(&body)?["postList"].take();

                    let [entry, ..] = serde_json::from_value::<[PostEntry; 1]>(json)?;
                    entry
                };

                entry.into_ok()
            })
            .join_all()
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>();

        (vec?, total_count).into_ok()
    }
}
