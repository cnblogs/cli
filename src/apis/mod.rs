//! cnblogs 闪存接口模块
//!
//! 封装[cnblogs Api](https://api.cnblogs.com/Help#0aee001a01835c83a3277a500ffc9040)至以下模块中：
//!
//! - statuses: 闪存相关api。
//! - blogs: 博客相关
//! - news: 新闻相关
//! - questions: 问题相关
//! - edu: edu 相关
//! - user: 用户相关
//! - token: 认证相关
//! - marks: 收藏相关

pub mod ing;
pub mod token;

pub const OAUTH_CLIENT: &str = "https://api.cnblogs.com/token";
pub const OAUTH_TOKEN: &str = "https://oauth.cnblogs.com/connect/token";
pub const OAUTHORIZE: &str = "https://oauth.cnblogs.com/connect/authorize";
