use crate::slg::scene::SceneObject;

#[derive(Default)]
pub struct SceneObjectDefinitions {
    scene_objects: Vec<SceneObject>,
}

impl SceneObjectDefinitions {
    pub fn is_scene_object_defined(&self, name: String) -> bool {
        self.scene_objects.is_obj_defined(name)
    }

    pub fn define_scene_object(&mut self, t: SceneObject) {}

    pub fn get_scene_object(&self, name: String) -> &SceneObject {
        self.scene_objects.get_obj_by_name(name)
    }

    pub fn get_scene_object_idx(&self, index: usize) -> &SceneObject {
        self.scene_objects.get_obj_by_index(index)
    }

    pub fn get_scene_object_index(&self, name: String) -> usize {
        self.scene_objects.get_index_by_name(name)
    }

    pub fn get_scene_object_index_t(&self, t: &SceneObject) -> usize {
        self.scene_objects.get_index_by_obj(t)
    }

    pub fn get_size(&self) -> usize {
        self.scene_objects.len()
    }

    pub fn get_scene_object_names(&self) -> Vec<String> {
        self.scene_objects.get_names()
    }

    pub fn delete_scene_object(&mut self, name: String) {
        self.scene_objects.delete_obj(name)
    }

    pub fn get_scene_object_sorted_names(&self) -> Vec<String> {
        vec![]
    }
}
