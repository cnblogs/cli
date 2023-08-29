pub mod get_post;

pub struct Post {
    pat: String,
}

impl Post {
    pub fn new(pat: String) -> Post {
        Post { pat }
    }
}
