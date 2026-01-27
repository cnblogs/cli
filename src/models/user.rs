use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub user_id: String,
    #[serde(rename = "SpaceUserID")]
    pub space_user_id: u64,
    pub account_id: u64,
    pub blog_id: u64,
    pub display_name: String,
    pub face: String,
    pub avatar: String,
    pub seniority: String,
    pub blog_app: String,
    pub following_count: u64,
    pub follower_count: u64,
    pub is_vip: bool,
    pub joined: String,
}

impl UserInfo {
    /// 提取公共的用户信息格式化逻辑
    pub fn format_user_info(&self) -> String {
        let mut info = Vec::new();

        info.push(if self.is_vip {
            format!("{}[VIP]", self.display_name).red().to_string()
        } else {
            self.display_name.to_string().blue().to_string()
        });

        info.push(format!("ID：{}", self.account_id));
        info.push(format!("加入时间：{}", self.joined));
        info.push(format!("博客：https://www.cnblogs.com/{}", self.blog_app));
        info.push("📊 数据统计".into());
        info.push(format!("├─ 关注：{} 人 ", self.following_count));
        info.push(format!("├─ 粉丝：{} 人 ", self.follower_count));
        if !self.seniority.is_empty() {
            info.push(format!("└─ 园龄：{}", self.seniority));
        }
        info.join("\n")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FollowInfo {
    pub alias: String,
    pub space_user_id: u64,
    pub display_name: String,
    pub blog_app: Option<String>,
}

impl FollowInfo {
    pub fn as_format(&self) -> String {
        format!(
            "{name}   [#{id}]   [{blog}]",
            name = self.display_name.bright_blue(),
            id = self.space_user_id.bright_green(),
            blog = self
                .blog_app
                .as_ref()
                .map_or("无博客".red().to_string(), |app| format!(
                    "https://www.cnblogs.com/{}",
                    app
                )
                .blue()
                .to_string())
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserFollow {
    pub items: Vec<FollowInfo>,
    pub total_count: u64,
}
