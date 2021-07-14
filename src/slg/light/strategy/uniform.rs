use crate::rays::geometry::{Normal, Point};
use crate::rays::Properties;
use crate::slg::light::strategy::{
    DistributionLightStrategy, LightStrategy, LightStrategyTask, LightStrategyType,
};
use crate::slg::light::traits::LightSource;
use crate::slg::Scene;

pub struct LightStrategyUniform {
    inner: DistributionLightStrategy,
}

impl LightStrategyUniform {
    pub fn new() -> Self {
        Self {
            inner: DistributionLightStrategy::new(LightStrategyType::Uniform),
        }
    }

    pub fn get_object_type() -> &'static LightStrategyType { &LightStrategyType::Uniform }

    pub fn get_object_tag() -> &'static str { "UNIFORM" }
}

impl From<&Properties> for LightStrategyUniform {
    fn from(_: &Properties) -> Self { todo!() }
}

impl Into<Properties> for LightStrategyUniform {
    fn into(self) -> Properties {
        let mut props = Properties::new();
        props.set("light.strategy.type", self.get_type().to_string());
        props
    }
}

impl LightStrategy for LightStrategyUniform {
    fn get_type(&self) -> &LightStrategyType { Self::get_object_type() }

    fn get_tag(&self) -> &String { &Self::get_object_tag().to_string() }

    fn preprocess(&mut self, scene: &Scene, task_type: LightStrategyTask, real_time: bool) {
        todo!()
    }

    fn sample_lights(
        &self,
        u: f32,
        p: &Point,
        n: &Normal,
        is_volume: bool,
        pdf: f32,
    ) -> Option<Box<dyn LightSource>> {
        self.inner.sample_lights(u, p, n, is_volume, pdf)
    }

    fn sample_light_pdf(
        &self,
        light: Box<dyn LightSource>,
        p: &Point,
        n: &Normal,
        is_volume: bool,
    ) -> f32 {
        self.inner.sample_light_pdf(light, p, n, is_volume)
    }

    fn sample_lights2(&self, u: f32, pdf: f32) -> Option<Box<dyn LightSource>> {
        self.inner.sample_lights2(u, pdf)
    }
}
