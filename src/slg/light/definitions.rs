use crate::rays::object::{GetObject, NamedObjectVector};
use crate::rays::Properties;
use crate::slg::light::strategy::*;
use crate::slg::light::traits::{EnvLightSource, LightSource};
use crate::slg::light::{LightSourceType, TriangleLight};
use crate::slg::material::Material;
use crate::slg::Scene;

pub struct LightSourceDefinitions {
    light_group_count: usize,
    light_type_count: Vec<usize>,
    lights: NamedObjectVector<Box<dyn LightSource>>,
    intersectable_light_sources: Vec<TriangleLight>,
    env_light_sources: Vec<Box<dyn EnvLightSource>>,

    // 2 tables to go from mesh index and triangle index to light index
    light_index_offset_by_mesh_index: Vec<usize>,
    light_index_by_tri_index: Vec<usize>,

    emit_light_strategy: Box<dyn LightStrategy>,
    illuminate_light_strategy: Box<dyn LightStrategy>,
    infinite_light_strategy: Box<dyn LightStrategy>,
}

impl LightSourceDefinitions {
    pub fn new() -> Self {
        Self {
            light_type_count: vec![],
            lights: NamedObjectVector::new(),
            intersectable_light_sources: vec![],
            env_light_sources: vec![],
            light_index_offset_by_mesh_index: vec![],
            light_index_by_tri_index: vec![],

            // strategies
            emit_light_strategy: Box::new(LightStrategyUniform::new()),
            illuminate_light_strategy: Box::new(LightStrategyUniform::new()),
            infinite_light_strategy: Box::new(LightStrategyUniform::new()),
            light_group_count: 1,
        }
    }

    pub fn define(&mut self, l: &Box<dyn LightSource>) { self.lights.define(l); }

    pub fn defined(&self, name: &str) -> bool { self.lights.defined(&name.to_string()) }

    pub fn size(&self) -> usize { self.lights.size() }

    pub fn names(&self) -> Vec<String> { self.lights.names() }

    pub fn delete(&mut self, name: &str) { self.lights.delete(&name.to_string()) }

    pub fn set_light_strategy(&mut self, props: &Properties) {
        if let Some(t) = LightStrategies::parse(props) {
            if &t != self.emit_light_strategy.get_type() {
                self.emit_light_strategy = LightStrategies::from(props);
            }

            if &t != self.illuminate_light_strategy.get_type() {
                self.illuminate_light_strategy = LightStrategies::from(props);
            }

            if &t != self.infinite_light_strategy.get_type() {
                self.infinite_light_strategy = LightStrategies::from(props)
            }
        }
    }

    pub fn update_visibility_maps(&mut self, scene: &Scene, rt: bool) {}

    pub fn get_light_source(&self, name: &str) -> Box<dyn LightSource> {
        Box::new(TriangleLight::new())
    }

    pub fn delete_light_source_start_with(&mut self, prefix: &str) {}

    pub fn delete_light_source_by_material(&mut self, mat: &Box<dyn Material>) {}

    /* Following methods require preprocess() */

    pub fn get_light_source_by_mesh_and_tri_index(
        &self,
        mesh_index: usize,
        tri_index: usize,
    ) -> TriangleLight {
        TriangleLight::new()
    }

    pub fn get_light_group_count(&self) -> usize { self.light_group_count }

    pub fn get_light_type_count(&self, t: LightSourceType) -> usize { self.light_type_count.len() }

    pub fn get_light_type_counts(&self) -> &Vec<usize> { &self.light_type_count }

    pub fn get_light_sources(&self) -> Vec<Box<dyn LightSource>> { self.lights.values() }

    pub fn get_env_light_sources(&self) -> &Vec<Box<dyn EnvLightSource>> { &self.env_light_sources }

    pub fn get_intersectable_light_sources(&self) -> &Vec<TriangleLight> {
        &self.intersectable_light_sources
    }

    pub fn get_light_index_offset_by_mesh_index(&self) -> &Vec<usize> {
        &self.light_index_offset_by_mesh_index
    }

    pub fn get_light_index_by_tri_index(&self) -> &Vec<usize> { &self.light_index_by_tri_index }

    pub fn get_emit_light_strategy(&self) -> &Box<dyn LightStrategy> { &self.emit_light_strategy }

    pub fn get_illuminate_light_strategy(&self) -> &Box<dyn LightStrategy> {
        &self.illuminate_light_strategy
    }

    pub fn get_infinite_light_strategy(&self) -> &Box<dyn LightStrategy> {
        &self.infinite_light_strategy
    }

    // Update light_group_count, evn_light_sources, intersectable_light_sources,
    // light_index_offset_by_mesh_index, light_strategy_type, etc.
    // This is called by Scene::preprocess()
    fn preprocess(&self, scene: &Scene, real_time: bool) {}
}

impl Default for LightSourceDefinitions {
    fn default() -> Self { LightSourceDefinitions::new() }
}

impl GetObject<String, Box<dyn LightSource>> for LightSourceDefinitions {
    fn get(&self, key: &String) -> &Box<dyn LightSource> { self.lights.get(key) }
}
