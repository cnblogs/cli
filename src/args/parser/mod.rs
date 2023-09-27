pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use crate::args::{Args, GlobalOpt};

fn get_skip(skip: &Option<usize>) -> usize {
    skip.unwrap_or(0)
}

fn get_take(take: &Option<usize>) -> usize {
    take.unwrap_or(8).min(100)
}

pub const fn no_operation(args: &Args) -> bool {
    matches!(
        args,
        Args {
            cmd: None,
            id: None,
            rev: false,
            skip: None,
            take: None,
            global_opt: GlobalOpt { with_pat: None, .. }
        }
    )
}
