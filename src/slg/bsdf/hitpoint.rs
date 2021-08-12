use crate::rays::color::Spectrum;
use crate::rays::geometry::{Frame, Normal, Point, Transform, Vector, UV};
use crate::rays::mesh::ExtMesh;
use crate::slg::volume::Volume;

pub struct HitPoint {
    // The incoming direction. It is the eye_dir when from_light = false
    // and light_dir when from_light = true
    pub fixed_dir: Vector,
    pub p: Point,
    pub geometry_n: Normal,
    pub interpolated_n: Normal,
    pub shade_n: Normal,

    // The "main" UV coordinate of the hit point (from UV set 0)
    pub default_uv: UV,

    // Note: dpdu and dpdv are orthogonal to shading normal (i.e not geometry normal)
    pub dpdu: Vector,
    pub dpdv: Vector,
    pub dndu: Vector,
    pub dndv: Vector,

    // Mesh information
    pub mesh: Option<Box<dyn ExtMesh>>,
    pub triangle_index: u32,
    pub triangle_bari_coord1: f32,
    pub triangle_bari_coord2: f32,

    pub pass_through_event: f32,

    // Transformation from local object to world reference frame
    pub local_to_world: Transform,

    /* Interior and exterior volume (this includes volume priority system
     * computation and scene default world volume) */
    pub interior_volume: Volume,
    pub exterior_volume: Volume,

    pub object_id: u32,

    pub from_light: bool,
    pub into_object: bool,

    // If i go here going trough a shadow transparency. it can be used to disable MIS.
    pub through_shadow_transparency: bool,
}

impl Default for HitPoint {
    fn default() -> Self { todo!() }
}

impl HitPoint {
    pub fn get_frame(&self) -> Frame { Frame::new(&self.dpdu, &self.dpdv, &self.shade_n) }

    pub fn get_landing_geometry_n(&self) -> Normal {
        (if self.into_object { 1.0 } else { -1.0 }) * &self.geometry_n
    }

    pub fn get_landing_interpolated_n(&self) -> Normal {
        (if self.into_object { 1.0 } else { -1.0 }) * &self.interpolated_n
    }

    pub fn get_landing_shade_n(&self) -> Normal {
        (if self.into_object { 1.0 } else { -1.0 }) * &self.shade_n
    }

    pub fn get_uv(&self, data_index: u32) -> UV {
        if let Some(mesh) = &self.mesh {
            if data_index == 0 {
                self.default_uv.clone()
            } else {
                mesh.interpolate_tri_uv(
                    self.triangle_index,
                    self.triangle_bari_coord1,
                    self.triangle_bari_coord2,
                    data_index,
                )
            }
        } else {
            UV::default()
        }
    }

    pub fn get_color(&self, data_index: u32) -> Spectrum {
        if let Some(mesh) = &self.mesh {
            mesh.interpolate_tri_color(
                self.triangle_index,
                self.triangle_bari_coord1,
                self.triangle_bari_coord2,
                data_index,
            )
        } else {
            Spectrum::from(1.0)
        }
    }

    pub fn get_alpha(&self, data_index: u32) -> f32 {
        if let Some(mesh) = &self.mesh {
            mesh.interpolate_tri_alpha(
                self.triangle_index,
                self.triangle_bari_coord1,
                self.triangle_bari_coord2,
                data_index,
            )
        } else {
            1.0
        }
    }

    pub fn get_vertex_aov(&self, data_index: u32) -> f32 {
        if let Some(mesh) = &self.mesh {
            mesh.interpolate_tri_vertex_aov(
                self.triangle_index,
                self.triangle_bari_coord1,
                self.triangle_bari_coord2,
                data_index,
            )
        } else {
            0.0
        }
    }

    pub fn get_tri_aov(&self, data_index: u32) -> f32 {
        if let Some(mesh) = &self.mesh {
            mesh.get_tri_aov(self.triangle_index, data_index)
        } else {
            0.0
        }
    }
}
