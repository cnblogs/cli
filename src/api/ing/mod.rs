pub mod comment;
pub mod publish;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use regex::Regex;
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum IngType {
    Follow = 1,
    Myself = 4,
    Public = 5,
    //RecentComment = 6,
    MyComment = 7,
    //Tag = 10,
    //Comment = 13,
    //Mention = 14,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
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

pub fn ing_star_tag_to_text(tag: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<img.*alt="\[(.*?)]"(\n|.)*>"#).expect("Invalid regexp");
    }
    let caps = REGEX
        .captures(tag)
        .unwrap_or_else(|| panic!("No captures for: {}", tag));
    let text = caps.get(1).expect("No capture at index 1").as_str();
    text.to_string()
}

pub fn fmt_content(content: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>(@.*?)</a>"#)
                .expect("Invalid regexp");
    }
    REGEX.captures(content).map_or_else(
        || content.to_owned(),
        |caps| {
            let at_user = caps.get(1).expect("No capture at index 1").as_str();
            REGEX.replace(content, at_user).to_string()
        },
    )
}

pub fn rm_ing_at_user_tag(text: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>(@.*?)</a>："#)
                .expect("Invalid regexp");
    }
    REGEX.replace(text, "").to_string()
}

pub fn get_ing_at_user_tag_text(text: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"<a.*href="https://home.cnblogs.com/u/.*?".*>@(.*?)</a>："#)
                .expect("Invalid regexp");
    }
    REGEX.captures(text).map_or_else(String::new, |caps| {
        caps.get(1)
            .expect("No capture at index 1")
            .as_str()
            .to_string()
    })
}
