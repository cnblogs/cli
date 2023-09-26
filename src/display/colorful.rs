use crate::api::fav::get_list::FavEntry;
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
use crate::infra::result::IntoResult;
use crate::infra::str::StrExt;
use crate::infra::terminal::get_term_width;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Not;
use std::path::PathBuf;
use unicode_width::UnicodeWidthStr;

#[inline]
pub fn fmt_err(e: &anyhow::Error) -> String {
    format!("{}: {}", "Err".red(), e)
}

#[inline]
pub fn fmt_result<T: Display>(result: &Result<T>) -> String {
    match result {
        Ok(t) => format!("{}: {}", "Ok".green(), t),
        Err(e) => fmt_err(e),
    }
}

pub fn login(cfg_path: &Result<PathBuf>) -> String {
    match cfg_path {
        Ok(pb) => format!("PAT was saved in {:?}", pb),
        Err(e) => fmt_err(e),
    }
}

pub fn logout(cfg_path: &Result<PathBuf>) -> String {
    match cfg_path {
        Ok(pb) => format!("{:?} was successfully removed", pb),
        Err(e) => fmt_err(e),
    }
}

pub fn user_info(info: &Result<UserInfo>) -> Result<String> {
    let info = match info {
        Ok(info) => info,
        Err(e) => return fmt_err(e).into_ok(),
    };

    let mut buf = String::new();
    {
        let buf = &mut buf;
        write!(buf, "{}", info.display_name.cyan())?;
        if info.is_vip {
            write!(buf, " {}", " VIP ".on_blue())?;
        }
        writeln!(buf)?;
        writeln!(
            buf,
            "{} Following {} Followers",
            info.following_count, info.followers_count
        )?;
        writeln!(buf, "ID     {}", info.blog_id)?;
        writeln!(buf, "Joined {}", info.joined)?;
        writeln!(buf, "Blog   https://www.cnblogs.com/{}", info.blog_app)?;
    }
    buf.into_ok()
}

// TODO: rm unnecessary line divider
pub fn list_ing(
    time_style: &TimeStyle,
    ing_with_comment_list: &Result<Vec<(IngEntry, Vec<IngCommentEntry>)>>,
    rev: bool,
    align: bool,
) -> Result<String> {
    let ing_with_comment_list = match ing_with_comment_list {
        Ok(o) => o,
        Err(e) => return fmt_err(e).into_ok(),
    };

    ing_with_comment_list.iter().dyn_rev(rev).try_fold(
        String::new(),
        |mut buf, (ing, comment_list)| try {
            {
                let buf = &mut buf;
                let create_time = display_cnb_time(&ing.create_time, time_style);
                write!(buf, "{}", create_time.dimmed())?;

                let send_from_mark = match ing.send_from {
                    IngSendFrom::Cli => Some("CLI"),
                    IngSendFrom::CellPhone => Some("Mobile"),
                    IngSendFrom::VsCode => Some("VSCode"),
                    IngSendFrom::Web => Some("Web"),
                    _ => None,
                };
                if let Some(mark) = send_from_mark {
                    write!(buf, " {}", mark.dimmed())?;
                }
                if ing.is_lucky {
                    let star_text = ing_star_tag_to_text(&ing.icons);
                    write!(buf, " {}⭐", star_text.yellow())?;
                }
                writeln!(buf, " {} {}", "#".dimmed(), ing.id.to_string().dimmed())?;
                let content = if align {
                    let user_name_width = ing.user_name.width_cjk();
                    let left_width = get_term_width().saturating_sub(user_name_width + 3);
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
                writeln!(buf, "  {} {}", ing.user_name.cyan(), content)?;

                let len = comment_list.len();
                if len != 0 {
                    let max_i = len - 1;
                    let comment_list_buf: Result<String> = comment_list
                        .iter()
                        .enumerate()
                        .try_fold(String::new(), |mut buf, (i, entry)| try {
                            {
                                let buf = &mut buf;
                                if i != max_i {
                                    write!(buf, "    │ {}", entry.user_name.blue())?;
                                } else {
                                    write!(buf, "    └ {}", entry.user_name.blue())?;
                                }
                                let at_user = get_ing_at_user_tag_text(&entry.content);
                                if at_user.is_empty().not() {
                                    write!(
                                        buf,
                                        " {}{}",
                                        "@".bright_black(),
                                        at_user.bright_black()
                                    )?;
                                }
                                let content = {
                                    let content = rm_ing_at_user_tag(&entry.content);
                                    fmt_content(&content)
                                };
                                writeln!(buf, " {}", content.dimmed())?;
                            }
                            buf
                        });
                    write!(buf, "{}", comment_list_buf?)?;
                }

                writeln!(buf)?;
            };
            buf
        },
    )
}

pub fn show_post(entry: &Result<PostEntry>) -> Result<String> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(e) => return fmt_err(e).into_ok(),
    };

    let mut buf = String::new();
    {
        let buf = &mut buf;
        writeln!(buf, "{}\n", entry.title.cyan().bold())?;
        if let Some(body) = &entry.body {
            writeln!(buf, "{}", body)?;
        }
    }
    buf.into_ok()
}

