mod comment;
mod publish;

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
    pub fn new(pat: String) -> Ing {
        Ing { pat }
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
    Code = 9,
}

impl TryFrom<usize> for IngSendFrom {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => IngSendFrom::None,
            1 => IngSendFrom::Ms,
            2 => IngSendFrom::GTalk,
            3 => IngSendFrom::Qq,
            5 => IngSendFrom::Sms,
            6 => IngSendFrom::CellPhone,
            8 => IngSendFrom::Web,
            9 => IngSendFrom::Code,
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
    if let Some(caps) = REGEX.captures(content) {
        let at_user = caps.get(1).unwrap().as_str();
        REGEX.replace(content, at_user).to_string()
    } else {
        content.to_string()
    }
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
    if let Some(caps) = REGEX.captures(text) {
        caps.get(1).unwrap().as_str().to_string()
    } else {
        "".to_string()
    }
}
