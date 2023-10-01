pub mod create;
pub mod del_one;
pub mod get_comment_list;
pub mod get_count;
pub mod get_meta_list;
pub mod get_one;
pub mod get_one_raw;
pub mod search_self;
pub mod search_site;
pub mod update;

pub struct Post {
    pat: String,
}

impl Post {
    pub const fn new(pat: String) -> Self {
        Self { pat }
    }
}
