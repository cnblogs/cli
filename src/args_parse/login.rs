use crate::args::Args;
use crate::infra::option::IntoOption;

pub fn parse_login(args: &Args) -> Option<&String> {
    match args {
        Args {
            login: Some(pat),
            logout: false,
            user_info: false,
            ing_list: None,
            pub_ing: None,
            comment_ing: None,
            id: None,
            with_pat: None,
        } => pat,
        _ => return None,
    }
    .into_some()
}
