use crate::api::user::User;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "SpaceUserID")]
    pub space_user_id: usize,
    #[serde(rename = "BlogId")]
    pub blog_id: usize,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Face")]
    pub face: String,
    #[serde(rename = "Avatar")]
    pub avatar: String,
    #[serde(rename = "Seniority")]
    pub seniority: String,
    #[serde(rename = "BlogApp")]
    pub blog_app: String,
    #[serde(rename = "FollowingCount")]
    pub following_count: usize,
    #[serde(rename = "FollowerCount")]
    pub followers_count: usize,
    #[serde(rename = "IsVip")]
    pub is_vip: bool,
    #[serde(rename = "Joined")]
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
            let json = body_or_err(resp).await?;
            json::deserialize::<UserInfo>(&json)?
        };

        user_info.into_ok()
    }
}
