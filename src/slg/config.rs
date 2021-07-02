use config::ConfigError;
use serde::Deserialize;

use crate::rays::Properties;
use crate::slg::engine::{Engine, TilePathCPURenderEngine};
use crate::slg::film::filter::Filter;
use crate::slg::film::Film;
use crate::slg::Scene;

#[derive(Default)]
pub struct Config {
    pub properties: Properties,
    pub scene: Scene,

    props_cache: Properties,
    save_additional_props: Properties,
    default_properties: Properties,
    allocated_scene: bool,
}

impl Config {
    pub fn new(props: &Properties, scene: Option<Scene>) -> Self {
        let scene = match scene {
            Some(val) => val,
            None => Scene::default(),
        };

        Config {
            scene,
            properties: props.clone(),
            props_cache: Properties::default(),
            default_properties: Properties::default(),
            save_additional_props: Properties::default(),
            allocated_scene: false,
        }
    }

    /// Returns false if a (long) kernel compilation time is required at the start of
    /// the rendering. True otherwise.
    pub fn has_cached_kernels(&self) -> bool {
        false
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, name: &str) -> Result<T, ConfigError> {
        self.properties.get(name)
    }

    pub fn parse(&self, props: &Properties) {}
    pub fn delete_all_film_image_pipelines_properties(&self) {}
    pub fn update_film_properties(&self, props: &Properties) {}
    pub fn delete(&mut self, prefix: &str) {}

    pub fn alloc_pixel_filter() -> Filter {
        Filter::default()
    }

    pub fn alloc_film(&self) -> Film {
        Film::default()
    }

    pub fn alloc_sample_shared_data() {}
    pub fn alloc_sampler() {}
    pub fn alloc_engine() -> Box<dyn Engine> {
        Box::new(TilePathCPURenderEngine::new())
    }

    /// Returns a reference to all Properties (including
    /// default values) defining the RenderConfig.
    pub fn to_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn default_properties() -> Properties {
        Properties::default()
    }

    pub fn load(filename: &str) -> Config {
        Config::default()
    }

    pub fn save_serialized(filename: &str, config: &Config) {}

    fn save() {}
    fn read() {}
    fn init_default_properties() {}
}
