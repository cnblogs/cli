use crate::args::{cmd, Args, Cmd};
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
            with_pat: None,
            rev: false,
            skip: None,
            take: None,
            debug: _,
            style: _,
            time_style: _,
            fail_on_error: _,
            quiet: _,
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
            time_style: _,
            fail_on_error: _,
            quiet: _,
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
            time_style: _,
            fail_on_error: _,
            quiet: _,
        }
    )
}
