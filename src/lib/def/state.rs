use super::{config::Config, display::Display};

pub struct State {
    pub config: Config,
    pub is_active: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            is_active: false,
        }
    }
}
