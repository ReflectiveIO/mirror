use super::material::Material;
use super::material::MaterialTrait;

#[derive(Default)]
pub struct MaterialDefinitions {
    materials: Vec<Material>,
}

impl MaterialDefinitions {
    pub fn is_material_defined(&self, name: String) -> bool {
        self.materials.is_obj_defined(name)
    }

    pub fn define_material(&mut self, t: Material) {}

    pub fn get_material(&self, name: String) -> &Box<dyn MaterialTrait> {
        self.materials.get_obj_by_name(name)
    }

    pub fn get_material_idx(&self, index: usize) -> &Box<dyn MaterialTrait> {
        self.materials.get_obj_by_index(index)
    }

    pub fn get_material_index(&self, name: String) -> usize {
        self.materials.get_index_by_name(name)
    }

    pub fn get_material_index_t(&self, t: &Box<dyn MaterialTrait>) -> usize {
        self.materials.get_index_by_obj(t)
    }

    pub fn get_size(&self) -> usize {
        self.materials.len()
    }

    pub fn get_material_names(&self) -> Vec<String> {
        self.materials.get_names()
    }

    pub fn delete_material(&mut self, name: String) {
        self.materials.delete_obj(name)
    }

    pub fn get_material_sorted_names(&self) -> Vec<String> {
        vec![]
    }
}
