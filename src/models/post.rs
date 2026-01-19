use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct PostInfo {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub description: String,
    pub author: String,
    pub blog_app: String,
    pub avatar: String,
    pub post_date: NaiveDateTime,
    pub view_count: u32,
    pub comment_count: u32,
    pub digg_count: u32,
}

impl PostInfo {
    pub fn into_format(self) -> String {
        let view_count = if self.view_count != 0 {
            format!("[阅读数：{}]", self.view_count)
        } else {
            "".to_string()
        };

        let comment_count = if self.comment_count != 0 {
            format!("[评论数：{}]", self.comment_count)
        } else {
            "".to_string()
        };

        let digg_count = if self.digg_count != 0 {
            format!("[点赞数：{}]", self.digg_count)
        } else {
            "".to_string()
        };
        format!(
            "[#{id}] {title}{space:>4}[{author}]{view_count}{comment_count}{digg_count}",
            id = self.id.bright_green(),
            title = self.title.bold().bright_magenta(),
            author = self.author,
            space = "",
            // date = self.post_date
        )
    }

    pub fn as_markdown_header(&self) -> String {
        format!("# {title}\n", title = self.title)
    }

    pub fn as_markdown_foot(&self) -> String {
        format!(
            "\n[阅读数：{vc}]  [评论数：{cc}]  [点赞数：{dc}]  [创建时间：{time}]",
            vc = self.view_count,
            cc = self.comment_count,
            dc = self.digg_count,
            time = self.post_date,
        )
    }
}

pub struct PostContent {
    pub info: PostInfo,
    pub content: String,
}

impl PostContent {
    pub fn into_format(self) -> String {
        let body = html2md::parse_html(&self.content);

        format!(
            "{header}{body}{footer}",
            header = self.info.as_markdown_header(),
            footer = self.info.as_markdown_foot()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = r#"[
    {
        "Id": 19276632,
        "Title": "Android 模拟器root权限",
        "Url": "https://www.cnblogs.com/linga/p/19276632",
        "Description": "前置： adb，Adnroid Studio Emulator，在命令行可执行，或者通过绝对路径执行 创建模拟器 首先，启动Android Studio并创建一个模拟器AVD（Android虚拟设备）。在创建AVD时请务必注意服务类型（Google Play Store，Google APIs，An",
        "Author": "咕咚!",
        "BlogApp": "linga",
        "Avatar": "https://pic.cnblogs.com/face/1284904/20180727165140.png",
        "PostDate": "2025-11-27T11:32:00",
        "ViewCount": 83,
        "CommentCount": 0,
        "DiggCount": 0
    },
    {
        "Id": 18193317,
        "Title": "Docker 编译安装Nginx正向代理",
        "Url": "https://www.cnblogs.com/linga/p/18193317",
        "Description": "先记录一波正向代理 目录： . ├── Dockerfile ├── Dockerfile.bak ├── nginx-1.24.0.tar.gz ├── nginx.conf ├── openssl-3.3.0.tar.gz ├── pcre2-10.43.tar.bz2 ├── pcre-8.4",
        "Author": "咕咚!",
        "BlogApp": "linga",
        "Avatar": "https://pic.cnblogs.com/face/1284904/20180727165140.png",
        "PostDate": "2024-05-15T10:28:00",
        "ViewCount": 256,
        "CommentCount": 0,
        "DiggCount": 0
    }
]"#;

    #[test]
    fn test_parse() {
        let a: Vec<PostInfo> = serde_json::from_str(A).unwrap();
        assert_eq!(a[0].id, 19276632)
    }
}
