use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::post::search_site::SearchResultEntry;
use crate::display::json::{fmt_err, fmt_ok, fmt_result};
use anyhow::Result;
use serde_json::json;

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
    fmt_ok(json)
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
) -> String {
    let comment_iter = match comment_iter {
        Ok(entry) => entry,
        Err(e) => return fmt_err(&e),
    };

    let comment_vec = comment_iter.collect::<Vec<_>>();
    fmt_ok(comment_vec)
}

pub fn search_self_post(result: Result<(impl ExactSizeIterator<Item = usize>, usize)>) -> String {
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
    fmt_ok(json)
}

pub fn search_site_post(
    entry_iter: Result<impl ExactSizeIterator<Item = SearchResultEntry>>,
) -> String {
    let entry_iter = match entry_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e),
    };

    let entry_vec = entry_iter.collect::<Vec<SearchResultEntry>>();
    fmt_ok(entry_vec)
}
