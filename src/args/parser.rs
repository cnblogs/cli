use crate::api::auth::session;
use crate::args::{sub_cmd, Args, SubCmds};
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

pub fn no_operation(args: &Args) -> bool {
    matches!(
        args,
        Args {
            command: None,
            id: None,
            with_pat: None,
            rev: false,
            skip: 0,
            ..
        }
    )
}

pub fn user_info(args: &Args) -> Option<Result<String>> {
    match args {
        Args {
            command:
                Some(SubCmds::User(sub_cmd::User {
                    login: None,
                    logout: false,
                    info: true,
                })),
            id: None,
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat.clone().or_eval_result(session::get_pat),
        _ => return None,
    }
    .into_some()
}

pub fn publish_ing(args: &Args) -> Option<Result<(String, &String)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Ing(sub_cmd::Ing {
                    list: None,
                    publish: Some(content),
                    comment: None,
                })),
            id: None,
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, content)),
        _ => return None,
    }
    .into_some()
}

pub fn login(args: &Args) -> Option<&String> {
    match args {
        Args {
            command:
                Some(SubCmds::User(sub_cmd::User {
                    login: Some(pat),
                    logout: false,
                    info: false,
                })),
            id: None,
            with_pat: None,
            rev: false,
            skip: 0,
            ..
        } => pat,
        _ => return None,
    }
    .into_some()
}

pub fn logout(args: &Args) -> bool {
    matches!(
        args,
        Args {
            command: Some(SubCmds::User(sub_cmd::User {
                login: None,
                logout: true,
                info: false,
            })),
            id: None,
            with_pat: None,
            rev: false,
            skip: 0,
            ..
        }
    )
}

pub fn list_ing(args: &Args) -> Option<Result<(String, usize, usize, bool)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Ing(sub_cmd::Ing {
                    list: Some(length),
                    publish: None,
                    comment: None,
                })),
            id: None,
            with_pat,
            rev,
            skip,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, *skip, (*length).min(100), *rev)),
        _ => return None,
    }
    .into_some()
}

pub fn comment_ing(args: &Args) -> Option<Result<(String, &String, usize)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Ing(sub_cmd::Ing {
                    list: None,
                    publish: None,
                    comment: Some(content),
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, content, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Post(sub_cmd::Post {
                    show: true,
                    show_meta: false,
                    list: None,
                    delete: false,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post_meta(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Post(sub_cmd::Post {
                    show: false,
                    show_meta: true,
                    list: None,
                    delete: false,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn list_post(args: &Args) -> Option<Result<(String, usize, usize, bool)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Post(sub_cmd::Post {
                    show: false,
                    show_meta: false,
                    list: Some(length),
                    delete: false,
                })),
            id: None,
            with_pat,
            rev,
            skip,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, *skip, (*length).min(100), *rev)),
        _ => return None,
    }
    .into_some()
}

pub fn delete_post(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(SubCmds::Post(sub_cmd::Post {
                    show: false,
                    show_meta: false,
                    list: None,
                    delete: true,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: 0,
            ..
        } => with_pat
            .clone()
            .or_eval_result(session::get_pat)
            .map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}
