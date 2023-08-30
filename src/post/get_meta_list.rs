use crate::blog_backend;
use crate::infra::http::{cons_query_string, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::post::get_one::PostEntry;
use crate::post::Post;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::ops::Not;

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

        let mut meta_list = vec![];

        for i in (skip + 1)..=(skip + take) {
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

            let code = resp.status();
            let body = resp.text().await?;

            if code.is_success().not() {
                bail!("{}: {}", code, body)
            }

            let mut body = {
                #[derive(Serialize, Deserialize, Debug)]
                struct Body {
                    #[serde(rename = "postList")]
                    pub list: Vec<PostEntry>,
                }

                json::deserialize::<Body>(&body)?
            };

            meta_list.append(&mut body.list)
        }

        meta_list.into_ok()
    }
}
