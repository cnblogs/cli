use crate::args::{cmd, Args, Cmd};
use crate::infra::option::IntoOption;

fn get_skip(skip: &Option<usize>) -> usize {
    skip.unwrap_or(0)
}

fn get_take(take: &Option<usize>) -> usize {
    take.unwrap_or(8).min(100)
}

pub const fn no_operation(args: &Args) -> bool {
    matches!(
        args,
        Args {
            cmd: None,
            id: None,
            with_pat: None,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        }
    )
}

pub const fn user_info(args: &Args) -> bool {
    matches!(
        args,
        Args {
            cmd: Some(Cmd::User(cmd::user::Opt {
                login: None,
                logout: false,
                info: true,
            })),
            id: None,
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        }
    )
}

pub fn publish_ing(args: &Args) -> Option<&String> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: false,
                    publish: Some(content),
                    comment: None,
                })),
            id: None,
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => content,
        _ => return None,
    }
    .into_some()
}

pub fn login(args: &Args) -> Option<&String> {
    match args {
        Args {
            cmd:
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
            fail_on_error: _,
        } => pat,
        _ => return None,
    }
    .into_some()
}

pub const fn logout(args: &Args) -> bool {
    matches!(
        args,
        Args {
            cmd: Some(Cmd::User(cmd::user::Opt {
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
            fail_on_error: _,
        }
    )
}

pub fn list_ing(args: &Args) -> Option<(usize, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: true,
                    publish: None,
                    comment: None,
                })),
            id: None,
            with_pat: _,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
            fail_on_error: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (skip, take)
        }
        _ => return None,
    }
    .into_some()
}

pub fn comment_ing(args: &Args) -> Option<(&String, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    list: false,
                    publish: None,
                    comment: Some(content),
                })),
            id: Some(id),
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => (content, *id),
        _ => return None,
    }
    .into_some()
}

pub fn show_post(args: &Args) -> Option<usize> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: true,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => *id,
        _ => return None,
    }
    .into_some()
}

pub fn show_post_meta(args: &Args) -> Option<usize> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: true,
                    list: false,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => *id,
        _ => return None,
    }
    .into_some()
}

pub fn list_post(args: &Args) -> Option<(usize, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: true,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: None,
            with_pat: _,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
            fail_on_error: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (skip, take)
        }
        _ => return None,
    }
    .into_some()
}

pub fn delete_post(args: &Args) -> Option<usize> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: true,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            with_pat: _,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => *id,
        _ => return None,
    }
    .into_some()
}

pub fn search_post(args: &Args) -> Option<(&String, usize, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: Some(keyword),
                    cmd: None,
                })),
            id: None,
            with_pat: _,
            rev: _,
            skip,
            take,
            debug: _,
            style: _,
            fail_on_error: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (keyword, skip, take)
        }
        _ => return None,
    }
    .into_some()
}

pub fn create_post(args: &Args) -> Option<(&String, &String, bool)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: None,
                    cmd:
                        Some(cmd::post::Cmd::Create {
                            title,
                            body,
                            publish,
                        }),
                })),
            id: None,
            with_pat: _,
            rev: _,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => (title, body, *publish),
        _ => return None,
    }
    .into_some()
}

// TODO
#[allow(clippy::type_complexity)]
pub fn update_post(
    args: &Args,
) -> Option<(usize, &Option<String>, &Option<String>, &Option<bool>)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    list: false,
                    delete: false,
                    search: None,
                    cmd:
                        Some(cmd::post::Cmd::Update {
                            title,
                            body,
                            publish,
                        }),
                })),
            id: Some(id),
            with_pat: _,
            rev: _,
            skip: None,
            take: None,
            debug: _,
            style: _,
            fail_on_error: _,
        } => (*id, title, body, publish),
        _ => return None,
    }
    .into_some()
}
