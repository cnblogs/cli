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
        Style::Colorful => colorful::user::login(cfg_path),
        Style::Normal => normal::user::login(cfg_path),
        Style::Json => json::user::login(cfg_path),
    }
}

pub fn logout(style: &Style, cfg_path: &Result<PathBuf>) -> String {
    match style {
        Style::Colorful => colorful::user::logout(cfg_path),
        Style::Normal => normal::user::logout(cfg_path),
        Style::Json => json::user::logout(cfg_path),
    }
}

pub fn user_info(style: &Style, user_info: &Result<UserInfo>) -> Result<String> {
    match style {
        Style::Colorful => colorful::user::user_info(user_info),
        Style::Normal => normal::user::user_info(user_info),
        Style::Json => json::user::user_info(user_info).into_ok(),
    }
}

pub fn list_ing(
    style: &Style,
    time_style: &TimeStyle,
    ing_with_comment_iter: Result<impl ExactSizeIterator<Item = (IngEntry, Vec<IngCommentEntry>)>>,
    align: bool,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::ing::list_ing(time_style, ing_with_comment_iter, align),
        Style::Normal => normal::ing::list_ing(time_style, ing_with_comment_iter, align),
        Style::Json => json::ing::list_ing(ing_with_comment_iter).into_ok(),
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
        Style::Colorful => colorful::post::show_post(entry),
        Style::Normal => normal::post::show_post(entry),
        Style::Json => json::post::show_post(entry).into_ok(),
    }
}

pub fn list_post(
    style: &Style,
    result: Result<(impl ExactSizeIterator<Item = PostEntry>, usize)>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::post::list_post(result),
        Style::Normal => normal::post::list_post(result),
        Style::Json => json::post::list_post(result).into_ok(),
    }
}

pub fn show_post_meta(
    style: &Style,
    time_style: &TimeStyle,
    entry: &Result<PostEntry>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::post::show_post_meta(time_style, entry),
        Style::Normal => normal::post::show_post_meta(time_style, entry),
        Style::Json => json::post::show_post_meta(entry).into_ok(),
    }
}

pub fn show_post_comment(
    style: &Style,
    time_style: &TimeStyle,
    comment_iter: Result<impl ExactSizeIterator<Item = PostCommentEntry>>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::post::show_post_comment(time_style, comment_iter),
        Style::Normal => normal::post::show_post_comment(time_style, comment_iter),
        Style::Json => json::post::show_post_comment(comment_iter).into_ok(),
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
        Style::Colorful => colorful::post::search_post(result),
        Style::Normal => normal::post::search_post(result),
        Style::Json => json::post::search_post(result).into_ok(),
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
        Style::Colorful => colorful::news::list_news(time_style, news_iter),
        Style::Normal => normal::news::list_news(time_style, news_iter),
        Style::Json => json::news::list_news(news_iter).into_ok(),
    }
}

pub fn list_fav(
    style: &Style,
    time_style: &TimeStyle,
    fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>,
) -> Result<String> {
    match style {
        Style::Colorful => colorful::fav::list_fav(time_style, fav_iter),
        Style::Normal => normal::fav::list_fav(time_style, fav_iter),
        Style::Json => json::fav::list_fav(fav_iter).into_ok(),
    }
}
