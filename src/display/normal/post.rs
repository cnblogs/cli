use crate::api::post::get_comment_list::PostCommentEntry;
use crate::api::post::get_one::PostEntry;
use crate::api::post::search_site::SearchResultEntry;
use crate::args::TimeStyle;
use crate::display::normal::fmt_err;
use crate::infra::result::WrapResult;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
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
                write!(buf, "# {}", entry.id)?;
                if entry.is_published {
                    write!(buf, " Pub")?;
                } else {
                    write!(buf, " Dft")?;
                }
                if entry.is_pinned {
                    write!(buf, " Pin")?;
                }
                write!(buf, " {}", entry.title)?;
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
        writeln!(buf, "{}\n", entry.title)?;
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
        writeln!(buf, "Title  {}", entry.title)?;
        {
            write!(buf, "Status")?;
            if entry.is_published {
                write!(buf, " Published")?;
            } else {
                write!(buf, " Draft")?;
            }
            if entry.is_pinned {
                write!(buf, " Pinned")?;
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
            writeln!(buf, "{} {}F", create_time, comment.floor)?;
            writeln!(buf, "  {} {}", comment.user_name, comment.content)?;
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

pub fn search_site_post(
    time_style: &TimeStyle,
    entry_iter: Result<impl ExactSizeIterator<Item = SearchResultEntry>>,
) -> Result<String> {
    let entry_iter = match entry_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).wrap_ok(),
    };

    entry_iter
        .map(|entry| try {
            let mut buf = String::new();
            {
                let buf = &mut buf;
                let create_time = display_cnb_time(&entry.create_time, time_style);
                writeln!(buf, "{} {}", create_time, entry.url)?;
                writeln!(buf, "  {}", entry.title)?;
                let view_vote_comment_count = format!(
                    "View {} Vote {} Comment {}",
                    entry.view_count, entry.vote_count, entry.comment_count
                );
                writeln!(buf, "    {}", view_vote_comment_count)?;
            }
            buf
        })
        .try_fold(String::new(), |mut acc, buf: Result<String>| try {
            writeln!(&mut acc, "{}", buf?)?;
            acc
        })
}
