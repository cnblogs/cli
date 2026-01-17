use chrono::{Datelike, NaiveDateTime};

pub trait DateFormatExt {
    fn as_time_age(&self) -> String;
}

impl DateFormatExt for NaiveDateTime {
    fn as_time_age(&self) -> String {
        let now = chrono::Local::now().naive_local();
        let year = now.year() - self.year();
        if year.ge(&1) {
            return self.format("[%y年%m月%d日%H:%M]").to_string();
        };

        let seconds = (now - *self).num_seconds();
        match seconds {
            secs if secs < 60 => format!("[{}秒前]", secs),
            secs if secs < 3600 => format!("[{}分钟前]", secs / 60),
            secs if secs < 86400 => format!("[{}小时前]", secs / 3600),
            secs if secs < 604800 => format!("[{}天前]", secs / 86400),
            _ => self.format("[%y年%m月%d日 %H:%M]").to_string(),
        }
    }
}
