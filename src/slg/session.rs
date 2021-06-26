use crate::rays::Properties;
use crate::slg::engine::{Engine, TilePathCPURenderEngine};

use super::config::Config;
use super::film::Film;
use super::state::State;

pub struct Session {
    pub config: Config,
    pub engine: Box<dyn Engine>,
    pub film: Film,
    state: State,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            config: Config::default(),
            engine: Box::new(TilePathCPURenderEngine::new()),
            film: Film::default(),
            state: State::default(),
        }
    }
}

impl Session {
    pub fn new(config: Config, state: Option<State>, film: Option<Film>) -> Self {
        let state = match state {
            Some(val) => val,
            None => State::default(),
        };

        let film = match film {
            Some(val) => val,
            None => Film::default(),
        };

        Session {
            config,
            engine: Box::new(TilePathCPURenderEngine::new()),
            film,
            state,
        }
    }

    pub fn started(&self) -> bool {
        true
    }

    pub fn start(&self) {
        warn!("@TODO: start the rendering");
    }

    pub fn stop(&self) {
        warn!("@TODO: stop the rendering");
    }

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
    pub fn dump(&self, filename: &str) {}

    pub fn check_periodic_save(&self, force: bool) {}

    pub fn state(&self) -> State {
        State::default()
    }

    pub fn parse(&self, props: &Properties) {}
}
