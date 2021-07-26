use crate::rays::mesh::ExtMesh;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::image_map::ImageMap;
use crate::slg::material::MaterialTrait;
use crate::slg::scene::ExtMeshCache;

pub struct SceneObject {
    mesh: Box<dyn ExtMesh>,
    material: Box<dyn MaterialTrait>,
    object_id: u32,
    bake_map: ImageMap,
    bake_map_uv_index: u32,
    camera_in_visible: bool,
}

impl SceneObject {
    pub fn new(m: Box<dyn ExtMesh>, mt: Box<dyn MaterialTrait>, id: u32, visible: bool) -> Self {
        Self {
            mesh: m,
            material: mt,
            object_id: id,
            bake_map: ImageMap::default(),
            bake_map_uv_index: 0,
            camera_in_visible: visible,
        }
    }

    pub fn get_ext_mesh(&self) -> &Box<dyn ExtMesh> { &self.mesh }

    pub fn get_material(&self) -> &Box<dyn MaterialTrait> { &self.material }

    pub fn get_id(&self) -> u32 { self.object_id }

    pub fn is_camera_in_visible(&self) -> bool { self.camera_in_visible }

    pub fn to_properties(&self, imc: &ExtMeshCache, real_filename: bool) -> Properties { todo!() }
}

impl Default for SceneObject {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl NamedObject for SceneObject {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
