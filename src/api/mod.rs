pub mod auth;
pub mod ing;
pub mod post;
pub mod user;

pub const BLOG_BACKEND: &str = "https://i.cnblogs.com/api";
#[macro_export]
macro_rules! blog_backend {
    ($($arg:tt)*) => {{
        use $crate::api::BLOG_BACKEND;
        format!("{}{}", BLOG_BACKEND, format_args!($($arg)*))
    }};
}

pub const OPENAPI: &str = "https://api.cnblogs.com/api";
#[macro_export]
macro_rules! openapi {
    ($($arg:tt)*) => {{
        use $crate::api::OPENAPI;
        format!("{}{}", OPENAPI, format_args!($($arg)*))
    }};
}

pub const OAUTH: &str = "https://oauth.cnblogs.com";
#[macro_export]
macro_rules! oauth {
    ($($arg:tt)*) => {{
        use $crate::api::OAUTH;
        format!("{}{}", OAUTH, format_args!($($arg)*))
    }};
}
