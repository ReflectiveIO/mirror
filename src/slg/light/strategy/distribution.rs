use crate::rays::geometry::{Normal, Point};
use crate::rays::utils::Distribution1D;
use crate::rays::Properties;
use crate::slg::light::strategy::{LightStrategy, LightStrategyTask, LightStrategyType};
use crate::slg::light::traits::LightSource;
use crate::slg::Scene;

pub struct DistributionLightStrategy {
    light_strategy_type: LightStrategyType,
    lights_distribution: Option<Distribution1D>,
    scene: Option<Scene>,
}

impl DistributionLightStrategy {
    pub fn new(t: LightStrategyType) -> Self {
        Self {
            light_strategy_type: t,
            lights_distribution: None,
            scene: None,
        }
    }
}

impl LightStrategy for DistributionLightStrategy {
    fn get_type(&self) -> &LightStrategyType { &self.light_strategy_type }

    fn get_tag(&self) -> &String { todo!() }

    fn preprocess(&mut self, scene: &Scene, task_type: LightStrategyTask, real_time: bool) {
        self.scene = Some(scene.clone())
    }

    fn sample_lights(
        &self,
        u: f32,
        p: &Point,
        n: &Normal,
        is_volume: bool,
        pdf: f32,
    ) -> Option<Box<dyn LightSource>> {
        self.sample_lights2(u, pdf)
    }

    fn sample_light_pdf(
        &self,
        light: Box<dyn LightSource>,
        p: &Point,
        n: &Normal,
        is_volume: bool,
    ) -> f32 {
        // match &self.lights_distribution {
        //     Some(lightsDistribution) => {
        //         lightsDistribution.pdf_discrete(self.light.light_scene_index)
        //     },
        //     None => 0.0,
        // }
        0.0
    }

    fn sample_lights2(&self, u: f32, pdf: f32) -> Option<Box<dyn LightSource>> {
        // match &self.lights_distribution {
        //     Some(lightsDistribution) => {
        //         let light_index = lightsDistribution.sample_discrete(u, pdf);
        //         if pdf > 0.0 {
        //             self.scene.light_defs.get_light_sources()[light_index]
        //         } else {
        //             None
        //         }
        //     },
        //     None => None,
        // }
        None
    }

    fn to_properties(&self) -> Properties {
        let mut props = Properties::new();
        props.set("light.strategy.type", self.get_type().to_string());
        props
    }
}
