use crate::slg::textures::Texture;

#[derive(Default)]
pub struct TextureDefinitions {
    textures: Vec<Texture>,
}

impl TextureDefinitions {
    pub fn is_texture_defined(&self, name: String) -> bool {
        self.textures.is_obj_defined(name)
    }

    pub fn define_texture(&mut self, t: Texture) {}

    pub fn get_texture(&self, name: String) -> &Texture {
        self.textures.get_obj_by_name(name)
    }

    pub fn get_texture_idx(&self, index: usize) -> &Texture {
        self.textures.get_obj_by_index(index)
    }

    pub fn get_texture_index(&self, name: String) -> usize {
        self.textures.get_index_by_name(name)
    }

    pub fn get_texture_index_t(&self, t: &Texture) -> usize {
        self.textures.get_index_by_obj(t)
    }

    pub fn get_size(&self) -> usize {
        self.textures.len()
    }

    pub fn get_texture_names(&self) -> Vec<String> {
        self.textures.get_names()
    }

    pub fn delete_texture(&mut self, name: String) {
        self.textures.delete_obj(name)
    }

    pub fn get_texture_sorted_names(&self) -> Vec<String> {
        vec![]
    }
}
