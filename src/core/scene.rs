use crate::core::CameraTrait;
use crate::rays::properties::Properties;
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
    // This list must be aligned with slg::StrendsShape::TessellationType
    Ribbon,
    RibbonAdaptive,
    Solid,
    SolidAdaptive,
}

pub trait SceneTrait {
    /// Returns the bounding box of the complete scene (as minimum and maximum point). It is
    /// available only during the rendering (i.e. after a Session::start()).
    fn get_bounding_box(&self, min: [f32], max: [f32]);

    /// Returns the Camera of the scene.
    fn camera(&self) -> Box<dyn CameraTrait>;

    /// Defines an image map (to be later used in textures, infinite lights, etc.).
    /// The memory allocated for pixels array is NOT freed by the Scene class nor
    /// is used after the execution of this method. The types supported are "unsigned char",
    /// "unsigned short" (as a place holder for half type) and "float".
    fn define_image_map<T>(
        &self,
        img_map_name: &str,
        pixels: T,
        gamma: f32,
        channels: u32,
        width: u32,
        height: u32,
        selection_type: ChannelSelectionType,
        wrap_type: WrapType,
    ) {
        panic!("Called Scene::define_image_map() with wrong type");
    }

    /// Check if an image map with the given name has been defined.
    fn is_image_map_defined(&self, img_map_name: &str) -> bool {
        true
    }

    /// Sets if te Scene class destructor will
    /// delete the arrays pointed to by the defined meshes.
    fn set_delete_mesh_data(&mut self, v: bool);

    /// Sets the applied transformation matrix for a normal
    /// mesh (i.e. not instanced or motion blurred).
    fn set_mesh_applied_transformation(mesh_name: &str, applied_trans_matrix: f32);
}

impl Scene {
    /// Create a new empty Scene. the scale used for storing any kind of image in memory.
    pub fn new(scale: f32) -> Scene {
        Scene {
            scene: slg::Scene::new(),
        }
    }

    /// Create a new Scene as defined by props.
    pub fn create(props: &Properties, scale: f32) -> Scene {
        Scene::new(0.0)
    }

    /// Create a new Scene as defined in filename file.
    ///
    /// the file can be a text SDL file or a serialized binary file. The extension for the
    /// binary format must be ".bsc". the scale used for storing an kind of image in memory.
    /// this parameter has no effect when loading binary serialized file.
    pub fn load(filename: &str, scale: f32) -> Scene {
        Scene::new(0.0)
    }
}
