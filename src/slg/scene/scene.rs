use crate::core::SceneTrait;
use crate::rays::color::Spectrum;
use crate::rays::device::IntersectionDevice;
use crate::rays::geometry::{
    BSphere, MotionSystem, Normal, Point, Ray, RayHit, Transform, Triangle, UV,
};
use crate::rays::mesh::{ExtMesh, ExtTriangleMesh};
use crate::rays::utils::HairFile;
use crate::rays::{Context, Dataset, NamedObject, Properties};
use crate::slg::bsdf::BSDF;
use crate::slg::cameras::{Camera, CameraTrait, CameraType, EnvironmentCamera};
use crate::slg::film::SampleResult;
use crate::slg::image_map::{ChannelSelectionType, ImageMap, ImageMapCache, WrapType};
use crate::slg::light::{LightSource, LightSourceDefinitions};
use crate::slg::material::{Material, MaterialDefinitions};
use crate::slg::scene::{ExtMeshCache, SceneObject, SceneObjectDefinitions};
use crate::slg::shape::TessellationType;
use crate::slg::textures::{Texture, TextureDefinitions, TextureMapping2D, TextureMapping3D};
use crate::slg::utils::PathVolumeInfo;
use crate::slg::volume::Volume;
use crate::slg::{EditAction, EditActionList};

type SceneRayType = i8;

pub struct Scene {
    // This volume is applied to rays hitting nothing
    pub default_world_volume: Volume,

    pub camera: Box<dyn CameraTrait>,

    // Mesh objects cache,
    pub ext_mesh_cache: ExtMeshCache,
    // Image maps cache
    pub img_map_cache: ImageMapCache,

    // Texture definitions
    pub tex_defs: TextureDefinitions,
    // Material definitions
    pub mat_defs: MaterialDefinitions,
    // SceneObject definitions
    pub obj_defs: SceneObjectDefinitions,
    // LightSource definitions
    pub light_defs: LightSourceDefinitions,

    pub dataset: Dataset,

    // the bounding sphere of the scene (including the camera)
    pub scene_bounding_sphere: BSphere,

