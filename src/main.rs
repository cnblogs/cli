#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]
#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

use crate::api::auth::session;
use crate::api::ing::{Ing, IngType};
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::parser::no_operation;
use crate::args::{parser, Args};
use crate::infra::fp::currying::eq;
use crate::infra::option::OptionExt;
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
    let args_vec = env::args().collect::<Vec<_>>();
    if args_vec.iter().any(eq(&"--debug".to_string())) {
        dbg!(args_vec);
    }

    let args: Args = Args::parse();
    if args.debug {
        dbg!(&args);
    }

    let pat = args.with_pat.clone().or_eval_result(session::get_pat);
    let style = &args.style;
    let rev = args.rev;
    // TODO
    let _fail_on_error = args.fail_on_error;

    match args {
        _ if let Some(pat) = parser::login(&args) => {
            let cfg_path = session::login(pat);
            display::login(style, &cfg_path);
        }
        _ if parser::logout(&args) => {
            let cfg_path = session::logout();
            display::logout(style, &cfg_path);
        }
        _ if parser::user_info(&args) => {
            let user_info = try {
                User::new(pat?).get_info().await?
            };
            display::user_info(style, &user_info);
        }
        _ if let Some((skip, take)) = parser::list_ing(&args) => {
            let ing_type = IngType::Public;
            let ing_vec = try {
                Ing::new(pat?).get_list(skip, take, &ing_type).await?
            };
            display::list_ing(style, &ing_vec, rev);
        }
        _ if let Some(content) = parser::publish_ing(&args) => {
            let content = try {
                Ing::new(pat?).publish(content).await?;
                content
            };
            display::publish_ing(style, &content);
        }
        _ if let Some((content, id))= parser::comment_ing(&args) => {
            let content = try {
                Ing::new(pat?).comment(id, content.clone(), None, None).await?;
                content
            };
            display::comment_ing(style, &content);
        }
        _ if let Some(id) = parser::show_post(&args) => {
            let entry = try { Post::new(pat?).get_one(id).await? };
            display::show_post(style, &entry);
        }
        _ if let Some(id) = parser::show_post_meta(&args) => {
            let entry = try { Post::new(pat?).get_one(id).await? };
            display::show_post_meta(style, &entry);
        }
        _ if let Some((skip, take)) = parser::list_post(&args) => {
            let result = try { Post::new(pat?).get_meta_list(skip, take).await? };
            display::list_post(style, &result, rev);
        }
        _ if let Some(id) = parser::delete_post(&args) => {
            let id = try {
                Post::new(pat?).del_one(id).await?;
                id
            };
            display::delete_post(style, &id);
        }
        _ if let Some((kw, skip, take)) = parser::search_post(&args) => {
            let result = try { Post::new(pat?).search(skip, take, kw).await? };
            display::search_post(style, &result, rev);
        }
        _ if let Some((title, body, publish)) = parser::create_post(&args) => {
            let id = try { Post::new(pat?).create(title, body, publish).await? };
            display::create_post(style, &id);
        }
        _ if let Some((id, title, body, publish)) = parser::update_post(&args) => {
            let id = try { Post::new(pat?).update(id,title, body, publish).await? };
            display::update_post(style, &id);
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
