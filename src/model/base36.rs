#[derive(Debug)]
pub struct Base36 {
    string: String,
}

impl Base36 {
    pub fn new(string: String) -> Self {
        Self { string }
    }
}
