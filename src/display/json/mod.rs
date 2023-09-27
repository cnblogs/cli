pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::fmt::Display;

#[inline]
pub fn fmt_ok(t: impl Serialize) -> String {
    let json = json!({
        "is_ok": true,
        "msg": t
    });
    json.to_string()
}

#[inline]
pub fn fmt_err(e: impl ToString) -> String {
    let json = json!({
        "is_ok": false,
        "msg": e.to_string()
    });
    json.to_string()
}

pub fn fmt_result<T: Serialize, E: Display>(result: &Result<T, E>) -> String {
    match result {
        Ok(t) => fmt_ok(t),
        Err(e) => fmt_err(e),
    }
}
