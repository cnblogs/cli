use crate::api::news::get_list::NewsEntry;
use crate::display::json::{fmt_err, fmt_ok};
use anyhow::Result;

pub fn list_news(news_iter: Result<impl ExactSizeIterator<Item = NewsEntry>>) -> String {
    let news_iter = match news_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e),
    };

    let vec = news_iter.collect::<Vec<_>>();

    fmt_ok(vec)
}
