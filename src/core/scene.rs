use super::CameraTrait;
use crate::rays::utils::HairFile;
use crate::rays::Properties;
use crate::slg;

/// Scene stores textures, materials and objects definitions.
pub struct Scene {
    pub scene: slg::Scene,
}

/// Types of image map channel selection.
pub enum ChannelSelectionType {
    // This list must be aligned with slg::ImageMapStorage::ChannelSelectionType
    Default,
    Red,
    Green,
    Blue,
    Alpha,
    Mean,
    WeightedMean,
    Rgb,
}

/// Types of image map wrap mode.
pub enum WrapType {
    // This list must be aligned with slg::ImageMapStorage::WrapType
    Repeat,
    Black,
    White,
    Clamp,
}

/// Types of strands tessellation.
pub enum StrandsTessellationType {
    // This list must be aligned with slg::shape::TessellationType
    Ribbon,
    RibbonAdaptive,
    Solid,
    SolidAdaptive,
}

pub trait SceneTrait {
    /// Returns the bounding box of the complete scene (as minimum and maximum
    /// point). It is available only during the rendering (i.e. after a
    /// Session::start()).
    fn get_bounding_box(&self, min: Vec<f32>, max: Vec<f32>);

    /// Returns the Camera of the scene.
    fn camera(&self) -> Box<dyn CameraTrait>;

    /// Defines an image map (to be later used in textures, infinite lights,
    /// etc.). The memory allocated for pixels array is NOT freed by the
    /// Scene class nor is used after the execution of this method. The
    /// types supported are "unsigned char", "unsigned short" (as a place
    /// holder for half type) and "float".
    fn define_image_map<T>(
        &self,
        img_map_name: &str, // name of the defined image map.
        pixels: T,          // pointer to an array of image map pixels.
        gamma: f32,         // the gamma correction value of the image.
        channels: u32,      // the number of data used for each pixel (1 or 3).
        width: u32,         // the width of the image map.
        height: u32,        // the height of the image map.
        selection_type: ChannelSelectionType,
        wrap_type: WrapType,
    ) {
        panic!("Called Scene::define_image_map() with wrong type");
    }

    /// Check if an image map with the given name has been defined.
    fn is_image_map_defined(&self, img_map_name: &str) -> bool;

    /// Sets if te Scene class destructor will
    /// delete the arrays pointed to by the defined meshes.
    fn set_delete_mesh_data(&mut self, v: bool);

    /// Sets the applied transformation matrix for a normal
    /// mesh (i.e. not instanced or motion blurred).
    fn set_mesh_applied_transformation(mesh_name: &str, applied_trans_matrix: f32);

    /// Defines a mesh (to be later used in one or more scene objects). The
    /// memory allocated or the ExtTriangleMesh is always freed by the Scene
    /// class, however freeing of memory for the vertices, triangles
    /// indices, etc. depends on the setting of set_delete_mesh_data().
    ///
    /// NOTE: vertices and triangles buffers MUST be allocated with
    /// Scene::alloc_vertices_buffer() and Scene::alloc_triangles_buffer().
    fn define_mesh(
        &self,
        mesh_name: &str,   // The name of the defined mesh.
        ply_nb_verts: i64, // The number of mesh vertices.
        ply_nb_tris: i64,  // The number of mesh triangles.
        p: Vec<f32>,       // Pointer to an array of vertices.
        vi: Vec<u32>,      // Pointer to an array of triangles.
        n: Vec<f32>,       // Pointer to an array of normals. It can be NULL.
        uvs: Vec<f32>,     // Pointer to an array of UV coordinates. It can be NULL.
        cols: Vec<f32>,    // Pointer to an array of vertices colors. It can be NULL.
        alphas: Vec<f32>,  // Pointer to an array of vertices alphas. It can be NULL.
    );

    /// this is a special version of Scene::define_mesh() used to
    /// define meshes with multiple set of UVs, Colors and/or Alphas.
    ///
    /// NOTE: the array of UVs, Colors and alphas pointers can be freed after
    /// the call however freeing of memory for the vertices, triangle
    /// indices, etc. depends on the setting of set_delete_mesh_data().
    fn define_mesh_ext(
        &self,
        mesh_name: &str,       // the name of the defined mesh.
        ply_nb_verts: i64,     // the number of mesh vertices.
        ply_nb_tris: i64,      // the number of mesh triangles.
        p: Vec<f32>,           // pointer to an array of vertices.
        vi: Vec<u32>,          // pointer to array of triangles.
        n: Vec<f32>,           // pointer to array of normals, it can be NULL.
        uvs: Vec<Vec<f32>>,    // pointer to an array of vertices coordinates, it can be NULL.
        cols: Vec<Vec<f32>>,   // pointer to an array of vertices colors. it can be NULL.
        alphas: Vec<Vec<f32>>, // pointer to an array of vertices alphas, it can be NULL.
    );

