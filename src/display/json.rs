use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::iter::IteratorExt;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;

pub fn login(cfg_path: &PathBuf) {
    let json = json!({"cfg_path":cfg_path});
    println!("{}", json)
}

pub fn logout(cfg_path: &PathBuf) {
    let json = json!({"cfg_path":cfg_path});
    println!("{}", json)
}

pub fn user_info(info: &UserInfo) -> Result<()> {
    let json = json::serialize(info)?;
    print!("{}", json);
    ().into_ok()
}

pub fn list_ing(ing_list: &[(IngEntry, Vec<IngCommentEntry>)], rev: bool) -> Result<()> {
    let vec = ing_list
        .iter()
        .dyn_rev(rev)
        .map(|(entry, comment_list)| {
            json!({
                "entry": entry,
                "comment_list": comment_list
            })
        })
        .collect::<Vec<_>>();
    let json = json::serialize(vec)?;
    print!("{}", json);
    ().into_ok()
}

pub fn publish_ing(result: &Result<&String>) {
    let json = match result {
        Ok(content) => json!({
            "is_ok": true,
            "msg": content
        }),
        Err(e) => json!({
            "is_ok": false,
            "msg": e.to_string()
        }),
    };
    println!("{}", json)
}

pub fn comment_ing(result: &Result<&String>) {
    let json = match result {
        Ok(content) => json!({
            "is_ok": true,
            "msg": content
        }),
        Err(e) => json!({
            "is_ok": false,
            "msg": e.to_string()
        }),
    };
    println!("{}", json)
}

pub fn show_post(entry: &PostEntry) {
    let json = json!({
        "title": entry.title,
        "body": entry.body
    });
    println!("{}", json);
}

pub fn show_post_meta(entry: &PostEntry) -> Result<()> {
    let json = json::serialize(entry)?;
    print!("{}", json);
    ().into_ok()
}

pub fn list_post(entry_list: &[PostEntry], total_count: usize, rev: bool) {
    let vec = entry_list.iter().dyn_rev(rev).collect::<Vec<_>>();
    let json = json!({
       "listed_count": vec.len(),
       "total_count": total_count,
       "entry_list": vec,
    });
    print!("{}", json);
}

pub fn delete_post(result: &Result<usize>) {
    let json = match result {
        Ok(id) => json!({
            "ok": true,
            "msg": id
        }),
        Err(e) => json!({
            "ok": false,
            "msg": e.to_string()
        }),
    };
    println!("{}", json)
}

pub fn search_post(id_list: &[usize], total_count: usize, rev: bool) {
    let id_list = id_list.iter().dyn_rev(rev).collect::<Vec<&usize>>();
    let json = json!({
       "listed_count": id_list.len(),
       "total_count": total_count,
       "id_list": id_list,
    });

    println!("{}", json);
}
