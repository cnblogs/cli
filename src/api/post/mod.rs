mod del_one;
pub mod get_count;
pub mod get_meta_list;
pub mod get_one;

pub struct Post {
    pat: String,
}

impl Post {
    pub fn new(pat: String) -> Post {
        Post { pat }
    }
}
