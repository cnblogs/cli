use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// TODO: not elegant
#[derive(Serialize, Deserialize, Debug)]
pub struct PostEntry {
    pub id: usize,
    pub title: String,
    pub url: String,

    #[serde(rename = "datePublished")]
    pub create_time: String,
    #[serde(rename = "dateUpdated")]
    pub modify_time: String,

    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "isPublished")]
    pub is_published: bool,

    // WRN:
    // Limited by the design of blog backend API
    // None implies that this filed is not fetched from server yet but DOSE NOT MEAN IT NOT EXIST
    #[serde(rename = "feedBackCount")]
    pub comment_count: Option<usize>,
    #[serde(rename = "postBody")]
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Post {
    pub async fn get_one(&self, id: usize) -> Result<PostEntry> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts/{}", id);
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;

        let entry = {
            let json = body_or_err(resp).await?;
            #[derive(Serialize, Deserialize, Debug)]
            struct Body {
                #[serde(rename = "blogPost")]
                pub entry: PostEntry,
            }
            let body = json::deserialize::<Body>(&json)?;
            body.entry
        };

        entry.into_ok()
    }
}
