use crate::infra::http::setup_auth;
use crate::infra::result::IntoResult;
use crate::ing::{
    fmt_content, get_ing_at_user_tag_text, ing_star_tag_to_text, rm_ing_at_user_tag, Ing, IngType,
};
use crate::openapi;
use anyhow::{bail, Result};
use chrono::prelude::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::ops::Not;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngEntry {
    #[serde(rename = "Id")]
    id: usize,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "IsPrivate")]
    is_private: bool,
    #[serde(rename = "IsLucky")]
    is_lucky: bool,
    #[serde(rename = "CommentCount")]
    comment_count: usize,
    #[serde(rename = "DateAdded")]
    create_time: String,
    #[serde(rename = "UserAlias")]
    user_alias: String,
    #[serde(rename = "UserDisplayName")]
    user_name: String,
    #[serde(rename = "UserIconUrl")]
    user_icon_url: String,
    #[serde(rename = "UserId")]
    user_id: usize,
    #[serde(rename = "UserGuid")]
    user_guid: String,
    #[serde(rename = "SendFrom")]
    send_from: usize,
    #[serde(rename = "Icons")]
    icons: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngCommentEntry {
    #[serde(rename = "Id")]
    id: usize,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "DateAdded")]
    create_time: String,
    #[serde(rename = "StatusId")]
    status_id: usize,
    #[serde(rename = "UserAlias")]
    user_alias: String,
    #[serde(rename = "UserDisplayName")]
    user_name: String,
    #[serde(rename = "UserIconUrl")]
    user_icon_url: String,
    #[serde(rename = "UserId")]
    user_id: usize,
    #[serde(rename = "UserGuid")]
    user_guid: String,
}

impl Ing {
    pub async fn get_list(
        &self,
        page_index: usize,
        page_size: usize,
        ing_type: IngType,
    ) -> Result<Vec<(IngEntry, Vec<IngCommentEntry>)>> {
        let url = openapi!("/statuses/@{}", ing_type as usize);

        let client = reqwest::Client::new().get(url);

        let queries = vec![("pageIndex", page_index), ("pageSize", page_size)];
        let req = setup_auth(client, &self.pat).query(&queries);

        let resp = req.send().await?;

        let code = resp.status();
        let body = resp.text().await?;

        if code.is_success().not() {
            bail!("{}: {}", code, body)
        }

        let val: Value = serde_json::from_str(&body)?;
        let ing_entry_vec = serde_json::from_value::<Vec<IngEntry>>(val)?;
        let iter = ing_entry_vec.into_iter().map(|entry| async move {
            let id = entry.id;
            (entry, self.get_comment_list(id).await)
        });
        futures::future::join_all(iter)
            .await
            .into_iter()
            .map(|(a, b)| (a, b.unwrap()))
            .collect::<Vec<_>>()
            .into_ok()
    }
}

impl Display for IngEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //f.write_fmt(format_args!("{:#?}\n", self))?;

        let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))
            .map(|dt| dt.format("%m-%d %H:%M").to_string())
            .unwrap();

        f.write_fmt(format_args!("{}", create_time.dimmed()))?;
        if self.is_lucky {
            let star_text = ing_star_tag_to_text(&self.icons);
            f.write_fmt(format_args!(" {}", star_text.yellow()))?;
            f.write_fmt(format_args!("{}", "⭐".blink()))?;
        }
        f.write_fmt(format_args!("\n  {}", self.user_name.cyan()))?;
        let content = fmt_content(&self.content);
        f.write_fmt(format_args!(" {}", content))?;
        f.write_fmt(format_args!(
            " {}{}",
            "#".dimmed(),
            self.id.to_string().dimmed()
        ))?;
        ().into_ok()
    }
}

impl Display for IngCommentEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //f.write_fmt(format_args!("{:#?}\n", self))?;

        // TODO: use create_time
        let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))
            .map(|dt| dt.format("%m-%d %H:%M").to_string())
            .unwrap();

        f.write_fmt(format_args!("    │ {}", self.user_name.blue()))?;
        let at_user = get_ing_at_user_tag_text(&self.content);
        if at_user.is_empty().not() {
            f.write_fmt(format_args!(
                " {}{}",
                "@".bright_black(),
                at_user.bright_black()
            ))?;
        }
        let content = rm_ing_at_user_tag(&self.content);
        let content = fmt_content(&content);
        f.write_fmt(format_args!(" {}", content.dimmed()))?;

        ().into_ok()
    }
}
