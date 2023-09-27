use crate::api::user::info::UserInfo;
use crate::display::json::fmt_result;
use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;

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
