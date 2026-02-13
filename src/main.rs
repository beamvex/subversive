#![deny(missing_docs)]
use std::cell::RefCell;
use std::rc::Rc;

use subversive::{debug, info};

const LOGO: &str = r"
  _________    ___.                            .__              
 /   _____/__ _\_ |_____  __ ___________  _____|__|__  __ ____  
 \_____  \|  |  \ __ \  \/ // __ \_  __ \/  ___/  \  \/ // __ \ 
 /        \  |  / \_\ \   /\  ___/|  | \/\___ \|  |\   /\  ___/ 
/_______  /____/|___  /\_/  \___  >__|  /____  >__| \_/  \___  >
        \/          \/          \/           \/              \/ 
";

pub struct Subversive<'a> {
    version: &'a str,
}

impl<'a> Subversive<'a> {
    #[must_use]
    pub fn new(version: &'a str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Subversive { version }))
    }

    #[must_use]
    pub const fn version(&self) -> &'a str {
        self.version
    }

    pub fn run(&self) {
        info!("\x1b[1;32m{LOGO}\x1b[0m");
        let version = self.version;
        info!("Subversive version: {version}");
    }
}

pub fn main() {
    debug!("Hello, world!");

    let subversive = Subversive::new("0.0.2");
    subversive.borrow().run();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        main();
    }
}
