use crate::api::fav::get_list::FavEntry;
use crate::display::json::fmt_err;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;

pub fn list_fav(fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>) -> Result<String> {
    let fav_iter = match fav_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let vec = fav_iter.collect::<Vec<_>>();

    json::serialize(vec)
}
