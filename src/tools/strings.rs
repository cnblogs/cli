use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;

// 预编译正则表达式以提升性能
lazy_static::lazy_static! {
    pub static ref AT_PEOPLE_REGEX: regex::Regex =
        regex::Regex::new(r"<a\s+[^>]*>([^<]*)</a>")
            .expect("无效的 @ 人员正则表达式");

    pub static ref ICON_ALT_REGEX: regex::Regex =
        regex::Regex::new(r#"<img[^>]*alt\s*=\s*["']([^"']*)["'][^>]*>"#)
            .expect("无效的图标正则表达式");
}
/// 如果文本中还有a标签链接，则进一步处理，将超链接替换成链接内容，
/// 主要是针对@xxx情况。
pub trait ExtractAtPeople {
    fn extract_name(self) -> Result<String>;
}

impl ExtractAtPeople for String {
    fn extract_name(self) -> Result<String> {
        // 使用预编译的正则表达式
        let re = &AT_PEOPLE_REGEX;

        if !re.is_match(&self) {
            return Ok(self);
        }

        let mut last_end = 0;
        let mut output = Self::with_capacity(self.len());

        // 更高效的单次遍历替换
        for caps in re.captures_iter(&self) {
            let m = caps.get(0).ok_or_else(|| anyhow!("未找到匹配项"))?;
            let link_text = caps.get(1).ok_or_else(|| anyhow!("未找到链接文本"))?;

            // 添加匹配前的文本
            output.push_str(&self[last_end..m.start()]);

            // 添加着色后的链接文本
            output.push_str(&link_text.as_str().blue().to_string());

            last_end = m.end();
        }

        // 添加最后一段文本
        if last_end < self.len() {
            output.push_str(&self[last_end..]);
        }

        Ok(output)
    }
}

pub trait ExtractIconAlt {
    fn extract_tag(self) -> Result<String>;
}

impl ExtractIconAlt for String {
    fn extract_tag(self) -> Result<String> {
        // 使用预编译的正则表达式
        let re = &ICON_ALT_REGEX;

        if let Some(caps) = re.captures(&self) {
            let alt_text = caps
                .get(1)
                .ok_or_else(|| anyhow!("未找到 alt 文本"))?
                .as_str();

            // 根据内容决定颜色
            if alt_text.contains("VIP") {
                Ok(format!("{}", alt_text.red()))
            } else {
                Ok(format!("{}", alt_text.bright_yellow()))
            }
        } else {
            Ok(Self::new())
        }
    }
}
