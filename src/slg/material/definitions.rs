use super::material::{Material, MaterialTrait};
use crate::rays::object::{GetIndex, GetObject, NamedObject, NamedObjectVector};

#[derive(Default)]
pub struct MaterialDefinitions {
    mats: NamedObjectVector<Box<dyn MaterialTrait>>,
}

impl MaterialDefinitions {
    pub fn define(&mut self, t: &Box<dyn MaterialTrait>) { self.mats.define(t); }

    pub fn defined(&self, name: &String) -> bool { self.mats.defined(name) }

    pub fn size(&self) -> usize { self.mats.size() }

    pub fn names(&self) -> Vec<String> { self.mats.names() }

    pub fn delete(&mut self, name: &String) { self.mats.delete(name) }

    pub fn sorted_names(&self) -> Vec<String> { vec![] }
}

impl GetObject<String, Box<dyn MaterialTrait>> for MaterialDefinitions {
    fn get(&self, key: &String) -> &Box<dyn MaterialTrait> { self.mats.get(key) }
}

impl GetObject<usize, Box<dyn MaterialTrait>> for MaterialDefinitions {
    fn get(&self, key: &usize) -> &Box<dyn MaterialTrait> { self.mats.get(key) }
}

impl GetIndex<String> for MaterialDefinitions {
    fn index(&self, key: &String) -> usize { self.mats.index(key) }
}

impl GetIndex<Box<dyn MaterialTrait>> for MaterialDefinitions {
    fn index(&self, key: &Box<dyn MaterialTrait>) -> usize { self.mats.index(key) }
}
