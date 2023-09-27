#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]
#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

use crate::api::auth::session;
use crate::api::fav::Fav;
use crate::api::ing::Ing;
use crate::api::news::News;
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::parser::no_operation;
use crate::args::{parser, Args};
use crate::infra::fp::currying::eq;
use crate::infra::infer::infer;
use crate::infra::iter::{ExactSizeIteratorExt, IntoIteratorExt};
use crate::infra::option::OptionExt;
use crate::infra::result::IntoResult;
use anyhow::Result;
use clap::Parser;
use clap::{Command, CommandFactory};
use colored::Colorize;
use std::env;

pub mod api;
pub mod args;
pub mod display;
pub mod infra;

fn show_non_printable_chars(text: String) -> String {
    #[inline]
    fn make_red(str: &str) -> String {
        format!("{}", str.red())
    }

    text.replace(' ', &make_red("·"))
        .replace('\0', &make_red("␀\0"))
        .replace('\t', &make_red("␉\t"))
        .replace('\n', &make_red("␊\n"))
        .replace('\r', &make_red("␍\r"))
        .replace("\r\n", &make_red("␍␊\r\n"))
}

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
    let time_style = &args.time_style;
    let rev = args.rev;
    let foe = args.fail_on_error;

    let output = match args {
        _ if let Some(pat) = parser::login(&args) => {
            let cfg_path = session::login(pat);
            foe.then(|| panic_if_err(&cfg_path));
            display::login(style, &cfg_path)
        }
        _ if parser::logout(&args) => {
            let cfg_path = session::logout();
            foe.then(|| panic_if_err(&cfg_path));
            display::logout(style, &cfg_path)
        }
        _ if parser::user_info(&args) => {
            let user_info = User::new(pat?).get_info().await;
            foe.then(|| panic_if_err(&user_info));
            display::user_info(style, &user_info)?
        }
        _ if let Some((skip, take, r#type, align)) = parser::list_ing(&args) => {
            let ing_with_comment_iter = infer::<Result<_, _>>(try {
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
            }).map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&ing_with_comment_iter));
            display::list_ing(style, time_style, ing_with_comment_iter, align)?
        }
        _ if let Some(content) = parser::publish_ing(&args) => {
            let content = try {
                Ing::new(pat?).publish(content).await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            display::publish_ing(style, &content)
        }
        _ if let Some((content, id)) = parser::comment_ing(&args) => {
            let content = try {
                Ing::new(pat?).comment(id, content.clone(), None, None).await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            display::comment_ing(style, &content)
        }
        _ if let Some(id) = parser::show_post(&args) => {
            let entry = Post::new(pat?).get_one(id).await;
            foe.then(|| panic_if_err(&entry));
            display::show_post(style, &entry)?
        }
        _ if let Some(id) = parser::show_post_meta(&args) => {
            let entry = Post::new(pat?).get_one(id).await;
            foe.then(|| panic_if_err(&entry));
            display::show_post_meta(style, time_style, &entry)?
        }
        _ if let Some(id) = parser::show_post_comment(&args) => {
            let comment_iter = Post::new(pat?)
                .get_comment_list(id).await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&comment_iter));
            display::show_post_comment(style, time_style, comment_iter)?
        }
        _ if let Some((skip, take)) = parser::list_post(&args) => {
            let meta_iter = Post::new(pat?)
                .get_meta_list(skip, take)
                .await
                .map(|(vec, count)| (vec.into_iter().dyn_rev(rev), count));
            foe.then(|| panic_if_err(&meta_iter));
            display::list_post(style, meta_iter)?
        }
        _ if let Some(id) = parser::delete_post(&args) => {
            let id = try {
                Post::new(pat?).del_one(id).await?;
                id
            };
            foe.then(|| panic_if_err(&id));
            display::delete_post(style, &id)
        }
        _ if let Some((kw, skip, take)) = parser::search_post(&args) => {
            let result = Post::new(pat?)
                .search(skip, take, kw)
                .await
                .map(|(vec, count)| (vec.into_iter().dyn_rev(rev), count));
            foe.then(|| panic_if_err(&result));
            display::search_post(style, result)?
        }
        _ if let Some((title, body, publish)) = parser::create_post(&args) => {
            let id = Post::new(pat?).create(title, body, publish).await;
            foe.then(|| panic_if_err(&id));
            display::create_post(style, &id)
        }
        _ if let Some((id, title, body, publish)) = parser::update_post(&args) => {
            let id = Post::new(pat?).update(id, title, body, publish).await;
            foe.then(|| panic_if_err(&id));
            display::update_post(style, &id)
        }
        _ if let Some((skip, take)) = parser::list_news(&args) => {
            let news_iter = News::new(pat?)
                .get_list(skip, take)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&news_iter));
            display::list_news(style, time_style, news_iter)?
        }
        _ if let Some((skip, take)) = parser::list_fav(&args) => {
            let fav_iter = Fav::new(pat?)
                .get_list(skip, take)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&fav_iter));
            display::list_fav(style, time_style, fav_iter)?
        }

        _ if no_operation(&args) =>
            infer::<Command>(Args::command()).render_help().to_string(),
        _ => "Invalid usage, follow '--help' for more information".to_owned()
    };

    if args.quiet {
        return ().into_ok();
    }

    let output = {
        let output = if output.ends_with("\n\n") {
            output[..output.len() - 1].to_owned()
        } else if output.ends_with('\n') {
            output
        } else {
            format!("{}\n", output)
        };
        if args.debug {
            show_non_printable_chars(output)
        } else {
            output
        }
    };

    print!("{}", output);

    ().into_ok()
}
