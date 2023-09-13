use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::ing::{
    fmt_content, get_ing_at_user_tag_text, ing_star_tag_to_text, rm_ing_at_user_tag, IngSendFrom,
};
use crate::api::news::get_list::NewsEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::iter::IteratorExt;
use crate::infra::time::patch_rfc3339;
use anyhow::Result;
use chrono::DateTime;
use colored::Colorize;
use std::fmt::Display;
use std::ops::Not;
use std::path::PathBuf;

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
            print!("{}", info.display_name.cyan());
            if info.is_vip {
                print!(" {}", " VIP ".on_blue());
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

pub fn list_ing(ing_list: &Result<Vec<(IngEntry, Vec<IngCommentEntry>)>>, rev: bool) {
    if let Err(e) = ing_list {
        println_err(e);
        return;
    }

    ing_list
        .as_ref()
        .unwrap()
        .iter()
        .dyn_rev(rev)
        .for_each(|(ing, comment_list)| {
            let create_time = {
                let rfc3339 = patch_rfc3339(&ing.create_time);
                let dt = DateTime::parse_from_rfc3339(&rfc3339).unwrap();
                dt.format("%m-%d %H:%M").to_string()
            };

            print!("{}", create_time.dimmed());
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
                print!(" {}⭐", star_text.yellow());
            }
            println!(" {} {}", "#".dimmed(), ing.id.to_string().dimmed());
            let content = fmt_content(&ing.content);
            println!("  {} {}", ing.user_name.cyan(), content);

            let len = comment_list.len();
            if len != 0 {
                let max_i = len - 1;
                comment_list.iter().enumerate().for_each(|(i, entry)| {
                    if i != max_i {
                        print!("    │ {}", entry.user_name.blue());
                    } else {
                        print!("    └ {}", entry.user_name.blue());
                    }
                    let at_user = get_ing_at_user_tag_text(&entry.content);
                    if at_user.is_empty().not() {
                        print!(" {}{}", "@".bright_black(), at_user.bright_black());
                    }
                    let content = {
                        let content = rm_ing_at_user_tag(&entry.content);
                        fmt_content(&content)
                    };
                    println!(" {}", content.dimmed());
                });
            }
            println!();
        });
}

pub fn show_post(entry: &Result<PostEntry>) {
    match entry {
        Ok(entry) => {
            println!("{}\n", entry.title.cyan().bold());
            if let Some(body) = &entry.body {
                println!("{}", body);
            }
        }
        Err(e) => println_err(e),
    }
}

pub fn show_post_meta(entry: &Result<PostEntry>) {
    if let Err(e) = entry {
        println_err(e);
        return;
    }

    let entry = entry.as_ref().unwrap();
    println!("Title  {}", entry.title.cyan().bold());
    {
        print!("Status");
        if entry.is_published {
            print!(" {}", "Published".green());
        } else {
            print!(" {}", "Draft".yellow());
        }
        if entry.is_pinned {
            print!(" {}", "Pinned".magenta());
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
    let create_time = {
        let rfc3339 = patch_rfc3339(&entry.create_time);
        let dt = DateTime::parse_from_rfc3339(&rfc3339).unwrap();
        dt.format("%Y-%m-%d %H:%M")
    };
    println!("Create {}", create_time);
    let modify_time = {
        let rfc3339 = patch_rfc3339(&entry.modify_time);
        let dt = DateTime::parse_from_rfc3339(&rfc3339).unwrap();
        dt.format("%Y-%m-%d %H:%M")
    };
    println!("Modify {}", modify_time);
    println!("Link   https:{}", entry.url);
}

pub fn list_post(result: &Result<(Vec<PostEntry>, usize)>, rev: bool) {
    if let Err(e) = result {
        println_err(e);
        return;
    }

    let (entry_list, total_count) = result.as_ref().unwrap();
    println!("{}/{}", entry_list.len(), total_count);
    entry_list.iter().dyn_rev(rev).for_each(|entry| {
        print!("{} {}", "#".dimmed(), entry.id.to_string().dimmed());
        print!(" {}", entry.title.cyan().bold());
        if entry.is_published {
            print!(" {}", "Pub".green());
        } else {
            print!(" {}", "Dft".yellow());
        }
        if entry.is_pinned {
            print!(" {}", "Pin".magenta());
        }
        println!()
    });
}

pub fn search_post(result: &Result<(Vec<usize>, usize)>, rev: bool) {
    if let Err(e) = result {
        println_err(e);
        return;
    }

    let (id_list, total_count) = result.as_ref().unwrap();
    println!("{}/{}", id_list.len(), total_count);
    id_list
        .iter()
        .dyn_rev(rev)
        .for_each(|id| println!("# {}", id));
}

pub fn println_err(e: &anyhow::Error) {
    println!("{}: {}", "Err".red(), e)
}

pub fn println_result<T: Display>(result: &Result<T>) {
    match result {
        Ok(t) => println!("{}: {}", "Ok".green(), t),
        Err(e) => println!("{}: {}", "Err".red(), e),
    }
}

pub fn list_news(news_list: &Result<Vec<NewsEntry>>, rev: bool) {
    if let Err(e) = news_list {
        println_err(e);
        return;
    }

    news_list
        .as_ref()
        .unwrap()
        .iter()
        .dyn_rev(rev)
        .for_each(|news| {
            let create_time = {
                let rfc3339 = patch_rfc3339(&news.create_time);
                let dt = DateTime::parse_from_rfc3339(&rfc3339).unwrap();
                dt.format("%Y-%m-%d %H:%M").to_string()
            };

            let url = format!("https://news.cnblogs.com/n/{}", news.id);
            println!("{} {}", create_time.dimmed(), url.dimmed(),);
            println!("  {}", news.title);
            println!("    {}{}", news.summary.dimmed(), "...".dimmed());
            println!();
        });
}
