// "WzLinkId": 5262184,
//         "Title": "MongoDB数据库 - suoning - 博客园",
//         "LinkUrl": "https://www.cnblogs.com/suoning/p/6759367.html#3682005",
//         "Summary": "",
//         "Tags": [],
//         "DateAdded": "2019-04-05T23:33:05.263",
//         "FromCNBlogs": true

use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::Deserialize;

use crate::tools::timer::DateFormatExt;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct FavInfo {
    pub wz_link_id: u64,
    pub title: String,
    pub link_url: String,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub date_added: NaiveDateTime,
    pub from_c_n_blogs: bool,
}

impl FavInfo {
    pub fn into_format(self) -> String {
        format!(
            "{title}   [#{id}][收藏@ {date}]\n{link}\n",
            title = self.title.bold().cyan(),
            id = self.wz_link_id.bright_green(),
            date = self.date_added.as_time_age(),
            link = self.link_url.blue(),
        )
    }
}
