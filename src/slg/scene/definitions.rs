use crate::rays::mesh::ExtMesh;
use crate::rays::object::{GetIndex, GetObject, NamedObjectVector};
use crate::slg::light::LightSourceDefinitions;
use crate::slg::scene::SceneObject;

#[derive(Default)]
pub struct SceneObjectDefinitions {
    objs: NamedObjectVector<SceneObject>,
}

impl SceneObjectDefinitions {
    pub fn define(&mut self, t: &SceneObject) { self.objs.define(t); }

    pub fn define_intersectable_lights(&self, defs: &LightSourceDefinitions, obj: SceneObject) {
        todo!()
    }

    pub fn defined(&self, name: &String) -> bool { self.objs.defined(name) }

    pub fn size(&self) -> usize { self.objs.size() }

    pub fn names(&self) -> Vec<String> { self.objs.names() }

    pub fn delete(&mut self, name: &String) { self.objs.delete(name) }

    pub fn sorted_names(&self) -> Vec<String> { vec![] }

    pub fn update_mesh_references(
        &self,
        old: Option<Box<dyn ExtMesh>>,
        new: &Box<dyn ExtMesh>,
    ) -> Vec<SceneObject> {
        todo!()
    }
}

impl GetObject<String, SceneObject> for SceneObjectDefinitions {
    fn get(&self, key: &String) -> &SceneObject { self.objs.get(key) }
}

impl GetObject<usize, SceneObject> for SceneObjectDefinitions {
    fn get(&self, key: &usize) -> &SceneObject { self.objs.get(key) }
}

impl GetIndex<String> for SceneObjectDefinitions {
    fn index(&self, key: &String) -> usize { self.objs.index(key) }
}

impl GetIndex<SceneObject> for SceneObjectDefinitions {
    fn index(&self, key: &SceneObject) -> usize { self.objs.index(key) }
}
