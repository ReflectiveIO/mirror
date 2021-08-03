use crate::rays::color::Spectrum;
use crate::rays::geometry::Point;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub enum MasonryBond {
    Flemish,
    Running,
    English,
    Herringbone,
    Basket,
    Ketting,
}

pub struct BrickTexture {
    mapping: TextureMapping3D,

    tex1: Box<dyn Texture>,
    tex2: Box<dyn Texture>,
    tex3: Box<dyn Texture>,

    bond: MasonryBond,
    offset: Point,

    brick_width: f32,
    brick_height: f32,
    brick_depth: f32,

    proportion: f32,
    inv_proportion: f32,
    run: f32,

    mortar_width: f32,
    mortar_height: f32,
    mortar_depth: f32,
    mortar_size: f32,

    initial_brick_width: f32,
    initial_brick_height: f32,
    initial_brick_depth: f32,

    modulation_bias: f32,
}

impl BrickTexture {
    pub fn new(
        mapping: TextureMapping3D,

        tex1: Box<dyn Texture>,
        tex2: Box<dyn Texture>,
        tex3: Box<dyn Texture>,

        brick_width: f32,
        mut brick_height: f32,
        mut brick_depth: f32,
        mortar_size: f32,
        run: f32,
        b: &str,
        modulation_bias: f32,
    ) -> Self {
        let mut run: f32 = 0.0;
        let bond: MasonryBond;
        let mut offset: Point = Point::default();
        match b {
            "stacked" => {
                bond = MasonryBond::Running;
                run = 0.0;
            },
            "flemish" => {
                bond = MasonryBond::Flemish;
            },
            "english" => {
                bond = MasonryBond::English;
                run = 0.25;
            },
            "herringbone" => {
                bond = MasonryBond::Herringbone;
            },
            "basket" => {
                bond = MasonryBond::Basket;
            },
            "chain link" => {
                bond = MasonryBond::Ketting;
                run = 1.25;
                offset = Point::new(0.25, -1.0, 0.0);
            },
            _ => {
                bond = MasonryBond::Running;
                offset = Point::new(0.0, -0.5, 0.0);
            },
        }

        let (mut proportion, mut inv_proportion) = (0.0, 0.0);
        if bond == MasonryBond::Herringbone || bond == MasonryBond::Basket {
            proportion = (brick_width / brick_width).floor();
            brick_height = brick_width;
            brick_depth = brick_height;
            inv_proportion = 1.0 / proportion;
        }

        let mortar_width = mortar_size / brick_width;
        let mortar_height = mortar_size / brick_height;
        let mortar_depth = mortar_size / brick_depth;

        Self {
            mapping,
            tex1,
            tex2,
            tex3,
            bond,
            offset,
            brick_width,
            brick_height,
            brick_depth,
            proportion,
            inv_proportion,
            run,
            mortar_width,
            mortar_height,
            mortar_depth,
            mortar_size,
            initial_brick_width: brick_width,
            initial_brick_height: brick_height,
            initial_brick_depth: brick_depth,
            modulation_bias,
        }
    }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }

    pub fn get_texture1(&self) -> &Box<dyn Texture> { &self.tex1 }

    pub fn get_texture2(&self) -> &Box<dyn Texture> { &self.tex2 }

    pub fn get_texture3(&self) -> &Box<dyn Texture> { &self.tex3 }

    pub fn get_bond(&self) -> &MasonryBond { &self.bond }

    pub fn get_offset(&self) -> &Point { &self.offset }

    pub fn get_initial_brick_width(&self) -> f32 { self.initial_brick_width }

    pub fn get_initial_brick_height(&self) -> f32 { self.initial_brick_height }

    pub fn get_initial_brick_depth(&self) -> f32 { self.initial_brick_depth }

    pub fn get_brick_width(&self) -> f32 { self.brick_width }

    pub fn get_brick_height(&self) -> f32 { self.brick_height }

    pub fn get_brick_depth(&self) -> f32 { self.brick_depth }

    pub fn get_mortar_size(&self) -> f32 { self.mortar_size }

    pub fn get_proportion(&self) -> f32 { self.proportion }

    pub fn get_inv_proportion(&self) -> f32 { self.inv_proportion }

    pub fn get_run(&self) -> f32 { self.run }

    pub fn get_mortar_width(&self) -> f32 { self.mortar_width }

    pub fn get_mortar_height(&self) -> f32 { self.mortar_height }

    pub fn get_mortar_depth(&self) -> f32 { self.mortar_depth }

    pub fn get_modulation_bias(&self) -> f32 { self.modulation_bias }
}

impl Texture for BrickTexture {
    fn get_type(&self) -> TextureType { TextureType::Brick }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.tex1.add_referenced_textures(v);
        self.tex2.add_referenced_textures(v);
        self.tex3.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {
        self.tex1.add_referenced_image_maps(v);
        self.tex2.add_referenced_image_maps(v);
        self.tex3.add_referenced_image_maps(v);
    }

    fn update_texture_references(&mut self, old_tex: &dyn Texture, new_tex: &dyn Texture) {
        if self.tex1.as_ref() == old_tex {
            self.tex1 = Box::new(new_tex);
        }
        if self.tex2.as_ref() == old_tex {
            self.tex2 = Box::new(new_tex);
        }
        if self.tex3.as_ref() == old_tex {
            self.tex3 = Box::new(new_tex);
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
