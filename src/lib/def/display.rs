use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Display {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub top: i32,
    pub left: i32,
    pub selected: bool,
}
