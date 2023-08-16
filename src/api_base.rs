pub const BLOG_BACKEND: &str = "https://i.cnblogs.com/api";
#[macro_export]
macro_rules! blog_backend {
    ($($arg:tt)*) => {{
        use $crate::api_base::BLOG_BACKEND;
        let rest = format!($($arg)*);
        format!("{}{}", BLOG_BACKEND, rest)
    }};
}

pub const OPENAPI: &str = "https://api.cnblogs.com/api";
#[macro_export]
macro_rules! openapi {
    ($($arg:tt)*) => {{
        use $crate::api_base::OPENAPI;
        let rest = format!($($arg)*);
        format!("{}{}", OPENAPI, rest)
    }};
}

pub const OAUTH: &str = "https://oauth.cnblogs.com";
#[macro_export]
macro_rules! oauth {
    ($($arg:tt)*) => {{
        use $crate::api_base::OAUTH;
        let rest = format!($($arg)*);
        format!("{}{}", OAUTH, rest)
    }};
}