pub fn show_post_meta(time_style: &TimeStyle, entry: &Result<PostEntry>) -> Result<String> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(e) => return fmt_err(e).into_ok(),
    };

    let mut buf = String::new();
    {
        let buf = &mut buf;
        writeln!(buf, "Title  {}", entry.title.cyan().bold())?;
        {
            write!(buf, "Status")?;
            if entry.is_published {
                write!(buf, " {}", "Published".green())?;
            } else {
                write!(buf, " {}", "Draft".yellow())?;
            }
            if entry.is_pinned {
                write!(buf, " {}", "Pinned".magenta())?;
            }
            writeln!(buf)?;
        };
        if let Some(body) = &entry.body {
            let words_count = words_count::count(body).words;
            writeln!(buf, "Words  {}", words_count)?;
        }
        if let Some(tags) = &entry.tags {
            if let Some(tags_text) = tags
                .clone()
                .into_iter()
                .reduce(|acc, tag| format!("{}, {}", acc, tag))
            {
                writeln!(buf, "Tags   {}", tags_text)?;
            }
        }
        let create_time = display_cnb_time(&entry.create_time, time_style);
        writeln!(buf, "Create {}", create_time)?;
        let modify_time = display_cnb_time(&entry.create_time, time_style);
        writeln!(buf, "Modify {}", modify_time)?;
        writeln!(buf, "Link   https:{}", entry.url)?;
    }
    buf.into_ok()
}

pub fn show_post_comment(
    time_style: &TimeStyle,
    comment_list: &Result<Vec<PostCommentEntry>>,
    rev: bool,
) -> Result<String> {
    let comment_list = match comment_list {
        Ok(entry) => entry,
        Err(e) => return fmt_err(e).into_ok(),
    };

    comment_list
        .iter()
        .dyn_rev(rev)
        .try_fold(String::new(), |mut buf, comment| try {
            {
                let buf = &mut buf;
                let create_time = display_cnb_time(&comment.create_time, time_style);
                let floor_text = format!("{}F", comment.floor);
                writeln!(buf, "{} {}", create_time.dimmed(), floor_text.dimmed())?;
                writeln!(buf, "  {} {}", comment.user_name.cyan(), comment.content)?;
            }
            buf
        })
}

pub fn list_post(result: &Result<(Vec<PostEntry>, usize)>, rev: bool) -> Result<String> {
    let (entry_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(e).into_ok(),
    };

    entry_list.iter().dyn_rev(rev).try_fold(
        format!("{}/{}\n", entry_list.len(), total_count),
        |mut buf, entry| try {
            {
                let buf = &mut buf;
                write!(buf, "{} {}", "#".dimmed(), entry.id.to_string().dimmed())?;
                write!(buf, " {}", entry.title.cyan().bold())?;
                if entry.is_published {
                    write!(buf, " {}", "Pub".green())?;
                } else {
                    write!(buf, " {}", "Dft".yellow())?;
                }
                if entry.is_pinned {
                    write!(buf, " {}", "Pin".magenta())?;
                }
                writeln!(buf)?;
            }
            buf
        },
    )
}

pub fn search_post(result: &Result<(Vec<usize>, usize)>, rev: bool) -> Result<String> {
    let (id_list, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(e).into_ok(),
    };

    id_list.iter().dyn_rev(rev).try_fold(
        format!("{}/{}\n", id_list.len(), total_count),
        |mut buf, id| try {
            writeln!(&mut buf, "# {}", id)?;
            buf
        },
    )
}

pub fn list_news(
    time_style: &TimeStyle,
    news_list: &Result<Vec<NewsEntry>>,
    rev: bool,
) -> Result<String> {
    let news_list = match news_list {
        Ok(o) => o,
        Err(e) => return fmt_err(e).into_ok(),
    };

    news_list
        .iter()
        .dyn_rev(rev)
        .map(|news| try {
            let mut buf = String::new();
            {
                let buf = &mut buf;
                let create_time = display_cnb_time(&news.create_time, time_style);
                let url = format!("https://news.cnblogs.com/n/{}", news.id);
                writeln!(buf, "{} {}", create_time.dimmed(), url.dimmed())?;
                writeln!(buf, "  {}", news.title)?;

                let summary = {
                    let summary = format!("{}...", news.summary);
                    summary.width_split(get_term_width() - 4).map_or_else(
                        || summary.clone(),
                        |vec| {
                            vec.into_iter()
                                .map(|line| format!("    {}", line))
                                .collect::<Vec<_>>()
                                .join("\n")
                        },
                    )
                };
                writeln!(buf, "{}", summary.dimmed())?;
            }
            buf
        })
        .try_fold(String::new(), |mut acc, buf: Result<String>| try {
            write!(&mut acc, "\n{}", buf?)?;
            acc
        })
}

// TODO: lift out rev option
pub fn list_fav(
    time_style: &TimeStyle,
    fav_list: &Result<Vec<FavEntry>>,
    rev: bool,
) -> Result<String> {
    let fav_list = match fav_list {
        Ok(o) => o,
        Err(e) => return fmt_err(e).into_ok(),
    };

    fav_list
        .iter()
        .dyn_rev(rev)
        .map(|fav| try {
            let mut buf = String::new();
            {
                let buf = &mut buf;
                let create_time = display_cnb_time(&fav.create_time, time_style);
                writeln!(buf, "{} {}", create_time.dimmed(), fav.url.dimmed())?;
                writeln!(buf, "  {}", fav.title)?;

                let summary = {
                    fav.summary.width_split(get_term_width() - 4).map_or_else(
                        || fav.summary.clone(),
                        |vec| {
                            vec.into_iter()
                                .map(|line| format!("    {}", line))
                                .collect::<Vec<_>>()
                                .join("\n")
                        },
                    )
                };
                if summary.is_empty().not() {
                    writeln!(buf, "{}", summary.dimmed())?;
                }
            }
            buf
        })
        .try_fold(String::new(), |mut acc, buf: Result<String>| try {
            write!(&mut acc, "\n{}", buf?)?;
            acc
        })
}
