//! cnblogs 闪存接口模块
//!
//! 实现封装[cnblogs Api](https://api.cnblogs.com/Help#0aee001a01835c83a3277a500ffc9040)中的`Statuses`。
//!
//! - 获取最新一条闪存内容    https://api.cnblogs.com/api/statuses/recent      
//! - 发布闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments   
//! - 获取闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments
//! - 删除闪存评论           https://api.cnblogs.com/api/statuses/{statusId}/comments/{id}
//! - 发布闪存               https://api.cnblogs.com/api/statuses
//! - 删除闪存               https://api.cnblogs.com/api/statuses/{id}
//! - 根据类型获取闪存列表    https://api.cnblogs.com/api/statuses/@{type}?pageIndex={pageIndex}&pageSize={pageSize}&tag={tag}
//! - 根据Id获取闪存         https://api.cnblogs.com/api/statuses/{id}
//!
