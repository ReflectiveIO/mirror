use std::any::Any;

use downcast_rs::Downcast;

use crate::core::SceneTrait;
use crate::rays::color::Spectrum;
use crate::rays::device::IntersectionDevice;
use crate::rays::geometry::*;
use crate::rays::mesh::{ExtMesh, ExtTriangleMesh, Mesh};
use crate::rays::object::{GetIndex, GetObject, NamedObject};
use crate::rays::utils::HairFile;
use crate::rays::{Context, Dataset, Properties};
use crate::slg::bsdf::BSDF;
use crate::slg::cameras::{BaseCamera, Camera, CameraType, EnvironmentCamera};
use crate::slg::film::SampleResult;
use crate::slg::image_map::{ChannelSelectionType, ImageMap, ImageMapCache, WrapType};
use crate::slg::light::traits::{
    IntersectableLightSource, LightSource, NotIntersectableLightSource,
};
use crate::slg::light::{LightSourceDefinitions, Spotlight, TriangleLight};
use crate::slg::material::{Material, MaterialDefinitions, MaterialTrait};
use crate::slg::scene::{ExtMeshCache, SceneObject, SceneObjectDefinitions};
use crate::slg::shape::TessellationType;
use crate::slg::textures::{Texture, TextureDefinitions, TextureMapping2D, TextureMapping3D};
use crate::slg::utils::PathVolumeInfo;
use crate::slg::volume::Volume;
use crate::slg::{EditAction, EditActionList};

type SceneRayType = i8;

pub const TRIANGLE_LIGHT_POSTFIX: &str = "__triangle__light__";

pub struct Scene {
    // This volume is applied to rays hitting nothing
    pub default_world_volume: Option<Volume>,

    pub camera: Option<Box<dyn Camera>>,

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

    // the bounding Sphere of the scene (including the camera)
    pub scene_bounding_sphere: BSphere,

    pub edit_actions: EditActionList,
    pub enable_parse_print: bool,
}

impl Scene {
    pub fn new(scale: f32) -> Self {
        let mut edit_actions = EditActionList::new();
        edit_actions.add_all_action();

        let mut img_map_cache = ImageMapCache::default();
        img_map_cache.set_image_resize(scale);
        img_map_cache.define_image_map(&ImageMap::random());

        Scene {
            default_world_volume: None,
            camera: None,
            ext_mesh_cache: ExtMeshCache::default(),
            img_map_cache,
            tex_defs: TextureDefinitions::default(),
            mat_defs: MaterialDefinitions::default(),
            obj_defs: SceneObjectDefinitions::default(),
            light_defs: LightSourceDefinitions::default(),
            dataset: Dataset::new(None),
            scene_bounding_sphere: BSphere::default(),
            edit_actions,
            enable_parse_print: false,
        }
    }

    pub fn from(props: &Properties, scale: f32) {
        let scene = Scene::new(scale);
        scene.parse(props)
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

    pub fn to_properties(&self, real_filename: bool) -> Properties {
        let mut props = Properties::new();

        // Write the camera information
        if self.camera.is_some() {
            props.merge(
                &self
                    .camera
                    .unwrap()
                    .to_properties(&self.img_map_cache, real_filename),
            );
        }

        // Save all not intersectable light sources
        for name in self.light_defs.names() {
            let light_source = self.light_defs.get_light_source(name.as_str());
            if let Some(light_source) = light_source.downcast_ref::<Spotlight>() {
                props.merge(&light_source.to_properties(&self.img_map_cache, real_filename));
            }
        }

        // Get the sorted list of texture names according their dependencies
        for name in self.tex_defs.sorted_names() {
            // Skip all textures starting with Implicit-ConstFloatTexture(3)
            // because they are expanded inline
            if name.starts_with("Implicit-ConstFloatTexture") {
                continue;
            }
            let texture = self.tex_defs.get(&name);
            props.merge(&texture.to_properties(&self.img_map_cache, real_filename));
        }

        // Get the sorted list of material names according their dependencies
        let mat_names = self.mat_defs.sorted_names();

        for name in mat_names {
            let material = self.mat_defs.get(&name);
            // Check if it is a volume
            if let Some(volume) = material.downcast_ref::<Volume>() {
                props.merge(&volume.to_properties(&self.img_map_cache, real_filename));
            }
        }

        // Set the default world interior/exterior volume if required
        if self.default_world_volume.is_some() {
            let vol: Box<dyn MaterialTrait> = Box::new(self.default_world_volume.unwrap());
            let index = self.mat_defs.index(&vol);
            props.set(
                "scene.world.volume.default",
                self.mat_defs.get(&index).get_name().as_str(),
            );
        }

        // Writes the materials information
        for name in mat_names {
            let material = self.mat_defs.get(&name);
            // Check if it is not a volume
            if !material.is::<Volume>() {
                props.merge(&material.to_properties(&self.img_map_cache, real_filename));
            }
        }

        // Write the object information
        for i in 0..self.obj_defs.size() {
            let object = self.obj_defs.get(&i);
            props.merge(&object.to_properties(&self.ext_mesh_cache, real_filename));
        }

        return props;
    }

    /* Methods to build and edit scene */

    pub fn define_image_map(&mut self, im: &ImageMap) {
        self.img_map_cache.define_image_map(im);
        self.edit_actions.add_action(EditAction::ImageMapsEdit);
    }

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
        self.img_map_cache.is_image_map_defined(name)
    }

