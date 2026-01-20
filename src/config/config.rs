use std::cell::RefCell;

pub struct Config {
    db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: String::from("db"),
        }
    }
}

impl Config {
    pub fn get_db_path(&self) -> &String {
        &self.db_path
    }

    pub fn set_db_path(&mut self, db_path: String) {
        self.db_path = db_path;
    }
}

thread_local! {
    pub static CONFIG: RefCell<Config> = RefCell::new(Config::default());
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
            config.borrow_mut().set_db_path(String::from("test"));
            assert_eq!(config.borrow().get_db_path(), "test");
        });
    }
}
