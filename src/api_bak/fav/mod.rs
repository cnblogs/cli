pub mod get_list;

// Aka cnblogs wz
pub struct Fav {
    pat: String,
}

impl Fav {
    pub const fn new(pat: String) -> Self {
        Self { pat }
    }
}
