#![feature(try_blocks)]
#![feature(if_let_guard)]

use crate::args::parser;
use crate::args::Args;
use crate::auth::session;
use crate::infra::result::IntoResult;
use crate::ing::{Ing, IngType};
use crate::post::Post;
use crate::user::User;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use colored::Colorize;
use crate::args::parser::no_option;

pub mod api_base;
pub mod args;
pub mod auth;
pub mod infra;
pub mod ing;
pub mod post;
pub mod user;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    match args {
        _ if let Some(pat) = parser::login(&args) => session::login(pat),
        _ if parser::logout(&args) => session::logout(),
        _ if let Some(pat) = parser::user_info(&args) => {
            let user_info = User::new(pat?).get_info().await?;
            println!("{}", user_info);
            ().into_ok()
        }
        _ if let Some(pair) = parser::list_ing(&args) => {
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
        _ if let Some(pair) = parser::pub_ing(&args) => {
            let (pat, content) = pair?;
            let result = Ing::new(pat).publish(content.clone()).await;

            if result.is_ok() {
                println!("{}: {}", "Published".green(), content);
            } else {
                println!("{}: {}", "Error".red(), result.unwrap_err());
            }

            ().into_ok()
        }
        _ if let Some(triple) = parser::comment_ing(&args) => {
            let (pat, content, id) = triple?;
            let result = Ing::new(pat).comment(id, content.clone(), None, None).await;

            if result.is_ok() {
                println!("{}: {}", "Commented".green(), content);
            } else {
                println!("{}: {}", "Error".red(), result.unwrap_err());
            }

            ().into_ok()
        }
        _ if let Some(pair) = parser::show_post(&args) => {
            let (pat, id) = pair?;

            let post_entry = Post::new(pat).get_post(id).await?;

            post_entry.display_title_body();

            ().into_ok()
        }
        _ if let Some(pair) = parser::show_post_meta(&args) => {
            let (pat, id) = pair?;

            let post_entry = Post::new(pat).get_post(id).await?;

            post_entry.display_meta()
        }

        _ if no_option(&args) => {
            Args::command().print_help()?;
            ().into_ok()
        }

        _ => {
            println!("Invalid usage, try 'cnb --help' for more information.");
            ().into_ok()
        }
    }?;
    //println!("{:?}", args);
    ().into_ok()
}
