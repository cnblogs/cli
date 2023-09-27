use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::display::json::fmt_err;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::json;

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
