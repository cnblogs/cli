use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::ing::{
    fmt_content, get_ing_at_user_tag_text, ing_star_tag_to_text, rm_ing_at_user_tag,
};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::result::IntoResult;
use anyhow::Result;
use chrono::DateTime;
use colored::Colorize;
use std::ops::Not;
use std::path::PathBuf;

pub fn login(cfg_path: &PathBuf) {
    println!("PAT was saved in {:?}", cfg_path);
}

pub fn logout(cfg_path: &PathBuf) {
    println!("{:?} was removed", cfg_path);
}

// TODO: impl variant types of ..Display trait
pub fn user_info(info: &UserInfo) {
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

pub fn list_ing(ing_list: &[(IngEntry, Vec<IngCommentEntry>)], rev: bool) {
    let iter: Box<dyn Iterator<Item = &(IngEntry, Vec<IngCommentEntry>)>> = if rev {
        Box::new(ing_list.iter().rev())
    } else {
        Box::new(ing_list.iter())
    };
    iter.for_each(|(ing, comment_list)| {
        let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", ing.create_time))
            .map(|dt| dt.format("%m-%d %H:%M").to_string())
            .unwrap();

        print!("{}", create_time.dimmed());
        if ing.is_lucky {
            let star_text = ing_star_tag_to_text(&ing.icons);
            print!(" {}", star_text.yellow());
            print!("{}", "⭐");
        }
        println!(" {} {}", "#".dimmed(), ing.id.to_string().dimmed());
        print!("  {}", ing.user_name.cyan());
        let content = fmt_content(&ing.content);
        println!(" {}", content);

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

pub fn publish_ing(result: &Result<&String>) {
    match result {
        Ok(content) => println!("{}: {}", "Published".green(), content),
        Err(e) => println!("{}: {}", "Error".red(), e),
    }
}

pub fn comment_ing(result: &Result<&String>) {
    match result {
        Ok(content) => println!("{}: {}", "Commented".green(), content),
        Err(e) => println!("{}: {}", "Error".red(), e),
    }
}

pub fn show_post(entry: &PostEntry) {
    println!("{}\n", entry.title.cyan().bold());
    if let Some(body) = &entry.body {
        println!("{}", body);
    }
}

pub fn show_post_meta(entry: &PostEntry) -> Result<()> {
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
    let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", entry.create_time))?;
    println!("Create {}", create_time.format("%Y/%m/%d %H:%M"));
    let modify_time = DateTime::parse_from_rfc3339(&format!("{}Z", entry.create_time))?;
    println!("Modify {}", modify_time.format("%Y/%m/%d %H:%M"));
    println!("Link   https:{}", entry.url);

    ().into_ok()
}

pub fn list_post(entry_list: &[PostEntry], total_count: usize, rev: bool) {
    println!("{}/{}", entry_list.len(), total_count);
    let iter: Box<dyn Iterator<Item = &PostEntry>> = if rev {
        Box::new(entry_list.iter().rev())
    } else {
        Box::new(entry_list.iter())
    };
    iter.for_each(|entry| {
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
