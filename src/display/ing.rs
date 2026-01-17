// use colored::Colorize;
use owo_colors::OwoColorize;

pub fn tag_lucky(is_lucky: bool) -> String {
    if is_lucky {
        "[幸运星]".yellow().to_string()
    } else {
        "".to_string()
    }
}

pub fn tag_public(is_private: bool) -> String {
    if is_private {
        "[未公开]".yellow().to_string()
    } else {
        "[公开]".blue().to_string()
    }
}

pub fn tag_sender(i: u8) -> String {
    match i {
        0 => "[none]".blue().to_string(),
        1 => "[ms]".blue().to_string(),
        2 => "[gtalk]".blue().to_string(),
        3 => "[qq]".blue().to_string(),
        5 => "[sms]".blue().to_string(),
        6 => "[mobile]".blue().to_string(),
        8 => "[web]".blue().to_string(),
        9 => "[vscode]".blue().to_string(),
        13 => "[cnb]".blue().to_string(),
        _ => "[unknow]".blue().to_string(),
    }
}

pub fn tag_comment_count(c: u64) -> String {
    if c > 0 {
        format!("[{}评论]", c)
    } else {
        "".to_string()
    }
}
