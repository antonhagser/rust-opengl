use std::sync::{Arc, Mutex};

use crate::{App, system::System};

pub struct AppBuilder<'a, T> where T: System {
    name: Option<&'a str>,
    system: Option<T>,
    size: (u32, u32), // width & height
}

impl<'a, T> AppBuilder<'a, T> where T: System {
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn size(mut self, size: (u32, u32)) -> Self {
        self.size = size;
        self
    }

    pub fn system(mut self, system: T) -> App<'a, T> {
        self.system = Some(system);
        
        App {
            name: self.name.expect("no name was set for application"),
            system: Arc::new(Mutex::new(self.system.expect("not system was set"))),
            size: self.size,

            context: None,
        }
    }

    pub fn logger(self, rules: &'a str) -> Self {
        // todo: add logging to file if running release
        // assign logger rules to env logger
        std::env::set_var("RUST_LOG", rules);
        env_logger::init();
        trace!("Enabled engine logger with env: {:?}.", rules);

        self
    }
}

impl<'a, T> Default for AppBuilder<'a, T> where T: System {
    fn default() -> Self {
        AppBuilder {
            name: None,
            system: None,
            size: (1280, 720),
        }
    }
}