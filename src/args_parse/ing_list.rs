use crate::args::Args;
use crate::auth::session;
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn parse_ing_list(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: false,
            ing_list: Some(length),
            pub_ing: None,
            comment_ing: None,
            id: None,
            with_pat,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, (*length).min(100))),
        _ => return None,
    }
    .into_some()
}
