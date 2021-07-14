use std::str::FromStr;

pub use distribution::DistributionLightStrategy;
pub use uniform::LightStrategyUniform;

use crate::rays::geometry::{Normal, Point};
use crate::rays::Properties;
use crate::slg::light::traits::LightSource;
use crate::slg::Scene;

mod distribution;
mod uniform;

pub enum LightStrategyTask {
    Emit,
    Illuminate,
    InfiniteOnly,
}

#[derive(Eq, PartialEq)]
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
            _ => {
                panic!("Unknown light strategy type")
            },
        }
    }
}

impl FromStr for LightStrategyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNIFORM" => Ok(LightStrategyType::Uniform),
            "POWER" => Ok(LightStrategyType::Power),
            "LOG_POWER" => Ok(LightStrategyType::LogPower),
            "DLS_CACHE" => Ok(LightStrategyType::DlsCache),
            _ => Err(String::from("Unknown light strategy type string")),
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

    fn to_properties(&self) -> Properties { Properties::new() }
}

pub struct LightStrategies;

impl LightStrategies {
    pub fn parse(props: &Properties) -> Option<LightStrategyType> {
        let t: String = props
            .get("light.strategy.type")
            .unwrap_or(String::from(LightStrategyUniform::get_object_tag()));

        match LightStrategyType::from_str(&*t) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    pub fn from(props: &Properties) -> Box<dyn LightStrategy> {
        let t = Self::parse(props).unwrap();

        let strategy = match t {
            LightStrategyType::Uniform => LightStrategyUniform::from(props),
            LightStrategyType::Power => LightStrategyUniform::from(props),
            _ => LightStrategyUniform::from(props),
        };

        Box::new(strategy)
    }
}
