use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::args::Style;
use anyhow::Result;
use std::path::PathBuf;

mod colorful;
mod json;
mod normal;

pub fn login(style: &Style, cfg_path: &PathBuf) {
    match style {
        Style::Colorful => colorful::login(cfg_path),
        Style::Normal => normal::login(cfg_path),
        Style::Json => json::login(cfg_path),
    }
}

pub fn logout(style: &Style, cfg_path: &PathBuf) {
    match style {
        Style::Colorful => colorful::logout(cfg_path),
        Style::Normal => normal::logout(cfg_path),
        Style::Json => json::logout(cfg_path),
    }
}

pub fn user_info(style: &Style, user_info: &UserInfo) {
    match style {
        Style::Colorful => colorful::user_info(user_info),
        Style::Normal => normal::user_info(user_info),
        Style::Json => json::user_info(user_info).unwrap(),
    }
}

pub fn list_ing(style: &Style, ing_list: &[(IngEntry, Vec<IngCommentEntry>)], rev: bool) {
    match style {
        Style::Colorful => colorful::list_ing(ing_list, rev),
        Style::Normal => normal::list_ing(ing_list, rev),
        Style::Json => json::list_ing(ing_list, rev).unwrap(),
    }
}

pub fn publish_ing(style: &Style, result: &Result<&String>) {
    match style {
        Style::Colorful => colorful::publish_ing(result),
        Style::Normal => normal::publish_ing(result),
        Style::Json => json::publish_ing(result),
    }
}

pub fn comment_ing(style: &Style, result: &Result<&String>) {
    match style {
        Style::Colorful => colorful::comment_ing(result),
        Style::Normal => normal::comment_ing(result),
        Style::Json => json::comment_ing(result),
    }
}

pub fn show_post(style: &Style, entry: &PostEntry) {
    match style {
        Style::Colorful => colorful::show_post(entry),
        Style::Normal => normal::show_post(entry),
        Style::Json => json::show_post(entry),
    }
}

pub fn show_post_meta(style: &Style, entry: &PostEntry) -> Result<()> {
    match style {
        Style::Colorful => colorful::show_post_meta(entry),
        Style::Normal => normal::show_post_meta(entry),
        Style::Json => json::show_post_meta(entry),
    }
}

pub fn list_post(style: &Style, entry_list: &[PostEntry], total_count: usize, rev: bool) {
    match style {
        Style::Colorful => colorful::list_post(entry_list, total_count, rev),
        Style::Normal => normal::list_post(entry_list, total_count, rev),
        Style::Json => json::list_post(entry_list, total_count, rev),
    }
}
