use crate::api::post::Post;
use crate::api::user::User;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostCommentEntry {
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "Body")]
    pub content: String,
    #[serde(rename = "Author")]
    pub user_name: String,
    #[serde(rename = "AuthorUrl")]
    pub user_home_url: String,
    #[serde(rename = "FaceUrl")]
    pub avatar_url: String,
    #[serde(rename = "Floor")]
    pub floor: usize,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
}

impl Post {
    pub async fn get_comment_list(&self, post_id: usize) -> Result<Vec<PostCommentEntry>> {
        let blog_app = User::new(self.pat.to_owned()).get_info().await?.blog_app;
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/blogs/{}/posts/{}/comments", blog_app, post_id);
            client.get(url).pat_auth(&self.pat)
        };
        let resp = req.send().await?;

        let entry_vec = {
            let body = body_or_err(resp).await?;
            json::deserialize::<Vec<PostCommentEntry>>(&body)?
        };

        entry_vec.wrap_ok()
    }
}
