use crate::args::parser::{get_skip, get_take};
use crate::args::{cmd, Args, Cmd};
use crate::infra::option::IntoOption;

pub fn list_fav(args: &Args) -> Option<(usize, usize)> {
    match args {
        Args {
            cmd: Some(Cmd::Fav(cmd::fav::Opt { list: true })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (skip, take)
        }
        _ => return None,
    }
    .into_some()
}
