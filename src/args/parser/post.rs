use crate::args::cmd::post::{CreateCmd, UpdateCmd};
use crate::args::parser::{get_skip, get_take};
use crate::args::{cmd, Args, Cmd};
use crate::infra::option::WrapOption;

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
    .wrap_some()
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
    .wrap_some()
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
    .wrap_some()
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
    .wrap_some()
}

pub fn search_self_post(args: &Args) -> Option<(&String, usize, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    show_comment: false,
                    list: false,
                    delete: false,
                    cmd:
                        Some(cmd::post::Cmd::Search(cmd::post::SearchCmd {
                            self_keyword: Some(keyword),
                        })),
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
    .wrap_some()
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
    .wrap_some()
}

pub fn create_post(args: &Args) -> Option<&CreateCmd> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    show_comment: false,
                    list: false,
                    delete: false,
                    cmd: Some(cmd::post::Cmd::Create(cmd)),
                })),
            id: None,
            rev: _,
            skip: None,
            take: None,
            global_opt: _,
        } => cmd,
        _ => return None,
    }
    .wrap_some()
}

pub fn update_post(args: &Args) -> Option<(usize, &UpdateCmd)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Post(cmd::post::Opt {
                    show: false,
                    show_meta: false,
                    show_comment: false,
                    list: false,
                    delete: false,
                    cmd: Some(cmd::post::Cmd::Update(cmd)),
                })),
            id: Some(id),
            rev: _,
            skip: None,
            take: None,
            global_opt: _,
        } => (*id, cmd),
        _ => return None,
    }
    .wrap_some()
}
