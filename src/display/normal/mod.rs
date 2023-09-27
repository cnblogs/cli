pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use anyhow::Result;
use std::fmt::Display;

#[inline]
pub fn fmt_err(e: &anyhow::Error) -> String {
    format!("Err: {}", e)
}

#[inline]
pub fn fmt_result<T: Display>(result: &Result<T>) -> String {
    match result {
        Ok(t) => format!("Ok: {}", t),
        Err(e) => fmt_err(e),
    }
}
