use crate::args::parser::{get_skip, get_take};
use crate::args::{cmd, Args, Cmd};
use crate::infra::option::IntoOption;

pub fn list_post(args: &Args) -> Option<(usize, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    show_comment: false,
                    list: true,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (skip, take)
        }
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
                    show_comment: false,
                    list: false,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
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
                    show_comment: false,
                    list: false,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
        } => *id,
        _ => return None,
    }
    .into_some()
}

pub fn show_post_comment(args: &Args) -> Option<usize> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    show_comment: true,
                    list: false,
                    delete: false,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            rev: _,
            skip: None,
            take: None,
            global_opt: _,
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
                    show_comment: false,
                    list: false,
                    delete: false,
                    search: Some(keyword),
                    cmd: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            (keyword, skip, take)
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
                    show_comment: false,
                    list: false,
                    delete: true,
                    search: None,
                    cmd: None,
                })),
            id: Some(id),
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
        } => *id,
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
                    show_comment: false,
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
            rev: _,
            skip: None,
            take: None,
            global_opt: _,
        } => (title, body, *publish),
        _ => return None,
    }
    .into_some()
}

// TODO: fix warn
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
                    show_comment: false,
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
            rev: _,
            skip: None,
            take: None,
            global_opt: _,
        } => (*id, title, body, publish),
        _ => return None,
    }
    .into_some()
}
