use crate::api::ing::IngType;
use crate::args::parser::{get_skip, get_take};
use crate::args::{cmd, Args, Cmd};
use crate::infra::option::WrapOption;

pub fn list_ing(args: &Args) -> Option<(usize, usize, IngType, bool)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd: Some(cmd::ing::Cmd::List { r#type, align }),
                    publish: None,
                    comment: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => {
            let skip = get_skip(skip);
            let take = get_take(take);
            let r#type = r#type.clone().unwrap_or(IngType::Public);
            (skip, take, r#type, *align)
        }
        _ => return None,
    }
    .wrap_some()
}

pub fn publish_ing(args: &Args) -> Option<&String> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd: None,
                    publish: Some(content),
                    comment: None,
                })),
            id: None,
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
        } => content,
        _ => return None,
    }
    .wrap_some()
}

pub fn comment_ing(args: &Args) -> Option<(&String, usize)> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd: None,
                    publish: None,
                    comment: Some(content),
                })),
            id: Some(id),
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
        } => (content, *id),
        _ => return None,
    }
    .wrap_some()
}
