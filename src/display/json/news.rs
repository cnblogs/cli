use crate::api::news::get_list::NewsEntry;
use crate::display::json::fmt_err;
use crate::infra::json;
use crate::infra::result::IntoResult;
use anyhow::Result;

pub fn list_news(news_iter: Result<impl ExactSizeIterator<Item = NewsEntry>>) -> Result<String> {
    let news_iter = match news_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    let vec = news_iter.collect::<Vec<_>>();

    json::serialize(vec)
}
