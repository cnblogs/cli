use crate::api::user::User;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub user_id: String,
    #[serde(rename = "SpaceUserID")]
    pub space_user_id: usize,
    pub blog_id: usize,
    pub display_name: String,
    pub face: String,
    pub avatar: String,
    pub seniority: String,
    pub blog_app: String,
    pub following_count: usize,
    #[serde(rename = "FollowerCount")]
    pub followers_count: usize,
    pub is_vip: bool,
    pub joined: String,
}

impl User {
    pub async fn get_info(&self) -> Result<UserInfo> {
        let client = reqwest::Client::new();

        let req = {
            let url = openapi!("/users");
            client.get(url).pat_auth(&self.pat)
        };

        let resp = req.send().await?;

        let user_info = {
            let body = body_or_err(resp).await?;
            json::deserialize::<UserInfo>(&body)?
        };

        user_info.wrap_ok()
    }
}
