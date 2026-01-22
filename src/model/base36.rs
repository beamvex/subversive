use std::fmt::Display;

#[derive(Debug)]
pub struct Base36 {
    string: String,
}

impl Base36 {
    pub fn new(string: String) -> Self {
        Self { string }
    }
    pub fn get_string(&self) -> &String {
        &self.string
    }
}

impl Display for Base36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
