use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::args::Style;
use crate::infra::result::IntoResult;
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

pub fn user_info(style: &Style, user_info: &UserInfo) -> Result<()> {
    match style {
        Style::Colorful => colorful::user_info(user_info),
        Style::Normal => normal::user_info(user_info),
        Style::Json => return json::user_info(user_info),
    };
    ().into_ok()
}

pub fn list_ing(
    style: &Style,
    ing_list: &[(IngEntry, Vec<IngCommentEntry>)],
    rev: bool,
) -> Result<()> {
    match style {
        Style::Colorful => colorful::list_ing(ing_list, rev),
        Style::Normal => normal::list_ing(ing_list, rev),
        Style::Json => return json::list_ing(ing_list, rev),
    };
    ().into_ok()
}

pub fn publish_ing(style: &Style, result: &Result<&String>) {
    match style {
        Style::Colorful => colorful::println_result(result),
        Style::Normal => normal::println_result(result),
        Style::Json => json::println_result(result),
    }
}

pub fn comment_ing(style: &Style, result: &Result<&String>) {
    match style {
        Style::Colorful => colorful::println_result(result),
        Style::Normal => normal::println_result(result),
        Style::Json => json::println_result(result),
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

pub fn delete_post(style: &Style, result: &Result<usize>) {
    match style {
        Style::Colorful => colorful::println_result(result),
        Style::Normal => normal::println_result(result),
        Style::Json => json::println_result(result),
    }
}

pub fn search_post(style: &Style, id_list: &[usize], total_count: usize, rev: bool) {
    match style {
        Style::Colorful => colorful::search_post(id_list, total_count, rev),
        Style::Normal => normal::search_post(id_list, total_count, rev),
        Style::Json => json::search_post(id_list, total_count, rev),
    }
}

pub fn create_post(style: &Style, result: &Result<usize>) {
    match style {
        Style::Colorful => colorful::println_result(result),
        Style::Normal => normal::println_result(result),
        Style::Json => json::println_result(result),
    }
}

pub fn update_post(style: &Style, result: &Result<usize>) {
    match style {
        Style::Colorful => colorful::println_result(result),
        Style::Normal => normal::println_result(result),
        Style::Json => json::println_result(result),
    }
}
