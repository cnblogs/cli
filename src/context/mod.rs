pub mod config;
pub mod output;

use core::time;
use std::fmt;

use anyhow::{Ok, Result};
use reqwest::{
    Client, ClientBuilder,
    header::{self, HeaderMap},
};

use crate::context::{
    config::{Cache, CacheDir},
    output::Terminal,
};

// const FILENAME: &str = ".cnblogs/token";

#[derive(Debug)]
pub struct Context {
    pub terminal: Terminal,
    pub client: Client,
    pub json: bool,
    pub cache: Cache,
    pub path: CacheDir,
}

impl Context {
    pub fn new() -> Result<Self> {
        let path = CacheDir::new()?;
        path.init()?;
        let buf = path.read()?;
        let cache = Cache::from_bytes(&buf).unwrap_or_default();
        let terminal = Terminal::new();
        let mut headers = HeaderMap::new();

        if !cache.token.is_empty() {
            let header_value = format!("Bearer {}", cache.token);
            headers.append(header::AUTHORIZATION, header_value.parse()?);
            headers.append("authorization-type", "pat".parse()?);
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .connect_timeout(time::Duration::from_secs(10))
            .https_only(true)
            .build()?;

        Ok(Self {
            terminal,
            client,
            json: false,
            cache,
            path,
        })
    }

    pub const fn set_json(&mut self, json: bool) {
        self.json = json;
    }

    pub fn print_message<T: fmt::Display>(&mut self, msg: T) -> Result<()> {
        self.terminal.writeln(msg)
    }

    /// 清空缓存信息
    pub fn clean(&self) -> Result<()> {
        let c = Cache::default();
        let buf = c.to_bytes()?;
        self.path.write(&buf)
    }

    /// 保存之缓存文件
    pub fn save_cache(&self, cache: Cache) -> Result<()> {
        self.path.write(&cache.to_bytes()?)
    }
}
