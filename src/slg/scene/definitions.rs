use crate::rays::object::{GetIndex, GetObject, NamedObject, NamedObjectVector};
use crate::slg::scene::SceneObject;

#[derive(Default)]
pub struct SceneObjectDefinitions {
    objs: NamedObjectVector<SceneObject>,
}

impl SceneObjectDefinitions {
    pub fn define(&mut self, t: &SceneObject) { self.objs.define(t); }

    pub fn defined(&self, name: &String) -> bool { self.objs.defined(name) }

    pub fn size(&self) -> usize { self.objs.size() }

    pub fn names(&self) -> Vec<String> { self.objs.names() }

    pub fn delete(&mut self, name: &String) { self.objs.delete(name) }

    pub fn sorted_names(&self) -> Vec<String> { vec![] }
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
