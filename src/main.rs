#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]
#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

use crate::api::auth::session;
use crate::api::ing::Ing;
use crate::api::news::News;
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::parser::no_operation;
use crate::args::{parser, Args};
use crate::infra::fp::currying::eq;
use crate::infra::iter::IntoIteratorExt;
use crate::infra::option::OptionExt;
use crate::infra::result::IntoResult;
use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use std::env;
use std::ops::Not;

pub mod api;
pub mod args;
pub mod display;
pub mod infra;

fn panic_if_err<T>(result: &Result<T>) {
    if let Err(e) = result {
        panic!("{}", e)
    }
}

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
    let foe = args.fail_on_error;
    let quiet = args.quiet;

    match args {
        _ if let Some(pat) = parser::login(&args) => {
            let cfg_path = session::login(pat);
            foe.then(|| panic_if_err(&cfg_path));
            quiet.not().then(|| display::login(style, &cfg_path));
        }
        _ if parser::logout(&args) => {
            let cfg_path = &session::logout();
            foe.then(|| panic_if_err(cfg_path));
            quiet.not().then(|| display::logout(style, cfg_path));
        }
        _ if parser::user_info(&args) => {
            let user_info = try {
                User::new(pat?).get_info().await?
            };
            foe.then(|| panic_if_err(&user_info));
            quiet.not().then(|| display::user_info(style, &user_info));
        }
        _ if let Some((skip, take, r#type, align)) = parser::list_ing(&args) => {
            let ing_with_comment_list = try {
                let ing_api = Ing::new(pat?);
                let ing_vec = ing_api.get_list(skip, take, &r#type).await?;
                ing_vec.into_iter()
                    .map(|ing| async {
                        let result = ing_api.get_comment_list(ing.id).await;
                        result.map(|comment_vec| (ing, comment_vec))
                    })
                    .join_all()
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?
            };
            foe.then(|| panic_if_err(&ing_with_comment_list));
            quiet.not().then(|| display::list_ing(style, &ing_with_comment_list, rev, align));
        }
        _ if let Some(content) = parser::publish_ing(&args) => {
            let content = try {
                Ing::new(pat?).publish(content).await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            quiet.not().then(|| display::publish_ing(style, &content));
        }
        _ if let Some((content, id)) = parser::comment_ing(&args) => {
            let content = try {
                Ing::new(pat?).comment(id, content.clone(), None, None).await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            quiet.not().then(|| display::comment_ing(style, &content));
        }
        _ if let Some(id) = parser::show_post(&args) => {
            let entry = try { Post::new(pat?).get_one(id).await? };
            foe.then(|| panic_if_err(&entry));
            quiet.not().then(|| display::show_post(style, &entry));
        }
        _ if let Some(id) = parser::show_post_meta(&args) => {
            let entry = try { Post::new(pat?).get_one(id).await? };
            foe.then(|| panic_if_err(&entry));
            quiet.not().then(|| display::show_post_meta(style, &entry));
        }
        _ if let Some((skip, take)) = parser::list_post(&args) => {
            let result = try { Post::new(pat?).get_meta_list(skip, take).await? };
            foe.then(|| panic_if_err(&result));
            quiet.not().then(|| display::list_post(style, &result, rev));
        }
        _ if let Some(id) = parser::delete_post(&args) => {
            let id = try {
                Post::new(pat?).del_one(id).await?;
                id
            };
            foe.then(|| panic_if_err(&id));
            quiet.not().then(|| display::delete_post(style, &id));
        }
        _ if let Some((kw, skip, take)) = parser::search_post(&args) => {
            let result = try { Post::new(pat?).search(skip, take, kw).await? };
            foe.then(|| panic_if_err(&result));
            quiet.not().then(|| display::search_post(style, &result, rev));
        }
        _ if let Some((title, body, publish)) = parser::create_post(&args) => {
            let id = try { Post::new(pat?).create(title, body, publish).await? };
            foe.then(|| panic_if_err(&id));
            quiet.not().then(|| display::create_post(style, &id));
        }
        _ if let Some((id, title, body, publish)) = parser::update_post(&args) => {
            let id = try { Post::new(pat?).update(id, title, body, publish).await? };
            foe.then(|| panic_if_err(&id));
            quiet.not().then(|| display::update_post(style, &id));
        }
        _ if let Some((skip, take)) = parser::list_news(&args) => {
            let news_vec = try {
                News::new(pat?).get_list(skip, take).await?
            };
            foe.then(|| panic_if_err(&news_vec));
            quiet.not().then(|| display::list_news(style, &news_vec, rev));
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
