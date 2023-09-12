pub mod get_body;
pub mod get_list;

pub struct News {
    pat: String,
}

impl News {
    pub const fn new(pat: String) -> Self {
        Self { pat }
    }
}
