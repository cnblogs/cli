use crate::api::ing::IngType;
use crate::args::cmd::ing::{QueryIng, CreateIng};
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

#[allow(unused)]
pub fn query(args: &Args) -> Option<QueryIng> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd:
                        Some(cmd::ing::Cmd::Query(QueryIng {
                            r#type,
                            page_index,
                            page_size,
                            tag,
                            id,
                        })),
                    publish: None,
                    comment: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => QueryIng {
            r#type: r#type.clone(),
            page_index: page_index.clone(),
            page_size: page_size.clone(),
            tag: tag.clone(),
            id: id.clone(),
        },
        _ => return None,
    }
    .wrap_some()
}


#[allow(unused)]
pub fn create_ing(args: &Args) -> Option<CreateIng> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd:
                        Some(cmd::ing::Cmd::Create(CreateIng {
                            content,
                            private,
                            lucky,
                            tag,
                        })),
                    publish: None,
                    comment: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => CreateIng {
            content: content.clone(),
            private:private.clone(),
            lucky: lucky.clone(),
            tag: tag.clone(),
        },
        _ => return None,
    }
    .wrap_some()
}


#[allow(unused)]
pub fn delete(args: &Args) -> Option<Vec<u64>> {
    match args {
        Args {
            cmd:
                Some(Cmd::Ing(cmd::ing::Opt {
                    cmd:
                        Some(cmd::ing::Cmd::Delete{id}),
                    publish: None,
                    comment: None,
                })),
            id: None,
            rev: _,
            skip,
            take,
            global_opt: _,
        } => id.clone(),
        _ => return None,
    }
    .wrap_some()
}