    // Mesh shape
    // Use one of the following methods, do not directly call
    // extMeshCache.DefineExtMesh()

    pub fn define_mesh(&mut self, mesh: &Box<dyn ExtMesh>) {
        let shape_name: &String = mesh.get_name();

        if self.ext_mesh_cache.is_ext_mesh_defined(shape_name) {
            // A replacement for an existing mesh
            let old = self.ext_mesh_cache.get_ext_mesh(shape_name);

            // Replace old mesh direct references with new one and get the list
            // of scene objects referencing the old mesh
            let modified_obj_lists = self.obj_defs.update_mesh_references(old, mesh);

            for object in modified_obj_lists {
                if !object.get_material().is_light_source() {
                    continue;
                }

                let obj_name = object.get_name();

                // Delete all old triangle lights
                self.light_defs.delete_light_source_start_with(&*format!(
                    "{}{}",
                    obj_name, TRIANGLE_LIGHT_POSTFIX
                ));

                // Add all new triangle lights
                info!(
                    "the {} object is a light sources with {} triangles",
                    obj_name,
                    mesh.get_total_triangle_count()
                );
                self.obj_defs
                    .define_intersectable_lights(&self.light_defs, object);

                let action = EditAction::LightsEdit | EditAction::LightTypesEdit;
                self.edit_actions.add_actions(action);
            }
        }

        // This is the only place where it is safe to all
        // ext_mesh_cache.define_ext_mesh()
        self.ext_mesh_cache.define_ext_mesh(mesh);
        self.edit_actions.add_action(EditAction::GeometryEdit);
    }

    pub fn define_mesh_args(
        &mut self,
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
        let mut mesh = ExtTriangleMesh::new(
            ply_nb_tris,
            ply_nb_verts,
            p,
            vi,
            n,
            vec![uvs],
            vec![cols],
            vec![alphas],
        );
        mesh.set_name(shape_name);
        let mesh: Box<dyn ExtMesh> = Box::new(mesh);
        self.define_mesh(&mesh);
    }

    pub fn define_mesh_ext(
        &mut self,
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
        let mut mesh = ExtTriangleMesh::new(ply_nb_verts, ply_nb_tris, p, vi, n, uvs, cols, alphas);
        mesh.set_name(shape_name);
        let mesh: Box<dyn ExtMesh> = Box::new(mesh);
        self.define_mesh(&mesh);
    }

    pub fn define_mesh_trans(
        &mut self,
        inst_mesh_name: &str,
        mesh_name: &str,
        trans: &Transform,
    ) -> Result<(), String> {
        let mesh = self.ext_mesh_cache.get_ext_mesh(&mesh_name.to_string());
        if !mesh.is_some() {
            return Err(format!(
                "Unknown mesh is Scene::define_mesh(): {}",
                mesh_name
            ));
        }

        if !mesh.unwrap().is::<ExtTriangleMesh>() {
            return Err(format!(
                "Wrong mesh type in Scene::define_mesh(): {}",
                mesh_name
            ));
        }

        // @TODO: define ext instance triangle mesh
        // let inst_mesh = ExtInstanceTriangleMesh::new(et_mesh, trans);
        // inst_mesh.set_name(inst_mesh_name);
        // self.define_mesh(inst_mesh);

        Ok(())
    }

    pub fn define_mesh_ms(
        &mut self,
        mot_mesh_name: &str,
        mesh_name: &str,
        ms: &MotionSystem,
    ) -> Result<(), String> {
        let mesh = self.ext_mesh_cache.get_ext_mesh(&mesh_name.to_string());
        if !mesh.is_some() {
            return Err(format!(
                "Unknown mesh in Scene:define_ext_mesh(): {}",
                mesh_name
            ));
        }

        if !mesh.unwrap().is::<ExtTriangleMesh>() {
            return Err(format!(
                "Wrong mesh type in Scene::define_mesh(): {}",
                mesh_name
            ));
        }

        // @TODO: Define ExtMotionTriangleMesh
        // let mot_mesh = ExtMotionTriangleMesh::new(et_mesh, ms);
        // mot_mesh.set_name(mot_mesh_name);
        // self.define_mesh(mot_mesh);

        Ok(())
    }

    pub fn set_mesh_vertex_aov(&mut self, mesh_name: &str, index: u32, data: Vec<f32>) {
        self.ext_mesh_cache
            .set_mesh_vertex_aov(mesh_name, index, data);
    }

    pub fn set_mesh_triangle_aov(&mut self, mesh_name: &str, index: u32, data: Vec<f32>) {
        self.ext_mesh_cache
            .set_mesh_triangle_aov(mesh_name, index, data);
    }

