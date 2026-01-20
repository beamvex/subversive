use std::cell::RefCell;

pub struct Config<'a> {
    db_path: &'a str,
}

impl Default for Config<'static> {
    fn default() -> Self {
        Self { db_path: "db" }
    }
}

impl Config<'static> {
    pub fn get_db_path(&self) -> &'static str {
        self.db_path
    }

    pub fn set_db_path(&mut self, db_path: &'static str) {
        self.db_path = db_path;
    }
}

thread_local! {
    pub static CONFIG: RefCell<Config<'static>> = RefCell::new(Config::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        CONFIG.with(|config| {
            assert_eq!(config.borrow().get_db_path(), "db");
        });
    }

    #[test]
    fn test_config_set_db_path() {
        CONFIG.with(|config| {
            config.borrow_mut().set_db_path("test");
            assert_eq!(config.borrow().get_db_path(), "test");
        });
    }
}
