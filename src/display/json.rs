use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
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
    let iter: Box<dyn Iterator<Item = &(IngEntry, Vec<IngCommentEntry>)>> = if rev {
        Box::new(ing_list.iter().rev())
    } else {
        Box::new(ing_list.iter())
    };
    let vec = iter.into_iter().collect::<Vec<_>>();
    let json = json::serialize(vec)?;
    print!("{}", json);
    ().into_ok()
}

pub fn publish_ing(result: &Result<&String>) {
    let json = match result {
        Ok(content) => json!({
            "ok": true,
            "msg": content
        }),
        Err(e) => json!({
            "ok": false,
            "msg": e.to_string()
        }),
    };
    println!("{}", json)
}

pub fn comment_ing(result: &Result<&String>) {
    let json = match result {
        Ok(content) => json!({
            "ok": true,
            "msg": content
        }),
        Err(e) => json!({
            "ok": false,
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
    let iter: Box<dyn Iterator<Item = &PostEntry>> = if rev {
        Box::new(entry_list.iter().rev())
    } else {
        Box::new(entry_list.iter())
    };
    let vec = iter.into_iter().collect::<Vec<_>>();
    let json = json!({
       "listed_count": total_count,
       "total_count": vec.len(),
       "entry_list": vec,
    });
    print!("{}", json);
}
