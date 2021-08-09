use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};

pub enum TextureType {
    ConstFloat,
    ConstFloat3,
    ImageMap,
    ScaleTex,
    FresnelApproxN,
    FresnelApproxK,
    MixTex,
    AddTex,
    SubtractTex,
    HitPointColor,
    HitPointAlpha,
    HitPointGrey,
    NormalMapTex,
    BlackBodyTex,
    IrregularDataTex,
    DensityGridTex,
    AbsTex,
    ClampTex,
    BilerpTex,
    ColorDepthTex,
    HsvTex,
    DivideTex,
    RemapTex,
    ObjectIdTex,
    ObjectIdColorTex,
    ObjectIdNormalizedTex,
    DotProductTex,
    PowerTex,
    LessThanTex,
    GreaterThanTex,
    RoundingTex,
    ModuloTex,
    ShadingNormalTex,
    PositionTex,
    SplitFloat3,
    MakeFloat3,
    BrightContrastTex,
    HitPointVertexAov,
    HitPointTriangleAov,
    TriplanarTex,
    RandomTex,
    BevelTex,
    DistortTex,
    BombingTex,
    BlenderBlend,
    BlenderClouds,
    BlenderDistortedNoise,
    BlenderMagic,
    BlenderMarble,
    BlenderMusgrave,
    BlenderNoise,
    BlenderStucci,
    BlenderWood,
    BlenderVoronoi,
    CheckerBoard2d,
    CheckerBoard3d,
    CloudTex,
    FbmTex,
    Marble,
    Dots,
    Brick,
    Windy,
    Wrinkled,
    UvTex,
    BandTex,
    WireFrameTex,
    FresnelColorTex,
    FresnelConstTex,
}

pub trait Texture {
    fn get_type(&self) -> TextureType;
    fn get_float_value(&self, hp: &HitPoint) -> f32;
    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum;
    fn y(&self) -> f32;
    fn filter(&self) -> f32;

    // Used for bump/normal mapping support
    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }
    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {}
    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
    }
    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties;
}

impl NamedObject for Box<dyn Texture> {
    fn get_name(&self) -> &String { &String::from("texture") }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl PartialEq<&Box<dyn Texture>> for Box<dyn Texture> {
    fn eq(&self, other: &&Box<dyn Texture>) -> bool { todo!() }
}

impl PartialEq for Box<dyn Texture> {
    fn eq(&self, other: &Self) -> bool { todo!() }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Self { self.clone() }
}
