use config::ConfigError;
use serde::Deserialize;

use super::film::Film;
use super::scene::Scene;
use super::state::State;
use crate::rays::Properties;
use crate::slg;

/// Config stores all the configuration settings used to render a scene.
#[derive(Default)]
pub struct Config {
    config: slg::Config,
}

impl Config {
    /// Create a new Config using the provided Properties and optional scene.
    pub fn new(props: &Properties, scene: Option<Scene>) -> Self {
        let mut config = Self {
            config: slg::Config::default(),
        };

        if let Some(scene) = scene {
            config.config = slg::Config::new(&props, Some(scene.scene));
        } else {
            config.config = slg::Config::new(&props, None);
        }

        return config;
    }

    /// Create a new Config using the provided binary file.
    ///
    /// filename is the binary file use to build the new config. The extension
    /// for the binary format must be ".bcf".
    pub fn build(filename: &str) -> Self {
        Self {
            config: slg::Config::load(filename),
        }
    }

    /// Create a new Config using the provided resume binary file.
    ///
    /// * filename: is the binary file used to build the new Config. Then
    ///   extension for
    /// the binary format must be ".rsm".
    /// * state: the pointer to the render state will be returned here.
    /// * film: the pointer to the film will be returned here.
    pub fn resume(filename: &str, state: &mut State, film: &mut Film) -> Self { Config::default() }

    /// Returns a reference to the Properties used to create the Config.
    pub fn get_properties(&self) -> &Properties { &self.config.properties }

    /// Returns the Property with the given name or the default value
    /// if it has not been defined.
    pub fn get<'de, T: Deserialize<'de>>(&self, name: &str) -> Result<T, ConfigError> {
        self.config.get(name)
    }

    /// Returns a reference to all Properties (including Default values)
    /// defining the Config.
    pub fn to_properties(&self) -> &Properties { self.config.to_properties() }

    // /// Returns a reference to the Scene used in the Config.
    // pub fn scene(&self) -> &Scene {
    //     &self.scene
    // }

    /// Sets configuration Properties with new values. This method can be
    /// used only when the Config is not in use by a Session.
    pub fn parse(&self, props: &Properties) { self.config.parse(props); }

    /// Deletes any configuration Property starting with the given prefix.
    /// this method can be used only when the Config is not in use by a Session.
    pub fn delete(&mut self, prefix: &str) { self.config.delete(prefix); }

    /// Return the configured Film width, height, sub-region width, height,
    /// and if sub-region is enabled.
    ///
    /// * width: is where the configured Film width is returnd if the pointer is
    ///   not NULL.
    /// * height: is where the configured Film height is returned if the pointer
    ///   is not NULL.
    /// * region: is an array of 4 values with the horizontal (followed by the
    ///   vertical) begin
    /// and end of the Film region to render (in pixels).
    ///
    /// * return: true if there is a sub-region to render, false otherwise.
    pub fn film_size(&self, width: u32, height: u32, region: [u32; 4]) -> bool {
        warn!("@TODO: get film size");
        false
    }

    /// Save all the scene related information (the core Config, and Scene) in a
    /// file.
    pub fn save(&self, filename: &str) { slg::Config::save_serialized(filename, &self.config); }

    /// Save all the scene related information (the core Config, and scene) in a
    /// directory using text format for the SDL. This performs the same work
    /// of FILESAVER render engine.
    pub fn export(&self, dirname: &str) {
        warn!("@TODO: export scene in FileSaverEngine");
    }

    /// Save all the scene related information in glTF 2.0 format.
    pub fn export_gltf(&self, filename: &str) { warn!("@TODO: export scene in glTF 2.0 format") }

    /// Returns false if a (long) kernel compilation time is required at the
    /// start of the rendering. True otherwise.
    pub fn has_cached_kernels(&self) -> bool { self.config.has_cached_kernels() }

    /// Returns a Properties container with all Default values.
    pub fn default_properties() -> Properties { slg::Config::default_properties() }
}
