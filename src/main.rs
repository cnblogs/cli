#![feature(try_blocks)]

use clap::CommandFactory;
use crate::args::Args;
use clap::Parser;
use anyhow::Result;
use tokio;
use crate::auth::session;
use crate::infra::result::IntoResult;

mod args;
mod auth;
mod infra;
mod user;
mod api_base;
//mod ing;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();
    match args {
        Args { login: Some(ref pat), .. } =>
            session::login(&pat),
        Args { logout: true, .. } =>
            session::logout(),
        Args { user_info: true, .. } => {
            let pat = session::get_pat()?;
            let user_info = user::info::UserInfo::get(&pat).await?;
            println!("{}", user_info);
            ().into_ok()
        }

        _ => {
            Args::command().print_help()?;
            ().into_ok()
        }
    }?;
    //println!("{:?}", args);
    ().into_ok()
}
