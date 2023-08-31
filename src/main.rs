#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]

use crate::api::auth::session;
use crate::api::ing::{Ing, IngType};
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::parser;
use crate::args::parser::no_option;
use crate::args::Args;
use crate::infra::result::IntoResult;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;

pub mod api;
pub mod args;
pub mod display;
pub mod infra;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    match args {
        _ if let Some(pat) = parser::login(&args) => {
            let cfg_path = session::login(pat)?;
            display::login(&cfg_path);
            ().into_ok()
        }
        _ if parser::logout(&args) => {
            let cfg_path = session::logout()?;
            display::logout(&cfg_path);
            ().into_ok()
        }
        _ if let Some(pat) = parser::user_info(&args) => {
            let user_info = User::new(pat?).get_info().await?;
            display::user_info(&user_info);
            ().into_ok()
        }
        _ if let Some(pair) = parser::list_ing(&args) => {
            let (pat, skip, take, rev) = pair?;
            let ing_type = IngType::Public;
            let ing_vec = Ing::new(pat).get_list(skip, take, &ing_type).await?;
            display::list_ing(&ing_vec, rev);
            ().into_ok()
        }
        _ if let Some(pair) = parser::pub_ing(&args) => {
            let (pat, content) = pair?;
            let result = Ing::new(pat).publish(content).await;
            display::pub_ing(&result.map(|_| content));
            ().into_ok()
        }
        _ if let Some(triple) = parser::comment_ing(&args) => {
            let (pat, content, id) = triple?;
            let result = Ing::new(pat).comment(id, content.clone(), None, None).await;
            display::comment_ing(&result.map(|_| content));
            ().into_ok()
        }
        _ if let Some(pair) = parser::show_post(&args) => {
            let (pat, id) = pair?;
            let entry = Post::new(pat).get_one(id).await?;
            display::show_post(&entry);
            ().into_ok()
        }
        _ if let Some(pair) = parser::show_post_meta(&args) => {
            let (pat, id) = pair?;
            let entry = Post::new(pat).get_one(id).await?;
            display::show_post_meta(&entry)
        }
        _ if let Some(pair) = parser::list_post(&args) => {
            let (pat, _, length, rev) = pair?;
            let entry_vec = Post::new(pat).get_meta_list(1, length).await?;
            display::list_post(&entry_vec, rev);
            ().into_ok()
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

    if args.debug {
        println!("{:#?}", args);
    }

    ().into_ok()
}
