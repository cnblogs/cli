use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{
    display::ing::{tag_comment_count, tag_lucky, tag_public, tag_sender},
    tools::strings::{ExtractAtPeople, ExtractIconAlt},
    tools::timer::DateFormatExt,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngInfo {
    pub id: u64,
    pub content: String,
    pub is_private: bool,
    pub is_lucky: bool,
    pub comment_count: u64,
    pub date_added: NaiveDateTime,
    pub user_alias: String,
    pub user_display_name: String,
    pub user_icon_url: String,
    pub user_id: u64,
    pub user_guid: String,
    pub send_from: u8,
    pub icons: String,
}

impl IngInfo {
    pub fn format(self) -> String {
        let content = self.content.extract_name().unwrap();
        let icon_tag = self.icons.extract_tag().unwrap();
        format!(
            "{name}：{content}{icon_tag}{emoji:>4}{time}[#{id}]{lucky}{public}{sender}{comments}",
            name = self.user_display_name.blue(),
            content = content,
            icon_tag = icon_tag,
            emoji = "📎",
            time = self.date_added.as_time_age().blue(),
            id = self.id.bright_green(),
            lucky = tag_lucky(self.is_lucky),
            public = tag_public(self.is_private),
            sender = tag_sender(self.send_from),
            comments = tag_comment_count(self.comment_count)
        )
        //
        // format!(
        //     "[#{id}]{name}：{content}{icon_tag}{emoji:>4}{time}{lucky}{public}{sender}{comments}",
        //     name = self.user_display_name.blue(),
        //     content = content,
        //     icon_tag = icon_tag,
        //     emoji = "📎",
        //     time = self.date_added.as_time_ago().blue(),
        //     id = self.id.bright_green(),
        //     lucky = tag_lucky(self.is_lucky),
        //     public = tag_public(self.is_private),
        //     sender = tag_sender(self.send_from),
        //     comments = tag_comment_count(self.comment_count)
        // )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngComment {
    pub id: u64,
    pub content: String,
    pub date_added: NaiveDateTime,
    pub status_id: u64,
    pub user_alias: String,
    pub user_display_name: String,
    pub user_icon_url: String,
    pub user_id: u64,
    pub user_guid: String,
}

impl IngComment {
    pub fn format(self) -> String {
        let content = self.content.extract_name().unwrap();
        // let icon_tag = self.icons.extract_tag().unwrap();
        format!(
            "{name}：{content}{emoji:>4}{time}[#{id}]",
            name = self.user_display_name.blue(),
            content = content,
            emoji = "📎",
            time = self.date_added.as_time_age(),
            id = self.id.bright_green(),
        )
    }
}

#[derive(Debug, Serialize)]
pub struct IngDetail {
    #[serde(flatten)]
    pub status: IngInfo,
    pub comments: Vec<IngComment>,
}

impl IngDetail {
    pub fn format(self) -> String {
        let res = format!("{}\n", self.status.format());

        self.comments.into_iter().fold(res, |mut acc, x| {
            acc = format!("{}{:>4}{}\n", acc, "", x.format());
            acc
        })
    }
}
