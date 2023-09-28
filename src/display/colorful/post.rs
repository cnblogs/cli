use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::args::TimeStyle;
use crate::display::colorful::fmt_err;
use crate::infra::result::WrapResult;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Write;

pub fn list_post(
    result: Result<(impl ExactSizeIterator<Item = PostEntry>, usize)>,
) -> Result<String> {
    let (mut entry_iter, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).wrap_ok(),
    };

    entry_iter.try_fold(
        format!("{}/{}\n", entry_iter.len(), total_count),
        |mut buf, entry| try {
            {
                let buf = &mut buf;
                write!(buf, "{} {}", "#".dimmed(), entry.id.to_string().dimmed())?;
                if entry.is_published {
                    write!(buf, " {}", "Pub".green())?;
                } else {
                    write!(buf, " {}", "Dft".yellow())?;
                }
                if entry.is_pinned {
                    write!(buf, " {}", "Pin".magenta())?;
                }
                write!(buf, " {}", entry.title.cyan().bold())?;
                writeln!(buf)?;
            }
            buf
        },
    )
}

pub fn show_post(entry: &Result<PostEntry>) -> Result<String> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(e) => return fmt_err(e).wrap_ok(),
    };

    let mut buf = String::new();
    {
        let buf = &mut buf;
        writeln!(buf, "{}\n", entry.title.cyan().bold())?;
        if let Some(body) = &entry.body {
            writeln!(buf, "{}", body)?;
        }
    }
    buf.wrap_ok()
}

pub fn show_post_meta(time_style: &TimeStyle, entry: &Result<PostEntry>) -> Result<String> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(e) => return fmt_err(e).wrap_ok(),
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
    buf.wrap_ok()
}

pub fn show_post_comment(
    time_style: &TimeStyle,
    comment_iter: Result<impl ExactSizeIterator<Item = PostCommentEntry>>,
) -> Result<String> {
    let mut comment_iter = match comment_iter {
        Ok(entry) => entry,
        Err(e) => return fmt_err(&e).wrap_ok(),
    };

    comment_iter.try_fold(String::new(), |mut buf, comment| try {
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

pub fn search_self_post(
    result: Result<(impl ExactSizeIterator<Item = usize>, usize)>,
) -> Result<String> {
    let (mut id_iter, total_count) = match result {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).wrap_ok(),
    };

    id_iter.try_fold(
        format!("{}/{}\n", id_iter.len(), total_count),
        |mut buf, id| try {
            writeln!(&mut buf, "# {}", id)?;
            buf
        },
    )
}
