#![feature(try_blocks)]

use crate::args::Args;
use crate::auth::session;
use crate::infra::result::IntoResult;
use crate::ing::{Ing, IngType};
use crate::user::User;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use colored::Colorize;
use tokio;

mod api_base;
mod args;
mod auth;
mod infra;
mod ing;
mod user;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    match args {
        Args {
            login: Some(ref pat),
            ..
        } => session::login(&pat),
        Args { logout: true, .. } => session::logout(),
        Args {
            user_info: true, ..
        } => {
            let pat = session::get_pat()?;
            let user_info = User::new(pat).get_info().await?;
            println!("{}", user_info);
            ().into_ok()
        }
        Args {
            ing_list: Some(length),
            ..
        } => {
            let length = length.min(100);
            let pat = session::get_pat()?;
            let ing_type = IngType::Public;
            let ing_vec = Ing::new(pat).get_list(1, length, ing_type).await?;

            ing_vec.iter().for_each(|(ing, comment_list)| {
                println!("{}", ing);
                comment_list.into_iter().for_each(|c| println!("{}", c));
                println!();
            });

            ().into_ok()
        }
        Args {
            pub_ing: Some(content),
            ..
        } => {
            let pat = session::get_pat()?;
            let result = Ing::new(pat).publish(content.clone()).await;

            if result.is_ok() {
                println!("{}: {}", "Published".green(), content);
            } else {
                println!("{}: {}", "Error".red(), result.unwrap_err());
            }

            ().into_ok()
        }
        Args {
            id: Some(id),
            comment_ing: Some(content),
            ..
        } => {
            let pat = session::get_pat()?;
            let result = Ing::new(pat).comment(id, content.clone(), None, None).await;

            if result.is_ok() {
                println!("{}: {}", "Commented".green(), content);
            } else {
                println!("{}: {}", "Error".red(), result.unwrap_err());
            }

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
