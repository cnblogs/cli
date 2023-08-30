pub mod info;

pub struct User {
    pat: String,
}

impl User {
    pub fn new(pat: String) -> User {
        User { pat }
    }
}
