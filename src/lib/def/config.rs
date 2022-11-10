use serde::{Deserialize, Serialize};

use super::display::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub displays: Vec<Display>,
    pub ui_display_factor: f32,
    pub active_by_default: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            displays: Vec::new(),
            ui_display_factor: 0.125,
            active_by_default: false,
        }
    }
}
