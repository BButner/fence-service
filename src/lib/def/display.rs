#[derive(Clone, Debug)]
pub struct Display {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub top: i32,
    pub left: i32,
    pub selected: bool,
}
