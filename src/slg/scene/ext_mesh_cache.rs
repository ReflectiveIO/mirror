use crate::rays::mesh::ExtMesh;

#[derive(Default)]
pub struct ExtMeshCache {
    delete_mesh_data: bool,
}

impl ExtMeshCache {
    pub fn new() -> Self {
        ExtMeshCache {
            delete_mesh_data: false,
        }
    }

    pub fn set_delete_mesh_data(&mut self, v: bool) {
        self.delete_mesh_data = v
    }

    // This method can be safely called only from Scene::define_mesh()
    pub fn define_ext_mesh(&mut self, mesh: &ExtMesh) {}

    pub fn set_mesh_vertex_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>) {}
    pub fn set_mesh_triangle_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>) {}

    pub fn is_ext_mesh_defined(&self, mesh_name: String) -> bool {
        false
    }

    // Not: before calls to delete_ext_mesh, be sure to not have an instance
    // referencing the geometry
    pub fn delete_ext_mesh(&self, mesh_name: &str) {}

    pub fn get_size(&self) -> usize {
        0
    }
    pub fn get_ext_mesh_names(&self) -> Vec<String> {
        vec![]
    }

    pub fn get_ext_mesh(&self, name: &str) -> Option<ExtMesh> {
        Some(ExtMesh::default())
    }

    pub fn get_ext_mesh_idx(&self, idx: usize) -> ExtMesh {
        ExtMesh::default()
    }

    pub fn get_ext_mesh_index(&self, name: &str) -> usize {
        0
    }
    pub fn get_ext_mesh_index_m(&self, m: &ExtMesh) -> usize {
        0
    }

    pub fn get_real_filename(&self, m: &ExtMesh) -> String {
        String::new()
    }

    pub fn get_sequence_filename(&self, m: &ExtMesh) -> String {
        String::new()
    }
}
