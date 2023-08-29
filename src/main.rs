#![feature(try_blocks)]
#![feature(if_let_guard)]

use crate::args::Args;
use crate::args_parse::ing_list::parse_ing_list;
use crate::args_parse::login::parse_login;
use crate::args_parse::logout::parse_logout;
use crate::args_parse::pub_ing::parse_pub_ing;
use crate::args_parse::user_info::parse_user_info;
use crate::auth::session;
use crate::infra::result::IntoResult;
use crate::ing::{Ing, IngType};
use crate::user::User;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use colored::Colorize;
use crate::args_parse::comment_ing::parse_comment_ing;

pub mod api_base;
pub mod args;
pub mod args_parse;
pub mod auth;
pub mod infra;
pub mod ing;
pub mod user;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    match args {
        _ if let Some(pat) = parse_login(&args) => session::login(pat),
        _ if parse_logout(&args) => session::logout(),
        _ if let Some(pat) = parse_user_info(&args) => {
            let user_info = User::new(pat?).get_info().await?;
            println!("{}", user_info);
            ().into_ok()
        }
        _ if let Some(pair) = parse_ing_list(&args) => {
            let (pat, length) = pair?;
            let ing_type = IngType::Public;
            let ing_vec = Ing::new(pat).get_list(1, length, ing_type).await?;

            ing_vec.iter().for_each(|(ing, comment_list)| {
                println!("{}", ing);
                comment_list.iter().for_each(|c| println!("{}", c));
                println!();
            });

            ().into_ok()
        }
        _ if let Some(pair) = parse_pub_ing(&args) => {
            let (pat, content) = pair?;
            let result = Ing::new(pat).publish(content.clone()).await;

            if result.is_ok() {
                println!("{}: {}", "Published".green(), content);
            } else {
                println!("{}: {}", "Error".red(), result.unwrap_err());
            }

            ().into_ok()
        }
        _ if let Some(triple) = parse_comment_ing(&args) => {
            let (pat, content, id) = triple?;
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
