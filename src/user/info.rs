use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::infra::http::setup_auth;
use crate::openapi;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde_json::Value;
use crate::infra::result::IntoResult;
use crate::user::User;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserInfo {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "SpaceUserID")]
    space_user_id: usize,
    #[serde(rename = "BlogId")]
    blog_id: usize,
    #[serde(rename = "DisplayName")]
    display_name: String,
    #[serde(rename = "Face")]
    face: String,
    #[serde(rename = "Avatar")]
    avatar: String,
    #[serde(rename = "Seniority")]
    seniority: String,
    #[serde(rename = "BlogApp")]
    blog_app: String,
    #[serde(rename = "FollowingCount")]
    following_count: usize,
    #[serde(rename = "FollowerCount")]
    followers_count: usize,
    #[serde(rename = "IsVip")]
    is_vip: bool,
    #[serde(rename = "Joined")]
    joined: String,
}

impl User {
    pub async fn get_info(&self) -> Result<UserInfo> {
        let url = openapi!("/users");

        let client = reqwest::Client::new().get(url);

        let req = setup_auth(client, &self.pat);

        let resp = req.send().await?;
        let code = resp.status();
        let body = resp.text().await?;

        if code.is_success() {
            let val: Value = serde_json::from_str(&body)?;
            let user_info = serde_json::from_value::<UserInfo>(val)?;
            user_info.into_ok()
        } else {
            anyhow!("{}: {}", code, body).into_err()?
        }
    }
}

impl Display for UserInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //f.write_fmt(format_args!("{:#?}\n", self))?;

        f.write_fmt(format_args!("{}", self.display_name.cyan()))?;
        if self.is_vip {
            f.write_fmt(format_args!(" {}", " VIP ".on_blue()))?;
        }
        f.write_fmt(format_args!("\n{} Following {} Followers", self.following_count, self.followers_count))?;
        f.write_fmt(format_args!("\nID     {}", self.blog_id))?;
        f.write_fmt(format_args!("\nJoined {}", self.joined))?;
        f.write_fmt(format_args!("\nBlog   https://www.cnblogs.com/{}", self.blog_app))?;
        ().into_ok()
    }
}