pub use distribution::DistributionLightStrategy;
pub use uniform::LightStrategyUniform;

use crate::rays::geometry::{Normal, Point};
use crate::rays::Properties;
use crate::slg::light::LightSource;
use crate::slg::Scene;

mod distribution;
mod uniform;

pub enum LightStrategyTask {
    Emit,
    Illuminate,
    InfiniteOnly,
}

pub enum LightStrategyType {
    Uniform,
    Power,
    LogPower,
    DlsCache,
}

impl ToString for LightStrategyType {
    fn to_string(&self) -> String {
        match self {
            LightStrategyType::Uniform => "UNIFORM".to_string(),
            LightStrategyType::Power => "POWER".to_string(),
            LightStrategyType::LogPower => "LOG_POWER".to_string(),
            LightStrategyType::DlsCache => "DLS_CACHE".to_string(),
            _ => {}
        }
    }
}

pub trait LightStrategy {
    fn get_type(&self) -> &LightStrategyType;
    fn get_tag(&self) -> &String;

    fn preprocess(&mut self, scene: &Scene, task_type: LightStrategyTask, real_time: bool);

    fn sample_lights(
        &self,
        u: f32,
        p: &Point,
        n: &Normal,
        is_volume: bool,
        pdf: f32,
    ) -> Option<Box<dyn LightSource>>;

    fn sample_light_pdf(
        &self,
        light: Box<dyn LightSource>,
        p: &Point,
        n: &Normal,
        is_volume: bool,
    ) -> f32;

    fn sample_lights2(&self, u: f32, pdf: f32) -> Option<Box<dyn LightSource>>;

    fn to_properties(&self) -> Properties;
}
