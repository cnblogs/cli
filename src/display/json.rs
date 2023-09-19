use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::api::news::get_list::NewsEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::iter::IteratorExt;
use crate::infra::json;
use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;

pub fn login(cfg_path: &Result<PathBuf>) {
    let json = cfg_path.as_ref().map(|pb| json!({"cfg_path":pb}));
    println_result(&json);
}

pub fn logout(cfg_path: &Result<PathBuf>) {
    let json = cfg_path.as_ref().map(|pb| json!({"cfg_path":pb}));
    println_result(&json);
}

pub fn user_info(info: &Result<UserInfo>) {
    println_result(info);
}

pub fn list_ing(ing_with_comment_list: &Result<Vec<(IngEntry, Vec<IngCommentEntry>)>>, rev: bool) {
    let ing_with_comment_list = match ing_with_comment_list {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    let json_vec = ing_with_comment_list
        .iter()
        .dyn_rev(rev)
        .map(|(entry, comment_list)| {
            json!({
                "entry": entry,
                "comment_list": comment_list
            })
        })
        .collect::<Vec<_>>();

    let json = json::serialize(json_vec).expect("Can not serialize json_vec");
    print!("{}", json);
}

pub fn show_post(entry: &Result<PostEntry>) {
    let json = entry.as_ref().map(|entry| {
        json!({
            "title": entry.title,
            "body": entry.body
        })
    });
    println_result(&json);
}

pub fn show_post_meta(entry: &Result<PostEntry>) {
    println_result(entry);
}

pub fn list_post(result: &Result<(Vec<PostEntry>, usize)>, rev: bool) {
    let (entry_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    let vec = entry_list.iter().dyn_rev(rev).collect::<Vec<_>>();
    let json = json!({
       "listed_count": vec.len(),
       "total_count": total_count,
       "entry_list": vec,
    });
    print!("{}", json);
}

pub fn search_post(result: &Result<(Vec<usize>, usize)>, rev: bool) {
    let (id_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    let id_list = id_list.iter().dyn_rev(rev).collect::<Vec<&usize>>();
    let json = json!({
       "listed_count": id_list.len(),
       "total_count": total_count,
       "id_list": id_list,
    });

    println!("{}", json);
}

pub fn println_err(e: &anyhow::Error) {
    let json = json!({
        "is_ok": false,
        "msg": e.to_string()
    });
    println!("{}", json)
}

pub fn println_result<T: Serialize, E: ToString>(result: &Result<T, E>) {
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
    println!("{}", json)
}

pub fn list_news(news_list: &Result<Vec<NewsEntry>>, rev: bool) {
    let news_list = match news_list {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    let vec = news_list.iter().dyn_rev(rev).collect::<Vec<_>>();

    let json =
        json::serialize(vec.clone()).unwrap_or_else(|_| panic!("Can not serialize: {:?}", vec));
    print!("{}", json);
}
