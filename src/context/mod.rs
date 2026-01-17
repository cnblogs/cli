pub mod output;

use core::time;
use std::{
    fmt,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Ok, Result, anyhow};
use owo_colors::OwoColorize;
use reqwest::{
    Client, ClientBuilder,
    header::{self, HeaderMap},
};

use crate::context::output::Terminal;

const FILENAME: &str = ".cnblogs/token";

#[derive(Debug)]
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
        let terminal = Terminal::new();
        let mut token = token;
        let home_dir =
            home::home_dir().ok_or_else(|| anyhow!("无法获取用户家目录，退出。".red()))?;
        let file = PathBuf::from(FILENAME);
        let full_path = home_dir.join(&file);
        let mut cache = Self::ensure_file(full_path.clone())?;
        let mut headers = HeaderMap::new();

        if token.is_empty() {
            let _ = cache.read_to_string(&mut token);
        } else {
            cache.write_all(token.as_bytes())?;
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

    pub fn ensure_file(full_path: PathBuf) -> Result<File> {
        let cnblogs = full_path
            .parent()
            .ok_or_else(|| anyhow!("获取`~/.cnblogs`文件夹失败， 退出。"))?;

        if !full_path.exists() {
            if !cnblogs.exists() {
                fs::create_dir_all(cnblogs)?;
            }
            Ok(fs::File::create(full_path.clone())?)
        } else if full_path.exists() {
            Ok(fs::File::open(full_path)?)
        } else {
            Ok(fs::File::create(full_path.clone())?)
        }
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
