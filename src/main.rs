#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]

use crate::api::auth::session;
use crate::api::ing::{Ing, IngType};
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::parser::no_operation;
use crate::args::{parser, Args};
use crate::infra::result::IntoResult;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use std::env;

pub mod api;
pub mod args;
pub mod display;
pub mod infra;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    if args.debug {
        dbg!(env::args().collect::<Vec<_>>());
        dbg!(&args);
    }
    let style = &args.style;

    match args {
        _ if let Some(pat) = parser::login(&args) => {
            let cfg_path = session::login(pat)?;
            display::login(style, &cfg_path);
        }
        _ if parser::logout(&args) => {
            let cfg_path = session::logout()?;
            display::logout(style, &cfg_path);
        }
        _ if let Some(pat) = parser::user_info(&args) => {
            let user_info = User::new(pat?).get_info().await?;
            return display::user_info(style, &user_info)
        }
        _ if let Some(r) = parser::list_ing(&args) => {
            let (pat, skip, take, rev) = r?;
            let ing_type = IngType::Public;
            let ing_vec = Ing::new(pat).get_list(skip, take, &ing_type).await?;
            return display::list_ing(style, &ing_vec, rev)
        }
        _ if let Some(r) = parser::publish_ing(&args) => {
            let (pat, content) = r?;
            let result = Ing::new(pat).publish(content).await;
            display::publish_ing(style, &result.map(|_| content));
        }
        _ if let Some(r) = parser::comment_ing(&args) => {
            let (pat, content, id) = r?;
            let result = Ing::new(pat).comment(id, content.clone(), None, None).await;
            display::comment_ing(style, &result.map(|_| content));
        }
        _ if let Some(r) = parser::show_post(&args) => {
            let (pat, id) = r?;
            let entry = Post::new(pat).get_one(id).await?;
            display::show_post(style, &entry);
        }
        _ if let Some(r) = parser::show_post_meta(&args) => {
            let (pat, id) = r?;
            let entry = Post::new(pat).get_one(id).await?;
            display::show_post_meta(style, &entry)?;
        }
        _ if let Some(r) = parser::list_post(&args) => {
            let (pat, skip, take, rev) = r?;
            let (entry_vec, total_count) = Post::new(pat).get_meta_list(skip, take).await?;
            display::list_post(style, &entry_vec, total_count, rev);
        }

        _ if no_operation(&args) => {
            Args::command().print_help()?;
        }
        _ => {
            println!("Invalid usage, follow '--help' for more information.");
        }
    };

    ().into_ok()
}
