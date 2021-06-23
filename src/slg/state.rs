#[derive(Default)]
pub struct State {
    tag: String,
}

impl State {
    pub fn new(tag: &str) -> Self {
        State::default()
    }

    pub fn check_engine_tag(&self, tag: &str) {}
    pub fn get_engine_tag(&self) -> &String {
        &self.tag
    }

    pub fn save_serialized(&self, filename: &str) {}
    pub fn load_serialized(filename: &str) -> State {
        State::default()
    }

    fn serialize(version: u32) {}
}
