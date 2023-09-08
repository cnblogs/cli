pub mod info;

pub struct User {
    pat: String,
}

impl User {
    pub const fn new(pat: String) -> Self {
        Self { pat }
    }
}
