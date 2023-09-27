use crate::api::fav::get_list::FavEntry;
use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::api::news::get_list::NewsEntry;
use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::args::{Style, TimeStyle};
use crate::infra::result::IntoResult;
use anyhow::Result;
use std::path::PathBuf;

mod colorful;
mod json;
mod normal;

pub fn login(style: &Style, cfg_path: &Result<PathBuf>) -> String {
    match style {
        Style::Colorful => colorful::login(cfg_path),
        Style::Normal => normal::login(cfg_path),
        Style::Json => json::login(cfg_path),
    }
}

pub fn logout(style: &Style, cfg_path: &Result<PathBuf>) -> String {
    match style {
        Style::Colorful => colorful::logout(cfg_path),
        Style::Normal => normal::logout(cfg_path),
        Style::Json => json::logout(cfg_path),
    }
}

pub fn user_info(style: &Style, user_info: &Result<UserInfo>) -> Result<String> {
    match style {
        Style::Colorful => colorful::user_info(user_info),
        Style::Normal => normal::user_info(user_info),
        Style::Json => json::user_info(user_info).into_ok(),
    }
}

pub fn list_ing(
    style: &Style,
    time_style: &TimeStyle,
    ing_with_comment_iter: Result<impl ExactSizeIterator<Item = (IngEntry, Vec<IngCommentEntry>)>>,
    align: bool,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::list_ing(time_style, ing_with_comment_iter, align),
        Style::Normal => normal::list_ing(time_style, ing_with_comment_iter, align),
        Style::Json => json::list_ing(ing_with_comment_iter),
    }
}

pub fn publish_ing(style: &Style, result: &Result<&String>) -> String {
    match style {
        Style::Colorful => colorful::fmt_result(result),
        Style::Normal => normal::fmt_result(result),
        Style::Json => json::fmt_result(result),
    }
}

pub fn comment_ing(style: &Style, result: &Result<&String>) -> String {
    match style {
        Style::Colorful => colorful::fmt_result(result),
        Style::Normal => normal::fmt_result(result),
        Style::Json => json::fmt_result(result),
    }
}

pub fn show_post(style: &Style, entry: &Result<PostEntry>) -> Result<String> {
    match style {
        Style::Colorful => colorful::show_post(entry),
        Style::Normal => normal::show_post(entry),
        Style::Json => json::show_post(entry).into_ok(),
    }
}

pub fn show_post_meta(
    style: &Style,
    time_style: &TimeStyle,
    entry: &Result<PostEntry>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::show_post_meta(time_style, entry),
        Style::Normal => normal::show_post_meta(time_style, entry),
        Style::Json => json::show_post_meta(entry).into_ok(),
    }
}

pub fn show_post_comment(
    style: &Style,
    time_style: &TimeStyle,
    comment_iter: Result<impl ExactSizeIterator<Item = PostCommentEntry>>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::show_post_comment(time_style, comment_iter),
        Style::Normal => normal::show_post_comment(time_style, comment_iter),
        Style::Json => json::show_post_comment(comment_iter),
    }
}

pub fn list_post(
    style: &Style,
    result: Result<(impl ExactSizeIterator<Item = PostEntry>, usize)>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::list_post(result),
        Style::Normal => normal::list_post(result),
        Style::Json => json::list_post(result).into_ok(),
    }
}

pub fn delete_post(style: &Style, result: &Result<usize>) -> String {
    match style {
        Style::Colorful => colorful::fmt_result(result),
        Style::Normal => normal::fmt_result(result),
        Style::Json => json::fmt_result(result),
    }
}

pub fn search_post(
    style: &Style,
    result: Result<(impl ExactSizeIterator<Item = usize>, usize)>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::search_post(result),
        Style::Normal => normal::search_post(result),
        Style::Json => json::search_post(result).into_ok(),
    }
}

pub fn create_post(style: &Style, result: &Result<usize>) -> String {
    match style {
        Style::Colorful => colorful::fmt_result(result),
        Style::Normal => normal::fmt_result(result),
        Style::Json => json::fmt_result(result),
    }
}

pub fn update_post(style: &Style, result: &Result<usize>) -> String {
    match style {
        Style::Colorful => colorful::fmt_result(result),
        Style::Normal => normal::fmt_result(result),
        Style::Json => json::fmt_result(result),
    }
}

pub fn list_news(
    style: &Style,
    time_style: &TimeStyle,
    news_iter: Result<impl ExactSizeIterator<Item = NewsEntry>>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::list_news(time_style, news_iter),
        Style::Normal => normal::list_news(time_style, news_iter),
        Style::Json => json::list_news(news_iter),
    }
}

pub fn list_fav(
    style: &Style,
    time_style: &TimeStyle,
    fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::list_fav(time_style, fav_iter),
        Style::Normal => normal::list_fav(time_style, fav_iter),
        Style::Json => json::list_fav(fav_iter),
    }
}
