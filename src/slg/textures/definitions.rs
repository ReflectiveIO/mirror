use crate::rays::object::{GetIndex, GetObject, NamedObject, NamedObjectVector};
use crate::slg::textures::Texture;

#[derive(Default)]
pub struct TextureDefinitions {
    textures: NamedObjectVector<Texture>,
}

impl TextureDefinitions {
    pub fn define(&mut self, t: &Texture) { self.textures.define(t); }

    pub fn defined(&self, name: &String) -> bool { self.textures.defined(name) }

    pub fn size(&self) -> usize { self.textures.size() }

    pub fn names(&self) -> Vec<String> { self.textures.names() }

    pub fn delete(&mut self, name: &String) { self.textures.delete(name) }

    pub fn sorted_names(&self) -> Vec<String> { vec![] }
}

impl GetObject<String, Texture> for TextureDefinitions {
    fn get(&self, key: &String) -> &Texture { self.textures.get(key) }
}

impl GetObject<usize, Texture> for TextureDefinitions {
    fn get(&self, key: &usize) -> &Texture { self.textures.get(key) }
}

impl GetIndex<String> for TextureDefinitions {
    fn index(&self, key: &String) -> usize { self.textures.index(key) }
}

impl GetIndex<Texture> for TextureDefinitions {
    fn index(&self, key: &Texture) -> usize { self.textures.index(key) }
}
