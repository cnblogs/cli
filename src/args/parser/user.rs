use crate::args::{cmd, Args, Cmd, GlobalOpt};
use crate::infra::option::IntoOption;

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
            rev: false,
            skip: None,
            take: None,
            global_opt: GlobalOpt { with_pat: None, .. },
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
            rev: false,
            skip: None,
            take: None,
            global_opt: GlobalOpt { with_pat: None, .. },
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
            rev: false,
            skip: None,
            take: None,
            global_opt: _,
        }
    )
}
