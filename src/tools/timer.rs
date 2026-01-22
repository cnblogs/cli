use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Utc};

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

impl DateFormatExt for DateTime<Utc> {
    fn as_time_age(&self) -> String {
        // 将UTC时间转换为本地时间
        let local_time = self.with_timezone(&Local);
        let now = Local::now();

        // 计算年份差
        let year = now.year() - local_time.year();
        if year >= 1 {
            return local_time.format("[%y年%m月%d日 %H:%M]").to_string();
        }

        // 计算秒数差
        let seconds = (now - local_time).num_seconds();
        match seconds {
            secs if secs < 60 => format!("[{}秒前]", secs),
            secs if secs < 3600 => format!("[{}分钟前]", secs / 60),
            secs if secs < 86400 => format!("[{}小时前]", secs / 3600),
            secs if secs < 604800 => format!("[{}天前]", secs / 86400),
            _ => local_time.format("[%y年%m月%d日 %H:%M]").to_string(),
        }
    }
}

/// 自定义反序列化模块
pub mod rfc3339_or_naive {
    use std::time::Duration;

    use super::*;

    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // 首先尝试解析RFC3339格式（带时区）
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(dt.with_timezone(&Utc));
        }

        // 2026-01-22T18:35:49.593
        if let Ok(ndt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3f") {
            let ndt = ndt - Duration::from_hours(8);
            return Ok(Utc.from_utc_datetime(&ndt));
        }

        // 2026-01-22T18:35:49
        if let Ok(ndt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
            let ndt = ndt - Duration::from_hours(8);
            return Ok(Utc.from_local_datetime(&ndt).unwrap());
        }

        Err(de::Error::custom(format!("无效的时间格式: {}", s)))
    }
}
