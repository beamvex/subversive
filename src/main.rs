use std::cell::RefCell;
use std::rc::Rc;

const LOGO: &str = r#"
  _________    ___.                            .__              
 /   _____/__ _\_ |_____  __ ___________  _____|__|__  __ ____  
 \_____  \|  |  \ __ \  \/ // __ \_  __ \/  ___/  \  \/ // __ \ 
 /        \  |  / \_\ \   /\  ___/|  | \/\___ \|  |\   /\  ___/ 
/_______  /____/|___  /\_/  \___  >__|  /____  >__| \_/  \___  >
        \/          \/          \/           \/              \/ 
"#;

pub struct Subversive<'a> {
    version: &'a str,
}

impl<'a> Subversive<'a> {
    pub fn new(version: &'a str) -> Rc<RefCell<Subversive<'a>>> {
        Rc::new(RefCell::new(Subversive { version }))
    }

    pub fn version(&self) -> &'a str {
        self.version
    }

    pub fn run(&self) {
        println!("\x1b[1;32m{LOGO}\x1b[0m");
        println!("Subversive version: {}", self.version);
    }
}

pub fn main() {
    println!("Hello, world!");

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
