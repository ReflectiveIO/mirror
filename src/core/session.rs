use crate::core::config::Config;
use crate::core::film::Film;
use crate::core::state::State;
use crate::rays::properties::Properties;

/// Session executes a rendering based on the Config provided.
#[derive(Default, Debug)]
pub struct Session {
    /// A reference to the Config
    config: Config,
    /// Pointer to the State.
    state: State,
    /// The reference to the Film
    film: Film,
    /// A Properties container with the statistics
    stats: Properties,
}

impl Session {
    /// Create a new Session using the provided config.
    ///
    /// config is the Config used to create the rendering session.
    /// The Config is not deleted by the destructor. and optional state
    /// & film used to resume rendering.
    pub fn new(config: &Config, state: &State, film: &Film) -> Self {
        Session::default()
    }

    /// Create a new Session using the provided config and state & film.
    pub fn restart(config: &Config, state_filename: &str, film_filename: &str) -> Self {
        Session::default()
    }

    /// Starts the rendering.
    pub fn start(&self) {}

    /// Stops the rendering.
    pub fn stop(&self) {}

    /// It can be used to check if the session has been started.
    pub fn started(&self) -> bool {
        false
    }

    /// Stops the rendering and allows to edit the scene.
    pub fn begin_edit(&self) {}

    /// Ends the scene editing and start the rendering again.
    pub fn end_edit(&self) {}

    /// It can be used to check if the session is in scene editing mode.
    pub fn editing(&self) -> bool {
        false
    }

    /// Pause the rendering.
    pub fn pause(&self) {}

    /// Resume the rendering.
    pub fn resume(&self) {}

    /// It can be used to check if the session is paused.
    pub fn paused(&self) -> bool {
        false
    }

    /// It can be used to check if the rendering is over.
    pub fn done(&self) -> bool {
        false
    }

    /// Used to wait for the end of the rendering.
    pub fn wait_for_done(&self) {}

    /// Used to wait for the next frame with real-time render engines like
    /// RTPATHOCL. it does nothing with other render engines.
    pub fn wait_for_frame(&self) {}

    /// Updates the statistics.
    pub fn update_stats(&self) {}

    /// Dynamic edit the definition of Config properties.
    ///
    /// props are the Properties with the definition of: film.imagepipeline(s).*
    /// (including radiance channel scales), film.outputs.*, film.width or film.height.
    pub fn parse(&self, props: &Properties) {}

    /// Save all the rendering related information (the Config, Scene, State and Film)
    /// in a file for a later restart. The resume file extension must be ".rsm".
    pub fn dump(filename: &str) {}

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn film(&self) -> &Film {
        &self.film
    }

    pub fn stats(&self) -> &Properties {
        &self.stats
    }
}
