use crate::args::Args;
use crate::auth::session;
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn user_info(args: &Args) -> Option<Result<String>> {
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
            show_post: false,
            show_post_meta: false,
        } => with_pat.clone().bind_result(session::get_pat),
        _ => return None,
    }
    .into_some()
}

pub fn pub_ing(args: &Args) -> Option<Result<(String, &String)>> {
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
            show_post: false,
            show_post_meta: false,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, content)),
        _ => return None,
    }
    .into_some()
}

pub fn login(args: &Args) -> Option<&String> {
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
            show_post: false,
            show_post_meta: false,
        } => pat,
        _ => return None,
    }
    .into_some()
}

pub fn logout(args: &Args) -> bool {
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
            show_post: false,
            show_post_meta: false,
        }
    )
}

pub fn ing_list(args: &Args) -> Option<Result<(String, usize)>> {
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
            show_post: false,
            show_post_meta: false,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, (*length).min(100))),
        _ => return None,
    }
    .into_some()
}

pub fn comment_ing(args: &Args) -> Option<Result<(String, &String, usize)>> {
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
            show_post: false,
            show_post_meta: false,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, content, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: false,
            ing_list: None,
            pub_ing: None,
            comment_ing: None,
            id: Some(id),
            with_pat,
            show_post: true,
            show_post_meta: false,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post_meta(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            login: None,
            logout: false,
            user_info: false,
            ing_list: None,
            pub_ing: None,
            comment_ing: None,
            id: Some(id),
            with_pat,
            show_post: false,
            show_post_meta: true,
        } => with_pat
            .clone()
            .bind_result(session::get_pat)
            .map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}
