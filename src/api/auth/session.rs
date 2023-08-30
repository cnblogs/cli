use crate::infra::result::IntoResult;
use anyhow::{anyhow, Result};
use home::home_dir;
use std::fs;
use std::fs::{metadata, remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn remove_pat(path: &Path) -> Result<()> {
    if metadata(path).is_ok() {
        remove_file(path)?;
    }
    ().into_ok()
}

fn save_pat(pat: &str, path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(pat.as_bytes())?;
    ().into_ok()
}

fn get_cfg_path() -> Result<PathBuf> {
    let home = home_dir().ok_or(anyhow!("Can not get home dir"))?;
    home.join(".cnbrc").into_ok()
}

pub fn login(pat: &str) -> Result<PathBuf> {
    let cfg_path = get_cfg_path()?;
    let cfg_path = cfg_path.as_path();

    remove_pat(cfg_path)?;
    save_pat(pat, cfg_path)?;

    cfg_path.to_owned().into_ok()
}

pub fn logout() -> Result<PathBuf> {
    let cfg_path = get_cfg_path()?;
    let cfg_path = cfg_path.as_path();

    remove_pat(cfg_path)?;

    cfg_path.to_owned().into_ok()
}

pub fn get_pat() -> Result<String> {
    let cfg_path = get_cfg_path()?;
    let cfg_path = cfg_path.as_path();

    let err_msg = format!("Can not read {:?}, please login first.", cfg_path);
    let pat = fs::read_to_string(cfg_path).expect(&err_msg);
    pat.into_ok()
}
