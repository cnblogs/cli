pub mod comment;
pub mod create;

use crate::infra::result::IntoResult;
use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;

pub mod get_comment_list;
pub mod get_list;

pub struct Ing {
    pat: String,
}

impl Ing {
    pub const fn new(pat: String) -> Self {
        Self { pat }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum IngSendFrom {
    None = 0,
    Ms = 1,
    GTalk = 2,
    Qq = 3,
    Sms = 5,
    CellPhone = 6,
    Web = 8,
    VsCode = 9,
    Cli = 13,
}

impl TryFrom<usize> for IngSendFrom {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Self::None,
            1 => Self::Ms,
            2 => Self::GTalk,
            3 => Self::Qq,
            5 => Self::Sms,
            6 => Self::CellPhone,
            8 => Self::Web,
            9 => Self::VsCode,
            13 => Self::Cli,
            u => bail!("Unknown value of ing source: {}", u),
        }
        .into_ok()
    }
}

pub fn ing_star_tag_to_text(tag: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r#"<img.*alt="\[(.*?)]"(\n|.)*>"#).unwrap();
    }
    let caps = REGEX.captures(tag).expect(tag);
    let text = caps.get(1).unwrap().as_str();
    text.to_string()
}

pub fn fmt_content(content: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>(@.*?)</a>"#).unwrap();
    }
    REGEX.captures(content).map_or_else(
        || content.to_string(),
        |caps| {
            let at_user = caps.get(1).unwrap().as_str();
            REGEX.replace(content, at_user).to_string()
        },
    )
}

pub fn rm_ing_at_user_tag(text: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>(@.*?)</a>："#).unwrap();
    }
    REGEX.replace(text, "".to_string()).to_string()
}

pub fn get_ing_at_user_tag_text(text: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>@(.*?)</a>："#).unwrap();
    }
    REGEX.captures(text).map_or_else(
        || "".to_string(),
        |caps| caps.get(1).unwrap().as_str().to_string(),
    )
}
