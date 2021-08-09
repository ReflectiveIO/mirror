use downcast_rs::Downcast;

use crate::slg::bsdf::HitPoint;
use crate::slg::light::traits::LightSource;
use crate::slg::material::Material;
use crate::slg::Scene;

/// Intersectable LightSource interface
pub trait IntersectableLightSource: LightSource + Downcast {
    fn is_intersectable(&self) -> bool { true }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        self.light_material().get_avg_pass_through_transparency()
    }

    fn get_id(&self) -> u32 { self.light_material().get_light_id() }
    fn get_power(&self, scene: &Scene) -> f32;
    fn get_importance(&self) -> f32 { self.light_material().get_emitted_importance() }

    fn is_visible_indirect_diffuse(&self) -> bool {
        self.light_material().is_visible_indirect_diffuse()
    }

    fn is_visible_indirect_glossy(&self) -> bool {
        self.light_material().is_visible_indirect_glossy()
    }

    fn is_visible_indirect_specular(&self) -> bool {
        self.light_material().is_visible_indirect_specular()
    }

    fn get_radiance(
        &self,
        hit_point: &HitPoint,
        direct_pd_fa: Option<Vec<f32>>,
        emission_pd_fw: Option<Vec<f32>>,
    );

    fn light_material(&self) -> &Box<dyn Material>;
}
impl_downcast!(IntersectableLightSource);
