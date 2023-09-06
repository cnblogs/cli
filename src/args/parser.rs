use crate::api::auth::session;
use crate::args::{cmd, Args, Cmd};
use crate::infra::option::{IntoOption, OptionExt};
use anyhow::Result;

fn get_skip(skip: &Option<usize>) -> usize {
    skip.unwrap_or(0)
}

fn get_take(take: &Option<usize>) -> usize {
    take.unwrap_or(8).min(100)
}

fn get_pat(pat: &Option<String>) -> Result<String> {
    pat.clone().or_eval_result(session::get_pat)
}

pub fn no_operation(args: &Args) -> bool {
    matches!(
        args,
        Args {
            command: None,
            id: None,
            with_pat: None,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        }
    )
}

pub fn user_info(args: &Args) -> Option<Result<String>> {
    match args {
        Args {
            command:
                Some(Cmd::User(cmd::user::Opt {
                    login: None,
                    logout: false,
                    info: true,
                })),
            id: None,
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat),
        _ => return None,
    }
    .into_some()
}

pub fn publish_ing(args: &Args) -> Option<Result<(String, &String)>> {
    match args {
        Args {
            command:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: false,
                    publish: Some(content),
                    comment: None,
                })),
            id: None,
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat).map(|pat| (pat, content)),
        _ => return None,
    }
    .into_some()
}

pub fn login(args: &Args) -> Option<&String> {
    match args {
        Args {
            command:
                Some(Cmd::User(cmd::user::Opt {
                    login: Some(pat),
                    logout: false,
                    info: false,
                })),
            id: None,
            with_pat: None,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => pat,
        _ => return None,
    }
    .into_some()
}

pub fn logout(args: &Args) -> bool {
    matches!(
        args,
        Args {
            command: Some(Cmd::User(cmd::user::Opt {
                login: None,
                logout: true,
                info: false,
            })),
            id: None,
            with_pat: None,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        }
    )
}

pub fn list_ing(args: &Args) -> Option<Result<(String, usize, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: true,
                    publish: None,
                    comment: None,
                })),
            id: None,
            with_pat,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            get_pat(with_pat).map(|pat| (pat, skip, take))
        }
        _ => return None,
    }
    .into_some()
}

pub fn comment_ing(args: &Args) -> Option<Result<(String, &String, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: false,
                    publish: None,
                    comment: Some(content),
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat).map(|pat| (pat, content, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: true,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: None,
                    create: None,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat).map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn show_post_meta(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: true,
                    list: false,
                    delete: false,
                    search: None,
                    create: None,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat).map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn list_post(args: &Args) -> Option<Result<(String, usize, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: true,
                    delete: false,
                    search: None,
                    create: None,
                })),
            id: None,
            with_pat,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            get_pat(with_pat).map(|pat| (pat, skip, take))
        }
        _ => return None,
    }
    .into_some()
}

pub fn delete_post(args: &Args) -> Option<Result<(String, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: true,
                    search: None,
                    create: None,
                })),
            id: Some(id),
            with_pat,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } => get_pat(with_pat).map(|pat| (pat, *id)),
        _ => return None,
    }
    .into_some()
}

pub fn search_post(args: &Args) -> Option<Result<(String, &String, usize, usize)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: Some(keyword),
                    create: None,
                })),
            id: None,
            with_pat,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            get_pat(with_pat).map(|pat| (pat, keyword, skip, take))
        }
        _ => return None,
    }
    .into_some()
}

pub fn create_post(args: &Args) -> Option<Result<(String, &String, &String)>> {
    match args {
        Args {
            command:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: None,
                    create: Some(vec),
                })),
            id: None,
            with_pat,
            rev: _,
            skip: None,
            take: None,
            debug: _,
            style: _,
        } if vec.len() == 2 => {
            let title = &vec[0];
            let body = &vec[1];
            get_pat(with_pat).map(|pat| (pat, title, body))
        }
        _ => return None,
    }
    .into_some()
}
