pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use anyhow::Result;
use serde::Serialize;
use serde_json::json;

#[inline]
pub fn fmt_err(e: &anyhow::Error) -> String {
    let json = json!({
        "is_ok": false,
        "msg": e.to_string()
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
