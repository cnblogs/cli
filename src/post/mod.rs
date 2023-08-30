pub mod get_one;
mod get_meta_list;

pub struct Post {
    pat: String,
}

impl Post {
    pub fn new(pat: String) -> Post {
        Post { pat }
    }
}
