use crate::api::ing::get_list::{IngCommentEntry, IngEntry};
use crate::api::post::get_one::PostEntry;
use crate::api::user::info::UserInfo;
use crate::infra::result::IntoResult;
use anyhow::Result;
use chrono::DateTime;
use colored::Colorize;
use std::path::PathBuf;

pub fn login(cfg_path: &PathBuf) {
    println!("PAT was saved in {:?}", cfg_path);
}

pub fn logout(cfg_path: &PathBuf) {
    println!("{:?} was removed", cfg_path);
}

pub fn user_info(user_info: &UserInfo) {
    println!("{}", user_info);
}

pub fn list_ing(ing_list: &[(IngEntry, Vec<IngCommentEntry>)], rev: bool) {
    let iter: Box<dyn Iterator<Item = _>> = if rev {
        Box::new(ing_list.iter().rev())
    } else {
        Box::new(ing_list.iter())
    };
    iter.for_each(|(ing, comment_list)| {
        println!("{}", ing);
        comment_list.iter().for_each(|c| println!("{}", c));
        println!();
    });
}

pub fn pub_ing(result: &Result<&String>) {
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
    println!("{}", entry.title.cyan().bold());
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

pub fn list_post(entry_list: &[PostEntry], rev: bool) {
    let iter: Box<dyn Iterator<Item = _>> = if rev {
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
