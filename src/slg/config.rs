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

        let mut config = Config {
            scene,
            properties: props.clone(),
            default_properties: Properties::default(),
            save_additional_props: Properties::default(),
            allocated_scene: false,
        };
        config.parse(props);
        return config;
    }

    /// Returns false if a (long) kernel compilation time is required at the start of
    /// the rendering. True otherwise.
    pub fn has_cached_kernels(&self) -> bool {
        false
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, name: &str) -> Result<T, ConfigError> {
        self.properties.get(name)
    }

    pub fn parse(&mut self, props: &Properties) {
        if props
            .get::<bool>("debug.config.parse.print")
            .unwrap_or(false)
        {
            print!(
                "========Config::parse========\n{:?},===================",
                props
            )
        }

        self.properties.merge(props);
        self.scene.enable_parse_print = self
            .properties
            .get("debug.scene.parse.print")
            .unwrap_or(false);

        self.update_film_properties(props);
        self.scene.light_defs.set_light_strategy(&self.properties);

        // Update the Camera
        let mut film_full_width: u32 = 0;
        let mut film_full_height: u32 = 0;
        let mut film_sub_region: [u32; 4] = [0, 0, 0, 0];

        Film::parse_film_size(
            props,
            &mut film_full_width,
            &mut film_full_height,
            &mut film_sub_region,
        );
        self.scene
            .camera
            .unwrap()
            .update(film_full_width, film_full_height, film_sub_region);
    }

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