    /// Set a mesh geometry vertex AOV
    fn set_mesh_vertex_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>);

    /// Set a mesh geometry triangle AOV
    fn set_mesh_triangle_aov(&self, mesh_name: &str, index: u32, data: Vec<f32>);

    /// Save a previously defined mesh to filesystem in PLY or BPY format.
    fn save_mesh(&self, mesh_name: &str, filename: &str);

    /// Defines a mesh (to be later used in one or more scene objects) starting
    /// from the strands/hairs definition included in stands file.
    fn define_stands(
        &self,
        shape_name: &str,                     // the name of the defined shape
        strands_file: HairFile,               // includes all information about the strands.
        tessel_type: StrandsTessellationType, /* the tessellation used to transform the strands
                                               * in a triangle mesh */

        adaptive_max_depth: u32, // maximum number of subdivisions for adaptive tessellation.
        adaptive_error: f32,     // the error threshold for adaptive tessellation

        sold_side_count: u32,   // the number of sides for solid tessellation
        solid_cap_bottom: bool, // a flag to set if strands has to have a bottom cap
        solid_cap_top: bool,    // a flag to set if stands has to have a top cap

        use_camera_positions: bool, /* a flag to set if ribbon tessellation has to be faced
                                     * toward the camera */
    );

    /// Check if a mesh with the give name has bee defined
    fn is_mesh_defined(&self, name: &str);

    /// check if a texture has been defined
    fn is_texture_defined(&self, name: &str);

    /// Check if a material with the given name has been defined
    fn is_material_defined(&self, name: &str);

    /// Returns the number of light sources in the Scene
    fn get_light_count(&self) -> u32 { 0 }

    /// Returns the number of objects in the Scene
    fn get_object_count(&self) -> u32 { 0 }

    /// Edits or creates camera, textures, materials and/or objects base on the
    /// Properties defined. If the scene is in use by a Session, it must be
    /// called between Session::begin_scene_edit() and Session::
    /// end_scene_edit().
    fn parse(&self, props: &Properties);

    /// Duplicate an object in an instance using the passed transformation
    fn duplicate_object(&self, src: &str, dst: &str, trans_mat: Vec<f32>, object_id: u32);

    /// Duplicate an object multiple times in instances using passed
    /// transformation. Mostly useful for fast creating many copies of the
    /// same object (for instance for particles)
    fn duplicate_objects(
        &self,
        src: &str,
        dst: &str,
        count: u32,
        trans_mat: Vec<f32>,
        ids: Vec<u32>,
    );

    /// Duplicate an object in a motion blur instance using the passed
    /// transformation
    fn duplicate_motion_blur_object(
        &self,
        src: &str,           // the name of the object to duplicate
        dst: &str,           // the name of the object to create
        steps: u32,          // the number of motion blur steps
        times: Vec<f32>,     // array of times to use
        trans_mat: Vec<f32>, // array of the transformation 4x4 matrix to use
        object_id: u32,      // object Id that will be assigned to the duplicate.
    );

    /// Duplicate an object multiple times in a motion blur instance using
    /// the passed transformations. Mostly useful for fast creating many copies
    /// of the same object (for instance for particles).
    fn duplicate_motion_blur_objects(
        &self,
        src: &str,            // the name of the object to duplicate
        dst_prefix: &str,     // the prefix of the names of the object to create
        count: u32,           // the number of the object to create
        steps: u32,           // the number of motion blur steps
        times: Vec<f32>,      // array of times to use
        trans_mat: Vec<f32>,  // array of the transformation 4x4 matrix to use
        object_ids: Vec<u32>, // object Id that will be assigned to the duplicate.
    );

    /// Apply a transformation to an object
    fn update_object_transformation(&mut self, obj_name: &str, trans_mat: Vec<f32>);

    /// Apply a new material to an object
    fn update_object_material(&mut self, obj_name: &str, mat_name: &str);

    /// Deletes an object from the scene
    fn delete_object(&mut self, obj_name: &str);

    /// Deletes a light from the scene
    fn delete_light(&mut self, light_name: &str);

    /// Removes all unused image maps
    fn remove_unused_image_maps(&mut self);

    /// Removes all unused textures
    fn remove_unused_textures(&mut self);

    /// Removes all unused materials
    fn remove_unused_materials(&mut self);

    /// Removes all unused meshes
    fn remove_unused_meshes(&mut self);

    /// Returns all the Properties required to define this Scene
    fn to_properties(&self) -> &Properties;

    /// Serializes a Scene in a file
    fn save(&self, filename: &str);

    /// This must be used to allocate Mesh vertices buffer
    fn alloc_vertices_buffer(count: usize) -> Vec<f32>;

    /// This must be used to allocate Mesh triangles buffer
    fn alloc_triangles_buffer(count: usize) -> Vec<u32>;
}

impl Scene {
    /// Create a new empty Scene. the scale used for storing any kind of image
    /// in memory.
    pub fn new(scale: f32) -> Scene {
        Scene {
            scene: slg::Scene::default(),
        }
    }

    /// Create a new Scene as defined by props.
    pub fn create(props: &Properties, scale: f32) -> Scene { Scene::new(0.0) }

    /// Create a new Scene as defined in filename file.
    ///
    /// the file can be a text SDL file or a serialized binary file. The
    /// extension for the binary format must be ".bsc". the scale used for
    /// storing an kind of image in memory. this parameter has no effect
    /// when loading binary serialized file.
    pub fn load(filename: &str, scale: f32) -> Scene { Scene::new(0.0) }

    pub fn from(scene: slg::Scene) -> Self { Self { scene } }
}

impl Default for Scene {
    fn default() -> Self { Self::new(1.0) }
}
