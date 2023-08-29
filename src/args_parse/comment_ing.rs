use crate::args::Args;
use crate::auth::session;
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn parse_comment_ing(args: &Args) -> Option<Result<(String, &String, usize)>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: false,
            ing_list: None,
            pub_ing: None,
            comment_ing: Some(content),
            id: Some(id),
            with_pat,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, content, *id)),
        _ => return None,
    }
    .into_some()
}
