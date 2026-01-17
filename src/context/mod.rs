pub mod output;

use core::time;
use std::{fmt, fs, io::Read, path::PathBuf};

use anyhow::{Ok, Result, anyhow};
use owo_colors::OwoColorize;
use reqwest::{
    Client, ClientBuilder,
    header::{self, HeaderMap},
};

use crate::context::output::Terminal;

const FILENAME: &str = ".cnblogs/token";

pub struct Context {
    pub terminal: Terminal,
    pub token: String,
    pub client: Client,
    pub headers: HeaderMap,
    pub home_dir: PathBuf,
    pub file: PathBuf,
    pub full_path: PathBuf,
    pub json: bool,
}

impl Context {
    pub fn new() -> Result<Self> {
        Self::new_with_token("".to_string())
    }

    pub fn new_with_token(token: String) -> Result<Self> {
        let mut terminal = Terminal::new();
        let mut token = token;
        let home_dir = home::home_dir().ok_or_else(|| anyhow!("未获取到家目录，退出。"))?;
        let file = PathBuf::from(FILENAME);
        let full_path = home_dir.join(&file);
        let mut headers = HeaderMap::new();

        if !full_path.exists() {
            let _ = terminal.writeln(format!("缓存文件 `{}` 不存在", FILENAME).red());
            if !full_path
                .parent()
                .ok_or_else(|| anyhow!("检查`~/.cnblogs`文件夹失败"))?
                .exists()
            {
                fs::create_dir_all(
                    full_path
                        .parent()
                        .ok_or_else(|| anyhow!("创建`~/.cnblogs`文件夹失败"))?,
                )?;
            }
            let _ = fs::File::create(full_path.clone())?;
        }

        if token.is_empty() {
            let _ = fs::File::open(full_path.clone())?.read_to_string(&mut token);
        } else {
            fs::write(full_path.clone(), token.as_bytes())?;
        }

        if !token.is_empty() {
            let header_value = format!("Bearer {token}");
            headers.append(header::AUTHORIZATION, header_value.parse()?);
        }

        let client = ClientBuilder::new()
            .default_headers(headers.clone())
            .connect_timeout(time::Duration::from_secs(10))
            .https_only(true)
            .build()?;

        Ok(Self {
            terminal,
            token,
            client,
            headers,
            home_dir,
            file,
            full_path,
            json: false,
        })
    }

    pub const fn set_json(&mut self, json: bool) {
        self.json = json;
    }

    pub fn update_auth_header(&mut self) -> Result<()> {
        let header_value = format!("Bearer {}", self.token);
        self.headers
            .insert(header::AUTHORIZATION, header_value.parse()?);
        Ok(())
    }

    pub fn update_cache_file(&self) -> Result<()> {
        fs::write(&self.full_path, self.token.as_bytes())?;
        Ok(())
    }

    pub fn update_token(&mut self, token: String) -> Result<()> {
        // if !token.is_empty() {
        self.token = token;
        self.update_auth_header()?;
        self.update_cache_file()
        // }
        // Ok(())
    }

    pub fn print_message<T: fmt::Display>(&mut self, msg: T) -> Result<()> {
        self.terminal.writeln(msg)
    }

    pub fn clean(&mut self) -> Result<()> {
        self.update_token("".to_string())
    }
}