    pub fn define_stands(
        &mut self,
        shape_name: &str,
        strands_file: &HairFile,
        tessel_type: TessellationType,
        adaptive_max_depth: u32,
        adaptive_error: f32,
        sold_side_count: u32,
        solid_cap_bottom: bool,
        solid_cap_top: bool,
        use_camera_positions: bool,
    ) {
        // @TODO: define_stands
        // let shape = StrandsShape::new(
        //     self,
        //     strands_file,
        //     tessel_type,
        //     adaptive_max_depth,
        //     adaptive_error,
        //     sold_side_count,
        //     solid_cap_bottom,
        //     solid_cap_top,
        //     use_camera_positions,
        // );
        // let mesh = shape.refine(self);
        // mesh.set_name(shape_name);
        // self.define_mesh(mesh);
        //
        // self.edit_actions.add_action(EditAction::GeometryEdit);
    }

    pub fn is_texture_defined(&self, name: &str) -> bool {
        self.tex_defs.defined(&name.to_string())
    }

    pub fn is_material_defined(&self, name: &str) -> bool {
        self.mat_defs.defined(&name.to_string())
    }

    pub fn is_mesh_defined(&self, name: &str) -> bool {
        self.ext_mesh_cache.is_ext_mesh_defined(&name.to_string())
    }

    pub fn parse(&self, props: &Properties) {
        if self.enable_parse_print {
            info!("===Scene::parse()===");
            info!("{:?}", props);
            info!("====================");
        }

        self.parse_textures(props);
        self.parse_volumes(props);
        self.parse_materials(props);

        // Read camera position and target, note: doing the parsing after volumes
        // because t may reference a volume
        self.parse_camera(props);

        self.parse_shapes(props);
        self.parse_objects(props);
        self.parse_lights(props);
    }

    pub fn delete_object(&mut self, obj_name: &str) {
        if !self.obj_defs.defined(&obj_name.to_string()) {
            return;
        }

        let old = self.obj_defs.get(&obj_name.to_string());
        let was_light_source = old.get_material().is_light_source();

        if was_light_source {
            let action = EditAction::LightsEdit | EditAction::LightTypesEdit;
            self.edit_actions.add_actions(action);

            let mesh = old.get_ext_mesh();
            for i in 0..mesh.get_total_triangle_count() {
                let light_source_name =
                    format!("{}{}{}", old.get_name(), TRIANGLE_LIGHT_POSTFIX, i);
                self.light_defs.delete(light_source_name.as_str());
            }
        }
    }

    pub fn delete_light(&mut self, light_name: &str) {
        if self.light_defs.defined(light_name) {
            self.light_defs.delete(light_name);
            let action = EditAction::LightsEdit | EditAction::LightTypesEdit;
            self.edit_actions.add_actions(action);
        }
    }

    pub fn duplicate_object_trans(&self, src: &str, dst: &str, trans: Transform, dst_obj_id: u32) {
        todo!()
    }

    pub fn duplicate_object_ms(&self, src: &str, dst: &str, ms: MotionSystem, dst_obj_id: u32) {
        todo!()
    }

    pub fn update_object_material(&mut self, obj_name: &str, mat_name: &str) { todo!() }

    pub fn update_object_transformation(&mut self, obj_name: &str, trans_mat: Vec<f32>) { todo!() }

    pub fn remove_unused_image_maps(&mut self) { todo!() }

    pub fn remove_unused_textures(&mut self) { todo!() }

    pub fn remove_unused_materials(&mut self) { todo!() }

    pub fn remove_unused_meshes(&mut self) { todo!() }

    pub fn load(filename: &str) -> Scene { Scene::default() }

    pub fn save(filename: &str, scene: &Scene) { todo!() }

    /* private methods */

    fn parse_camera(&self, props: &Properties) {}

    fn parse_textures(&self, props: &Properties) {}

    fn parse_volumes(&self, props: &Properties) {}

    fn parse_materials(&self, props: &Properties) {}

    fn parse_shapes(&self, props: &Properties) {}

    fn parse_objects(&self, props: &Properties) {}

    fn parse_lights(&self, props: &Properties) {}

    fn get_texture(&self, name: &str) -> Texture { Texture::default() }

    fn create_camera(&self, props: &Properties) -> BaseCamera {
        BaseCamera::new(CameraType::Environment)
    }

    fn create_texture_mapping_2d(&self, prefix: &str, props: &Properties) -> TextureMapping2D {
        TextureMapping2D::default()
    }

    fn create_texture_mapping_3d(&self, prefix: &str, props: &Properties) -> TextureMapping3D {
        TextureMapping3D::default()
    }

    fn create_texture(&self, name: &str, props: &Properties) -> Texture { Texture::default() }

    fn create_volume(&self, id: u32, name: &str, props: &Properties) -> Volume { Volume::default() }

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

    fn create_light_source(&self, name: &str, props: &Properties) -> &dyn LightSource { todo!() }

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
    fn default() -> Self { Self::new(1.0) }
}
