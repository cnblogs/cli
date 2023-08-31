use crate::api::post::get_one::PostEntry;
use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, cons_query_string, setup_auth};
use crate::infra::json;
use crate::infra::option::IntoOption;
use crate::infra::result::IntoResult;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

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
    pub async fn get_meta_list(&self, skip: usize, take: usize) -> Result<Vec<PostEntry>> {
        // WRN:
        // This impl has low performance but robust
        // Current API of blog backend is buggy
        // It's not worth to design a more efficient impl

        let client = reqwest::Client::new();

        // total_count is used for patch the buggy blog backend API
        // If index is greater than the max page index, API will still return the last page
        let mut total_count = None;

        let mut entry_vec = vec![];

        for i in (skip + 1)..=(skip + take) {
            if let Some(count) = total_count && count == i {
                break;
            }
            let req = {
                let url = {
                    let query = vec![('t', 1), ('p', i), ('s', 1)];
                    let query = cons_query_string(query);
                    blog_backend!("/posts/list?{}", query)
                };

                let req = client.get(url);
                setup_auth(req, &self.pat)
            };

            let resp = req.send().await?;

            let entry = {
                let json = body_or_err(resp).await?;
                #[derive(Serialize, Deserialize, Debug)]
                struct Body {
                    #[serde(rename = "postList")]
                    pub list: Vec<PostEntry>,
                    #[serde(rename = "postsCount")]
                    pub total_count: usize,
                }
                let mut body = json::deserialize::<Body>(&json)?;

                if total_count.is_none() {
                    total_count = body.total_count.into_some();
                }

                body.list.pop().ok_or(anyhow!("No item in response list"))
            }?;

            entry_vec.push(entry)
        }

        entry_vec.into_ok()
    }
}
