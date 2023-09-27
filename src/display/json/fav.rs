use crate::api::fav::get_list::FavEntry;
use crate::display::json::{fmt_err, fmt_ok};
use anyhow::Result;

pub fn list_fav(fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>) -> String {
    let fav_iter = match fav_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e),
    };

    let vec = fav_iter.collect::<Vec<_>>();

    fmt_ok(vec)
}
