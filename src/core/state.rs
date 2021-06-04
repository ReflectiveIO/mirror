/// State is used to resume a rendering from a previous saved point.
#[derive(Default, Debug)]
pub struct State;

impl State {
    /// Create a new empty State.
    pub fn new() -> Self {
        State
    }

    /// Creates a new State from a file.
    pub fn load(filename: &str) -> Self {
        State::default()
    }

    /// Serializes a State in a file.
    pub fn save(&self, filename: &str) {}
}
