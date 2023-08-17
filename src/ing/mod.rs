/*mod comment;
mod r#pub;
*/
use crate::infra::option::IntoOption;
use crate::infra::result::IntoResult;
use anyhow::{anyhow, bail};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Error;

mod get_comment_list;
mod get_list;

pub struct Ing {
    pat: String,
}

impl Ing {
    pub fn new(pat: String) -> Ing {
        Ing { pat }
    }
}

pub enum IngType {
    Following = 1,
    Myself = 4,
    Public = 5,
    RecentComment = 6,
    MyComment = 7,
    Tag = 10,
    Comment = 13,
    Mention = 14,
}

pub enum IngSendFrom {
    None = 0,
    MS = 1,
    GTalk = 2,
    QQ = 3,
    SMS = 5,
    CellPhone = 6,
    Web = 8,
    Code = 9,
}

impl TryFrom<usize> for IngSendFrom {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => IngSendFrom::None,
            1 => IngSendFrom::MS,
            2 => IngSendFrom::GTalk,
            3 => IngSendFrom::QQ,
            5 => IngSendFrom::SMS,
            6 => IngSendFrom::CellPhone,
            8 => IngSendFrom::Web,
            9 => IngSendFrom::Code,
            u => bail!("Unknown value of ing source: {}", u),
        }
        .into_ok()
    }
}

pub fn ing_star_icon_to_text(icon: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r#"<img.*alt="\[(.*?)]".*>"#).unwrap();
    }
    let caps = REGEX.captures(icon).unwrap();
    let star_text = caps.get(1).unwrap().as_str();
    star_text.to_string()
}
