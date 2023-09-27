use crate::api::user::info::UserInfo;
use crate::display::normal::fmt_err;
use crate::infra::result::IntoResult;
use anyhow::Result;
use std::fmt::Write;
use std::path::PathBuf;

pub fn login(cfg_path: &Result<PathBuf>) -> String {
    match cfg_path {
        Ok(pb) => format!("PAT was saved in {:?}", pb),
        Err(e) => fmt_err(e),
    }
}

pub fn logout(cfg_path: &Result<PathBuf>) -> String {
    match cfg_path {
        Ok(pb) => format!("{:?} was successfully removed", pb),
        Err(e) => fmt_err(e),
    }
}

pub fn user_info(info: &Result<UserInfo>) -> Result<String> {
    let info = match info {
        Ok(info) => info,
        Err(e) => return fmt_err(e).into_ok(),
    };

    let mut buf = String::new();
    {
        let buf = &mut buf;
        write!(buf, "{}", info.display_name)?;
        if info.is_vip {
            write!(buf, " VIP")?;
        }
        writeln!(buf)?;
        writeln!(
            buf,
            "{} Following {} Followers",
            info.following_count, info.followers_count
        )?;
        writeln!(buf, "ID     {}", info.blog_id)?;
        writeln!(buf, "Joined {}", info.joined)?;
        writeln!(buf, "Blog   https://www.cnblogs.com/{}", info.blog_app)?;
    }
    buf.into_ok()
}
