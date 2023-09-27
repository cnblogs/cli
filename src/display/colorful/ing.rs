use crate::api::ing::get_comment_list::IngCommentEntry;
use crate::api::ing::get_list::IngEntry;
use crate::api::ing::{
    fmt_content, get_ing_at_user_tag_text, ing_star_tag_to_text, rm_ing_at_user_tag, IngSendFrom,
};
use crate::args::TimeStyle;
use crate::display::colorful::fmt_err;
use crate::infra::result::IntoResult;
use crate::infra::str::StrExt;
use crate::infra::terminal::get_term_width;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Write;
use std::ops::Not;
use unicode_width::UnicodeWidthStr;

// TODO: rm unnecessary line divider
pub fn list_ing(
    time_style: &TimeStyle,
    ing_with_comment_iter: Result<impl ExactSizeIterator<Item = (IngEntry, Vec<IngCommentEntry>)>>,
    align: bool,
) -> Result<String> {
    let mut ing_with_comment_iter = match ing_with_comment_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    ing_with_comment_iter.try_fold(String::new(), |mut buf, (ing, comment_list)| try {
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
                let comment_list_buf: Result<String> = comment_list.iter().enumerate().try_fold(
                    String::new(),
                    |mut buf, (i, entry)| try {
                        {
                            let buf = &mut buf;
                            if i != max_i {
                                write!(buf, "    │ {}", entry.user_name.blue())?;
                            } else {
                                write!(buf, "    └ {}", entry.user_name.blue())?;
                            }
                            let at_user = get_ing_at_user_tag_text(&entry.content);
                            if at_user.is_empty().not() {
                                write!(buf, " {}{}", "@".bright_black(), at_user.bright_black())?;
                            }
                            let content = {
                                let content = rm_ing_at_user_tag(&entry.content);
                                fmt_content(&content)
                            };
                            writeln!(buf, " {}", content.dimmed())?;
                        }
                        buf
                    },
                );
                write!(buf, "{}", comment_list_buf?)?;
            }

            writeln!(buf)?;
        };
        buf
    })
}
