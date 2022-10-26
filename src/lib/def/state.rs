pub struct State {
    pub displays: Vec<Display>,
    pub is_active: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            displays: Vec::new(),
            is_active: false,
        }
    }
}
