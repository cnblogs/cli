use const_format::formatcp;

pub const BLOG_BACKEND: &str = "https://i.cnblogs.com/api";
pub const OPENAPI: &str = "https://api.cnblogs.com/api";
pub const OAUTH: &str = "https://oauth.cnblogs.com";

// 修复：正确的参数数量
pub const USER: &str = formatcp!("{}/users", OPENAPI);
pub const STATUS: &str = formatcp!("{}/statuses/", OPENAPI);
pub const COMMENTS_PATH: &str = "comments";
