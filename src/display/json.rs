use crate::api::fav::get_list::FavEntry;
use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::api::news::get_list::NewsEntry;
use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;

#[inline]
pub fn fmt_err(e: &anyhow::Error) -> String {
    let json = json!({
        "is_ok": false,
        "msg": e.to_string()
    });
    json.to_string()
}

pub fn login(cfg_path: &Result<PathBuf>) -> String {
    let json = cfg_path.as_ref().map(|pb| json!({"cfg_path":pb}));
    fmt_result(&json)
}

pub fn logout(cfg_path: &Result<PathBuf>) -> String {
    let json = cfg_path.as_ref().map(|pb| json!({"cfg_path":pb}));
    fmt_result(&json)
}

pub fn user_info(info: &Result<UserInfo>) -> String {
    fmt_result(info)
}

pub fn list_ing(
    ing_with_comment_list: Result<impl ExactSizeIterator<Item = (IngEntry, Vec<IngCommentEntry>)>>,
) -> Result<String> {
    let ing_with_comment_list = match ing_with_comment_list {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let json_vec = ing_with_comment_list
        .map(|(entry, comment_list)| {
            json!({
                "entry": entry,
                "comment_list": comment_list
            })
        })
        .collect::<Vec<_>>();

    json::serialize(json_vec)
}

pub fn show_post(entry: &Result<PostEntry>) -> String {
    let json = entry.as_ref().map(|entry| {
        json!({
            "title": entry.title,
            "body": entry.body
        })
    });
    fmt_result(&json)
}

pub fn show_post_meta(entry: &Result<PostEntry>) -> String {
    fmt_result(entry)
}

pub fn show_post_comment(
    comment_iter: Result<impl ExactSizeIterator<Item = PostCommentEntry>>,
) -> Result<String> {
    let comment_iter = match comment_iter {
        Ok(entry) => entry,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let comment_vec = comment_iter.collect::<Vec<_>>();
    json::serialize(comment_vec)
}

pub fn list_post(result: Result<(impl ExactSizeIterator<Item = PostEntry>, usize)>) -> String {
    let (entry_iter, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(&e),
    };

    let vec = entry_iter.collect::<Vec<_>>();
    let json = json!({
       "listed_count": vec.len(),
       "total_count": total_count,
       "entry_list": vec,
    });
    json.to_string()
}

pub fn search_post(result: Result<(impl ExactSizeIterator<Item = usize>, usize)>) -> String {
    let (id_iter, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(&e),
    };

    let id_list = id_iter.collect::<Vec<usize>>();
    let json = json!({
       "listed_count": id_list.len(),
       "total_count": total_count,
       "id_list": id_list,
    });
    json.to_string()
}

pub fn fmt_result<T: Serialize, E: ToString>(result: &Result<T, E>) -> String {
    let json = match result {
        Ok(t) => json!({
            "is_ok": true,
            "msg": t
        }),
        Err(e) => json!({
            "is_ok": false,
            "msg": e.to_string()
        }),
    };
    json.to_string()
}

pub fn list_news(news_iter: Result<impl ExactSizeIterator<Item = NewsEntry>>) -> Result<String> {
    let news_iter = match news_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let vec = news_iter.collect::<Vec<_>>();

    json::serialize(vec)
}

pub fn list_fav(fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>) -> Result<String> {
    let fav_iter = match fav_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let vec = fav_iter.collect::<Vec<_>>();

    json::serialize(vec)
}
