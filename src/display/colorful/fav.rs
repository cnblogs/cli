use crate::api::fav::get_list::FavEntry;
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

pub fn list_fav(
    time_style: &TimeStyle,
    fav_iter: Result<impl ExactSizeIterator<Item = FavEntry>>,
) -> Result<String> {
    let fav_iter = match fav_iter {
        Ok(o) => o,
        Err(e) => return fmt_err(&e).into_ok(),
    };

    fav_iter
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
