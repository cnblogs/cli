#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]
#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

use crate::api::auth::session;
use crate::api::fav::Fav;
use crate::api::ing::Ing;
use crate::api::news::News;
use crate::api::post::Post;
use crate::api::user::User;
use crate::args::cmd::post::{CreateCmd, UpdateCmd};
use crate::args::parser::no_operation;
use crate::args::{parser, Args};
use crate::infra::fp::currying::eq;
use crate::infra::infer::infer;
use crate::infra::iter::{ExactSizeIteratorExt, IntoIteratorExt};
use crate::infra::option::OptionExt;
use crate::infra::result::WrapResult;
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

#[allow(clippy::missing_const_for_fn)]
fn panic_if_err<T>(result: &Result<T>) {
    if let Err(e) = result {
        panic!("{}", e)
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args_vec = env::args().collect::<Vec<_>>();
    if args_vec.iter().any(eq(&"--debug".to_owned())) {
        dbg!(args_vec);
    }

    let args: Args = Args::parse();
    let global_opt = &args.global_opt;
    if global_opt.debug {
        dbg!(&args);
    }

    let pat = global_opt.with_pat.clone().or_eval_result(session::get_pat);
    let style = &global_opt.style;
    let time_style = &global_opt.time_style;
    let rev = args.rev;
    let foe = global_opt.fail_on_error;

    let output = match args {
        _ if let Some(pat) = parser::user::login(&args) => {
            let cfg_path = session::login(pat);
            foe.then(|| panic_if_err(&cfg_path));
            display::login(style, &cfg_path)
        }
        _ if parser::user::logout(&args) => {
            let cfg_path = session::logout();
            foe.then(|| panic_if_err(&cfg_path));
            display::logout(style, &cfg_path)
        }
        _ if parser::user::user_info(&args) => {
            let user_info = User::new(pat?).get_info().await;
            foe.then(|| panic_if_err(&user_info));
            display::user_info(style, &user_info)?
        }
        _ if let Some((skip, take, r#type, align)) = parser::ing::list_ing(&args) => {
            let ing_with_comment_iter = infer::<Result<_, _>>(
                try {
                    let ing_api = Ing::new(pat?);
                    let ing_vec = ing_api.get_list(skip, take, &r#type).await?;
                    ing_vec
                        .into_iter()
                        .map(|ing| async {
                            let result = ing_api.get_comment_list(ing.id).await;
                            result.map(|comment_vec| (ing, comment_vec))
                        })
                        .join_all()
                        .await
                        .into_iter()
                        .collect::<Result<Vec<_>>>()?
                },
            )
            .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&ing_with_comment_iter));
            display::list_ing(style, time_style, ing_with_comment_iter, align)?
        }
        _ if let Some(content) = parser::ing::publish_ing(&args) => {
            let content = try {
                Ing::new(pat?).publish(content).await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            display::publish_ing(style, &content)
        }
        _ if let Some((content, id)) = parser::ing::comment_ing(&args) => {
            let content = try {
                Ing::new(pat?)
                    .comment(id, content.clone(), None, None)
                    .await?;
                content
            };
            foe.then(|| panic_if_err(&content));
            display::comment_ing(style, &content)
        }
        _ if let Some(id) = parser::post::show_post(&args) => {
            let entry = Post::new(pat?).get_one(id).await;
            foe.then(|| panic_if_err(&entry));
            display::show_post(style, &entry)?
        }
        _ if let Some(id) = parser::post::show_post_meta(&args) => {
            let entry = Post::new(pat?).get_one(id).await;
            foe.then(|| panic_if_err(&entry));
            display::show_post_meta(style, time_style, &entry)?
        }
        _ if let Some(id) = parser::post::show_post_comment(&args) => {
            let comment_iter = Post::new(pat?)
                .get_comment_list(id)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&comment_iter));
            display::show_post_comment(style, time_style, comment_iter)?
        }
        _ if let Some((skip, take)) = parser::post::list_post(&args) => {
            let meta_iter = Post::new(pat?)
                .get_meta_list(skip, take)
                .await
                .map(|(vec, count)| (vec.into_iter().dyn_rev(rev), count));
            foe.then(|| panic_if_err(&meta_iter));
            display::list_post(style, meta_iter)?
        }
        _ if let Some(id) = parser::post::delete_post(&args) => {
            let id = try {
                Post::new(pat?).del_one(id).await?;
                id
            };
            foe.then(|| panic_if_err(&id));
            display::delete_post(style, &id)
        }
        _ if let Some((kw, skip, take)) = parser::post::search_self_post(&args) => {
            let result = Post::new(pat?)
                .search_self(skip, take, kw)
                .await
                .map(|(vec, count)| (vec.into_iter().dyn_rev(rev), count));
            foe.then(|| panic_if_err(&result));
            display::search_self_post(style, result)?
        }
        _ if let Some((kw, skip, take)) = parser::post::search_site_post(&args) => {
            let result = Post::new(pat?)
                .search_site(skip, take, kw)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&result));
            display::search_site_post(style, time_style, result)?
        }
        _ if let Some(create_cmd) = parser::post::create_post(&args) => {
            let CreateCmd {
                title,
                body,
                publish,
            } = create_cmd;
            let id = Post::new(pat?).create(title, body, *publish).await;
            foe.then(|| panic_if_err(&id));
            display::create_post(style, &id)
        }
        _ if let Some((id, update_cmd)) = parser::post::update_post(&args) => {
            let UpdateCmd {
                title,
                body,
                publish,
            } = update_cmd;
            let id = Post::new(pat?).update(id, title, body, publish).await;
            foe.then(|| panic_if_err(&id));
            display::update_post(style, &id)
        }
        _ if let Some((skip, take)) = parser::news::list_news(&args) => {
            let news_iter = News::new(pat?)
                .get_list(skip, take)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&news_iter));
            display::list_news(style, time_style, news_iter)?
        }
        _ if let Some((skip, take)) = parser::fav::list_fav(&args) => {
            let fav_iter = Fav::new(pat?)
                .get_list(skip, take)
                .await
                .map(|vec| vec.into_iter().dyn_rev(rev));
            foe.then(|| panic_if_err(&fav_iter));
            display::list_fav(style, time_style, fav_iter)?
        }

        _ if no_operation(&args) => infer::<Command>(Args::command()).render_help().to_string(),
        _ => "Invalid usage, follow '--help' for more information".to_owned(),
    };

    if global_opt.quiet {
        return ().wrap_ok();
    }

    let output = {
        let output = if output.ends_with("\n\n") {
            output[..output.len() - 1].to_owned()
        } else if output.ends_with('\n') {
            output
        } else {
            format!("{}\n", output)
        };
        if global_opt.debug {
            show_non_printable_chars(output)
        } else {
            output
        }
    };

    print!("{}", output);

    ().wrap_ok()
}
