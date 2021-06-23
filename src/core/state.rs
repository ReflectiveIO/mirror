use crate::slg;

/// State is used to resume a rendering from a previous saved point.
#[derive(Default)]
pub struct State {
    state: slg::State,
}

impl State {
    /// Create a new empty State.
    pub fn new() -> Self {
        State {
            state: slg::State::default(),
        }
    }

    /// Creates a new State from a file.
    pub fn load(&mut self, filename: &str) {
        self.state = slg::State::load_serialized(filename)
    }

    /// Serializes a State in a file.
    pub fn save(&self, filename: &str) {
        self.state.save_serialized(filename)
    }
}
