use super::config::Config;
use super::film::Film;
use super::state::State;
use crate::rays::Properties;
use crate::slg::engine::{Engine, EngineType, TilePathCPURenderEngine};

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

    pub fn started(&self) -> bool { self.engine.started() }

    pub fn start(&mut self) {
        trace!("start the rendering");

        // I need to allocate a new film because the current one has already been
        // used. For instance, it can happen when stopping and starting the same
        // session.
        if self.film.initialized() {
            self.film = self.config.alloc_film()
        }

        self.engine.start(&self.film)
    }

    pub fn stop(&self) {
        trace!("stop the rendering");
        self.check_periodic_save(true);
        self.engine.stop()
    }

    pub fn editing(&self) -> bool { self.engine.editing() }

    pub fn begin_edit(&self) { self.engine.begin_scene_edit() }

    pub fn end_edit(&mut self) {
        // Make a copy of the edit actions
        let edit_actions = self.config.scene.edit_actions.clone();
        if self.engine.get_type() != EngineType::RTPathOCLRenderEngine
            && self.engine.get_type() != EngineType::RTPathCPURenderEngine
        {
            info!("Edit actions: {:?}", edit_actions);

            // RTPATHs handle film Rest on their own
            if edit_actions.has_any_action() {
                self.film.reset(false);
            }
        }
        self.engine.end_scene_edit(&edit_actions);
    }

    pub fn paused(&self) -> bool { self.engine.paused() }

    pub fn pause(&self) { self.engine.pause() }

    pub fn resume(&self) { self.engine.resume() }

    pub fn save_film_outputs(&self) {
        // Ask the Engine to update the film
        self.engine.update_film();
        self.film.output();
    }

    pub fn save_film(&self, filename: &str) {
        trace!("saving film: {}", filename);

        // Ask the Engine to update film
        self.engine.update_film();

        if self.config.get("film.safesave").unwrap() {
            // @TODO: safe save
        } else {
            // @TODO: Film::SaveSerialized()
        }
    }

    pub fn dump(&self, filename: &str) {}

    pub fn check_periodic_save(&self, force: bool) {}

    pub fn state(&self) -> Option<&State> {
        if !self.paused() {
            error!("A rendering state can be retrieved only while the rendering session is paused");
        }
        Some(self.engine.get_state())
    }

    pub fn parse(&mut self, props: &Properties) {
        assert!(self.engine.started());

        if (props.has("film.width")
            && props.get::<u32>("film.width").unwrap() != self.film.get_width())
            || (props.has("film.height")
                && props.get::<u32>("film.height").unwrap() != self.film.get_height())
        {
            // I have to use a special procedure if the parsed props
            // include a film size
            self.engine.begin_film_edit();

            // Update render config properties
            self.config.update_film_properties(props);

            // Delete the old film and create the new film
            self.film = self.config.alloc_film();

            // I have to update the camera
            self.config.scene.preprocess_camera(
                self.film.get_width(),
                self.film.get_height(),
                self.film.get_sub_region(),
            );
            self.engine.end_film_edit();
        } else {
            self.film.parse(props);
            self.config.update_film_properties(props);
        }
    }
}
