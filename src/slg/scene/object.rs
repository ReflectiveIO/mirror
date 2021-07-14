use crate::rays::mesh::ExtMesh;
use crate::slg::image_map::ImageMap;
use crate::slg::material::MaterialTrait;

pub struct SceneObject {
    mesh: ExtMesh,
    material: Box<dyn MaterialTrait>,
    object_id: u32,
    bake_map: ImageMap,
    bake_map_uv_index: u32,
    camera_in_visible: bool,
}

impl SceneObject {
    pub fn new(m: &ExtMesh, mt: &Box<dyn MaterialTrait>, id: u32, visible: bool) -> Self { Self {} }

    pub fn get_ext_mesh(&self) -> &ExtMesh { &self.mesh }

    pub fn get_material(&self) -> &Box<dyn MaterialTrait> { &self.material }

    pub fn get_id(&self) -> u32 { self.object_id }

    pub fn is_camera_in_visible(&self) -> bool { self.camera_in_visible }
}

impl Default for SceneObject {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