    pub edit_actions: EditActionList,
    pub enable_parse_print: bool,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            default_world_volume: Volume::default(),
            camera: Box::new(EnvironmentCamera::new()),
            ext_mesh_cache: ExtMeshCache::default(),
            img_map_cache: ImageMapCache::default(),
            tex_defs: TextureDefinitions::default(),
            mat_defs: MaterialDefinitions::default(),
            obj_defs: SceneObjectDefinitions::default(),
            light_defs: LightSourceDefinitions::default(),
            dataset: Dataset::new(None),
            scene_bounding_sphere: BSphere::default(),
            edit_actions: EditActionList::new(),
            enable_parse_print: false,
        }
    }

    pub fn intersect(
        &self,
        device: Box<dyn IntersectionDevice>,
        ray_type: SceneRayType,
        vol_info: PathVolumeInfo,
        pass_through: f32,
        ray: &Ray,
        ray_hit: RayHit,
        bsdf: &BSDF,
        connection_throughput: &Spectrum,
        path_throughput: &Spectrum,
        sample_result: &SampleResult,
        back_tracing: bool,
    ) -> bool {
        false
    }

    pub fn preprocess_camera(&self, film_width: u32, film_height: u32, film_sub_region: &Vec<u32>) {
    }

    pub fn preprocess(
        &self,
        ctx: &Context,
        film_width: u32,
        film_height: u32,
        film_sub_region: &Vec<u32>,
        use_rt_mode: bool,
    ) {
    }

    pub fn to_properties(&self) -> Properties {
        Properties::default()
    }

    /* Methods to build and edit scene */

    pub fn define_image_map(&self, im: &ImageMap) {}

    pub fn define_image_map_args<T>(
        &mut self,
        name: &str,
        pixels: T,
        gamma: f32,
        channels: u32,
        width: u32,
        height: u32,
        selection_type: ChannelSelectionType,
        wrap_type: WrapType,
    ) {
        let mut map = ImageMap::alloc_image_map::<T>(gamma, channels, width, height, wrap_type);
        map.set_name(name);
        // @TODO memcpy pixels data
        map.reverse_gamma_correction();
        map.select_channel(selection_type);

        self.define_image_map(&map);
        self.edit_actions.add_action(EditAction::ImageMapsEdit);
    }

    pub fn is_image_map_defined(&self, name: &str) -> bool {
        false
    }

    // Mesh shape
    // Use one of the following methods, do not directly call extMeshCache.DefineExtMesh()

    pub fn define_mesh(&self, mesh: &ExtMesh) {}

    pub fn define_mesh_args(
        &self,
        shape_name: &str,
        ply_nb_verts: i64,
        ply_nb_tris: i64,
        p: Point,
        vi: Triangle,
        n: Normal,
        uvs: UV,
        cols: Box<Spectrum>,
        alphas: Vec<f32>,
    ) {
        todo!()
    }

    pub fn define_mesh_ext(
        &self,
        shape_name: &str,
        ply_nb_verts: i64,
        ply_nb_tris: i64,
        p: Point,
        vi: Triangle,
        n: Normal,
        uvs: Vec<UV>,
        cols: Vec<Box<Spectrum>>,
        alphas: Vec<Vec<f32>>,
    ) {
        todo!()
    }

    pub fn define_mesh_trans(&self, inst_mesh_name: &str, mesh_name: &str, trans: &Transform) {}
    pub fn define_mesh_ms(&self, mot_mesh_name: &str, mesh_name: &str, ms: &MotionSystem) {}

    pub fn set_mesh_vertex_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>) {
        todo!()
    }

    pub fn set_mesh_triangle_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>) {
        todo!()
    }

    pub fn define_stands(
        &self,
        shape_name: &str,
        strands_file: HairFile,
        tessel_type: TessellationType,
        adaptive_max_depth: u32,
        adaptive_error: f32,
        sold_side_count: u32,
        solid_cap_bottom: bool,
        solid_cap_top: bool,
        use_camera_positions: bool,
    ) {
        todo!()
    }

    pub fn is_texture_defined(&self, name: &str) {
        todo!()
    }

    pub fn is_material_defined(&self, name: &str) {
        todo!()
    }

    pub fn is_mesh_defined(&self, name: &str) {
        todo!()
    }

    pub fn parse(&self, props: &Properties) {
        todo!()
    }

    pub fn delete_object(&mut self, obj_name: &str) {
        todo!()
    }

    pub fn delete_light(&mut self, light_name: &str) {
        todo!()
    }

    pub fn duplicate_object_trans(&self, src: &str, dst: &str, trans: Transform, dst_obj_id: u32) {
        todo!()
    }

    pub fn duplicate_object_ms(&self, src: &str, dst: &str, ms: MotionSystem, dst_obj_id: u32) {
        todo!()
    }

    pub fn update_object_material(&mut self, obj_name: &str, mat_name: &str) {
        todo!()
    }

    pub fn update_object_transformation(&mut self, obj_name: &str, trans_mat: Vec<f32>) {
        todo!()
    }

    pub fn remove_unused_image_maps(&mut self) {
        todo!()
    }

    pub fn remove_unused_textures(&mut self) {
        todo!()
    }

    pub fn remove_unused_materials(&mut self) {
        todo!()
    }

    pub fn remove_unused_meshes(&mut self) {
        todo!()
    }

    pub fn load(filename: &str) -> Scene {
        Scene::new()
    }

    pub fn save(filename: &str, scene: &Scene) {
        todo!()
    }

    /* private methods */
    fn init(&self, scale: f32) {}

    fn parse_camera(&self, props: &Properties) {}
    fn parse_textures(&self, props: &Properties) {}
    fn parse_volumes(&self, props: &Properties) {}
    fn parse_materials(&self, props: &Properties) {}
    fn parse_shapes(&self, props: &Properties) {}
    fn parse_objects(&self, props: &Properties) {}
    fn parse_lights(&self, props: &Properties) {}

    fn get_texture(&self, name: &str) -> Texture {
        Texture::default()
    }

    fn create_camera(&self, props: &Properties) -> Camera {
        Camera::new(CameraType::ENVIRONMENT)
    }

    fn create_texture_mapping_2d(&self, prefix: &str, props: &Properties) -> TextureMapping2D {
        TextureMapping2D::default()
    }

    fn create_texture_mapping_3d(&self, prefix: &str, props: &Properties) -> TextureMapping3D {
        TextureMapping3D::default()
    }

    fn create_texture(&self, name: &str, props: &Properties) -> Texture {
        Texture::default()
    }

    fn create_volume(&self, id: u32, name: &str, props: &Properties) -> Volume {
        Volume::default()
    }

    fn create_material(&self, id: u32, name: &str, props: &Properties) -> Material {
        Material::default()
    }

    fn create_shape(&self, name: &str, props: &Properties) -> ExtTriangleMesh {
        ExtTriangleMesh::default()
    }

    fn create_object(&self, id: &str, name: &str, props: &Properties) -> SceneObject {
        SceneObject::default()
    }

    fn create_emission_map(&self, name: &str, props: &Properties) -> ImageMap {
        ImageMap::default()
    }

    fn create_light_source(&self, name: &str, props: &Properties) -> LightSource {
        LightSource::default()
    }

    fn create_inlined_mesh(
        &self,
        shape_name: &str,
        prop_name: &str,
        props: &Properties,
    ) -> ExtTriangleMesh {
        ExtTriangleMesh::default()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
