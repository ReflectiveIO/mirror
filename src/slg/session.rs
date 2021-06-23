use crate::rays::Properties;
use crate::slg::engine::Engine;

use super::config::Config;
use super::film::Film;
use super::state::State;

#[derive(Default)]
pub struct Session {
    pub config: Config,
    pub engine: Engine,
    pub film: Film,
    state: State,
}

impl Session {
    pub fn new(config: Config, state: Option<State>, film: Option<Film>) -> Self {
        let state = match state {
            Some(val) => val,
            None => State::default(),
        };

        let film = match film {
            Some(val) => Film,
            None => Film::default(),
        };

        Session {
            config,
            engine: Default::default(),
            film,
            state,
        }
    }

    pub fn started(&self) -> bool {
        false
    }

    pub fn start() {}
    pub fn stop() {}

    pub fn editing(&self) -> bool {
        false
    }

    pub fn begin_edit(&self) {}
    pub fn end_edit(&self) {}

    pub fn paused(&self) -> bool {
        false
    }
    pub fn pause(&self) {}
    pub fn resume(&self) {}

    pub fn save_film_outputs(&self) {}
    pub fn save_film(&self, filename: &str) {}
    pub fn save_resume_file(&self, filename: &str) {}

    pub fn check_periodic_save(&self, force: bool) {}

    pub fn state(&self) -> State {
        State::default()
    }

    pub fn parse(&self, props: &Properties) {}
}
