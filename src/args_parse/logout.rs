use crate::args::Args;

pub fn parse_logout(args: &Args) -> bool {
    matches!(
        args,
        Args {
            login: None,
            logout: true,
            user_info: false,
            ing_list: None,
            pub_ing: None,
            comment_ing: None,
            id: None,
            with_pat: None,
        }
    )
}
