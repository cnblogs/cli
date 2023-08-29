use crate::args::Args;
use crate::auth::session;
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn parse_pub_ing(args: &Args) -> Option<Result<(String, &String)>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: false,
            ing_list: None,
            pub_ing: Some(content),
            comment_ing: None,
            id: None,
            with_pat,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, content)),
        _ => return None,
    }
    .into_some()
}
