pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use std::fmt;

use clap::{Parser, Subcommand, ValueEnum};
#[derive(Debug, Parser)]
#[command(name = "cnblogs", about = "博客园CLI工具", version)]
pub struct Cli {
    #[arg(long, short = 'j', default_value = "false", global = true)]
    pub json: bool,
    #[arg(long, short = 'v', global = true, help = "显示详细信息")]
    pub verbose: bool,
    #[arg(long, short = 's', global = true, default_value_t = Style::Pretty, help = "输出样式")]
    pub style: Style,
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    User(user::UserCommand),
    Ing {
        #[command(subcommand)]
        action: ing::IngAction,
    },
    Post(post::PostCommand),
    News(news::NewsCommand),
    Fav(fav::FaverateCommand),
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum Style {
    #[clap(name = "json", help = "JSON格式输出")]
    Json,
    #[clap(name = "pretty", help = "美化输出格式输出")]
    Pretty,
    #[clap(name = "quiet", help = "禁用输出")]
    Quiet,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Pretty => write!(f, "pretty"),
            Self::Quiet => write!(f, "quiet"),
        }
    }
}

// 验证不能为零
fn validate_non_zero_id(s: &str) -> Result<u64, String> {
    let id = s
        .parse::<u64>()
        .map_err(|_| format!("'{}' 不是有效的数字", s))?;

    if id == 0 {
        Err("ID不能为0".to_string())
    } else {
        Ok(id)
    }
}

pub fn validate_non_string(s: &str) -> Result<String, String> {
    let res = s
        .parse::<String>()
        .map_err(|_| format!("'{}' 不是有效的字符串", s))?;

    if res.is_empty() {
        Err("ID不能为0".to_string())
    } else {
        Ok(res)
    }
}
