use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::api::ing::{
    fmt_content, get_ing_at_user_tag_text, ing_star_tag_to_text, rm_ing_at_user_tag, IngSendFrom,
};
use crate::api::news::get_list::NewsEntry;
use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::args::TimeStyle;
use crate::infra::iter::IteratorExt;
use crate::infra::str::StrExt;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Display;
use std::ops::Not;
use std::path::PathBuf;
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

pub fn login(cfg_path: &Result<PathBuf>) {
    match cfg_path {
        Ok(pb) => println!("PAT was saved in {:?}", pb),
        Err(e) => println_err(e),
    };
}

pub fn logout(cfg_path: &Result<PathBuf>) {
    match cfg_path {
        Ok(pb) => println!("{:?} was successfully removed", pb),
        Err(e) => println_err(e),
    }
}

pub fn user_info(info: &Result<UserInfo>) {
    match info {
        Ok(info) => {
            print!("{}", info.display_name);
            if info.is_vip {
                print!(" VIP");
            }
            println!();
            println!(
                "{} Following {} Followers",
                info.following_count, info.followers_count
            );
            println!("ID     {}", info.blog_id);
            println!("Joined {}", info.joined);
            println!("Blog   https://www.cnblogs.com/{}", info.blog_app);
        }
        Err(e) => println_err(e),
    }
}

pub fn list_ing(
    time_style: &TimeStyle,
    ing_with_comment_list: &Result<Vec<(IngEntry, Vec<IngCommentEntry>)>>,
    rev: bool,
    align: bool,
) {
    let ing_with_comment_list = match ing_with_comment_list {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    ing_with_comment_list
        .iter()
        .dyn_rev(rev)
        .for_each(|(ing, comment_list)| {
            let create_time = display_cnb_time(&ing.create_time, time_style);
            print!("{}", create_time);

            let send_from_mark = match ing.send_from {
                IngSendFrom::Cli => Some("CLI"),
                IngSendFrom::CellPhone => Some("Mobile"),
                IngSendFrom::VsCode => Some("VSCode"),
                IngSendFrom::Web => Some("Web"),
                _ => None,
            };
            if let Some(mark) = send_from_mark {
                print!(" {}", mark.dimmed());
            }
            if ing.is_lucky {
                let star_text = ing_star_tag_to_text(&ing.icons);
                print!(" {}⭐", star_text);
            }
            println!(" # {}", ing.id);
            let content = if align {
                let user_name_width = ing.user_name.width_cjk();
                let term_width = terminal_size().expect("Can not get terminal size").0 .0 as usize;
                let left_width = term_width.saturating_sub(user_name_width + 3);
                fmt_content(&ing.content)
                    .width_split(left_width)
                    .map_or_else(
                        || ing.content.clone(),
                        |lines| {
                            if comment_list.is_empty().not() {
                                lines.join("\n").replace(
                                    '\n',
                                    &format!("\n    │{}", " ".repeat(user_name_width - 2)),
                                )
                            } else {
                                lines.join("\n").replace(
                                    '\n',
                                    &format!("\n{}", " ".repeat(user_name_width + 3)),
                                )
                            }
                        },
                    )
            } else {
                fmt_content(&ing.content)
            };
            println!("  {} {}", ing.user_name, content);

            let len = comment_list.len();
            if len != 0 {
                let max_i = len - 1;
                comment_list.iter().enumerate().for_each(|(i, entry)| {
                    if i != max_i {
                        print!("    │ {}: ", entry.user_name);
                    } else {
                        print!("    └ {}: ", entry.user_name);
                    }
                    let at_user = get_ing_at_user_tag_text(&entry.content);
                    if at_user.is_empty().not() {
                        print!(" @{}", at_user);
                    }
                    let content = {
                        let content = rm_ing_at_user_tag(&entry.content);
                        fmt_content(&content)
                    };
                    println!(" {}", content);
                });
            }
            println!();
        });
}

pub fn show_post(entry: &Result<PostEntry>) {
    match entry {
        Ok(entry) => {
            println!("{}\n", entry.title);
            if let Some(body) = &entry.body {
                println!("{}", body);
            }
        }
        Err(e) => println_err(e),
    }
}

pub fn show_post_meta(time_style: &TimeStyle, entry: &Result<PostEntry>) {
    let entry = match entry {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    println!("Title  {}", entry.title);
    {
        print!("Status");
        if entry.is_published {
            print!(" Published");
        } else {
            print!(" Draft");
        }
        if entry.is_pinned {
            print!(" Pinned");
        }
        println!()
    };
    if let Some(body) = &entry.body {
        let words_count = words_count::count(body).words;
        println!("Words  {}", words_count);
    }
    if let Some(tags) = &entry.tags {
        if let Some(tags_text) = tags
            .clone()
            .into_iter()
            .reduce(|acc, tag| format!("{}, {}", acc, tag))
        {
            println!("Tags   {}", tags_text);
        }
    }
    let create_time = display_cnb_time(&entry.create_time, time_style);
    println!("Create {}", create_time);
    let modify_time = display_cnb_time(&entry.modify_time, time_style);
    println!("Modify {}", modify_time);
    println!("Link   https:{}", entry.url);
}

pub fn show_post_comment(
    time_style: &TimeStyle,
    comment_list: &Result<Vec<PostCommentEntry>>,
    rev: bool,
) {
    let comment_list = match comment_list {
        Ok(entry) => entry,
        Err(e) => return println_err(e),
    };

    comment_list.iter().dyn_rev(rev).for_each(|comment| {
        let create_time = display_cnb_time(&comment.create_time, time_style);
        println!("{} {}F", create_time, comment.floor);
        println!("  {} {}", comment.user_name, comment.content);
    })
}

pub fn list_post(result: &Result<(Vec<PostEntry>, usize)>, rev: bool) {
    let (entry_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    println!("{}/{}", entry_list.len(), total_count);
    entry_list.iter().dyn_rev(rev).for_each(|entry| {
        print!("# {}", entry.id);
        print!(" {}", entry.title);
        if entry.is_published {
            print!(" Pub");
        } else {
            print!(" Dft");
        }
        if entry.is_pinned {
            print!(" Pin");
        }
        println!()
    });
}

pub fn search_post(result: &Result<(Vec<usize>, usize)>, rev: bool) {
    let (id_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    println!("{}/{}", id_list.len(), total_count);
    id_list
        .iter()
        .dyn_rev(rev)
        .for_each(|id| println!("# {}", id));
}

pub fn println_err(e: &anyhow::Error) {
    println!("Err: {}", e)
}

pub fn println_result<T: Display>(result: &Result<T>) {
    match result {
        Ok(t) => println!("Ok: {}", t),
        Err(e) => println!("Err: {}", e),
    }
}

pub fn list_news(time_style: &TimeStyle, news_list: &Result<Vec<NewsEntry>>, rev: bool) {
    let news_list = match news_list {
        Ok(o) => o,
        Err(e) => return println_err(e),
    };

    news_list.iter().dyn_rev(rev).for_each(|news| {
        let create_time = display_cnb_time(&news.create_time, time_style);
        let url = format!("https://news.cnblogs.com/n/{}", news.id);
        println!("{} {}", create_time, url);
        println!("  {}", news.title);
        println!("    {}...", news.summary);
        println!();
    });
}
