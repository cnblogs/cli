use crate::api::news::get_list::NewsEntry;
use crate::args::TimeStyle;
use crate::display::colorful::fmt_err;
use crate::infra::result::WrapResult;
use crate::infra::str::StrExt;
use crate::infra::terminal::get_term_width;
use crate::infra::time::display_cnb_time;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Write;

pub fn list_news(
    time_style: &TimeStyle,
    news_iter: Result<impl ExactSizeIterator<Item = NewsEntry>>,
) -> Result<String> {
    let news_iter = match news_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).wrap_ok(),
    };

    news_iter
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
            writeln!(&mut acc, "{}", buf?)?;
            acc
        })
}
