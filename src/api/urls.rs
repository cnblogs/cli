use const_format::formatcp;

pub const BLOG_BACKEND: &str = "https://i.cnblogs.com/api";
pub const OPENAPI: &str = "https://api.cnblogs.com/api";
pub const OAUTH: &str = "https://oauth.cnblogs.com";

pub const USER: &str = formatcp!("{}/users", OPENAPI);
pub const STATUS: &str = formatcp!("{}/statuses/", OPENAPI);
pub const COMMENTS_PATH: &str = "comments";

pub const POST_PREFIX: &str = formatcp!("{}/blogs", OPENAPI);
pub const BLOG_POST_PREFIX: &str = formatcp!("{}/blogposts", OPENAPI);
pub const BLOG_BACKEND_POST: &str = formatcp!("{}/potst", BLOG_BACKEND);
