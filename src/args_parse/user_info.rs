use crate::args::Args;
use crate::auth::session;
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn parse_user_info(args: &Args) -> Option<Result<String>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: true,
            ing_list: None,
            pub_ing: None,
            comment_ing: None,
            id: None,
            with_pat,
        } => with_pat.clone().bind_result(session::get_pat),
        _ => return None,
    }
    .into_some()
}
