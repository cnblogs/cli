use std::fs::File;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::models::user::UserInfo;

const CACHE_DIR: &str = ".cnblogs";
const CACHE: &str = "token";

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Cache {
    pub id: u64,
    pub blog_id: u64,
    pub blog_app: String,
    pub username: String,
    pub token: String,
}

impl Cache {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(buf)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    /// 检查 token 是否为空
    pub fn is_token_empty(&self) -> bool {
        self.token.trim().is_empty()
    }

    /// 验证缓存数据的有效性
    pub fn is_valid(&self) -> bool {
        !self.is_token_empty() && self.id > 0
    }
}

impl From<UserInfo> for Cache {
    fn from(value: UserInfo) -> Self {
        Self {
            id: value.account_id,
            blog_id: value.blog_id,
            blog_app: value.blog_app,
            username: value.display_name,
            token: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CacheDir {
    pub cache_dir: PathBuf,
    pub cache_file: PathBuf,
    pub home_dir: PathBuf,
}

impl CacheDir {
    pub fn new() -> Result<Self> {
        let home_dir =
            home::home_dir().ok_or_else(|| anyhow!("无法获取用户家目录，退出。".red()))?;
        let cache_dir = PathBuf::from(CACHE_DIR);
        let cache_file = PathBuf::from(CACHE);

        Ok(Self {
            cache_dir,
            cache_file,
            home_dir,
        })
    }

    /// 初始化，检查文件夹和目录是否存在，如果不存在则创建
    pub fn init(&self) -> Result<()> {
        self.ensure_dir()?;
        self.ensure_file()
    }

    /// 获取缓存目录的完整路径
    pub fn full_cache_dir(&self) -> PathBuf {
        self.home_dir.join(&self.cache_dir)
    }

    /// 获取缓存文件的完整路径
    pub fn full_cache_file(&self) -> PathBuf {
        self.full_cache_dir().join(&self.cache_file)
    }

    /// 检查缓存目录是否存在，不存在创建。
    pub fn ensure_dir(&self) -> Result<()> {
        let p = self.full_cache_dir();
        if !p.exists() {
            fs::create_dir_all(p)?;
        }
        Ok(())
    }

    /// 检查缓存文件是否存在，不存在创建。
    pub fn ensure_file(&self) -> Result<()> {
        let p = self.full_cache_file();
        if !p.exists() {
            fs::File::create(p)?;
        }
        Ok(())
    }

    /// 写入缓存文件
    pub fn write(&self, buf: &[u8]) -> Result<()> {
        let mut f = File::create(self.full_cache_file())?;
        f.write_all(buf)?;
        Ok(())
    }

    /// 读取缓存文件
    pub fn read(&self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        let mut f = File::open(self.full_cache_file())?;
        f.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
