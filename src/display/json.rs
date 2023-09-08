use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
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

pub fn list_ing(ing_list: &Result<Vec<(IngEntry, Vec<IngCommentEntry>)>>, rev: bool) {
    if let Err(e) = ing_list {
        println_err(e);
        return;
    }

    let vec = ing_list
        .as_ref()
        .unwrap()
        .iter()
        .dyn_rev(rev)
        .map(|(entry, comment_list)| {
            json!({
                "entry": entry,
                "comment_list": comment_list
            })
        })
        .collect::<Vec<_>>();
    let json = json::serialize(vec).unwrap();
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
    if let Err(e) = result {
        println_err(e);
        return;
    }
    let (entry_list, total_count) = result.as_ref().unwrap();
    let vec = entry_list.iter().dyn_rev(rev).collect::<Vec<_>>();
    let json = json!({
       "listed_count": vec.len(),
       "total_count": total_count,
       "entry_list": vec,
    });
    print!("{}", json);
}

pub fn search_post(result: &Result<(Vec<usize>, usize)>, rev: bool) {
    if let Err(e) = result {
        println_err(e);
        return;
    }

    let (id_list, total_count) = result.as_ref().unwrap();
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
