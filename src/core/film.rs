use crate::rays::properties::Properties;

/// Film stores all the outputs of a rendering. It can be obtained from RenderSession
/// or as stand alone object loaded from a file.
#[derive(Debug, Default)]
pub struct Film {
    pub width: u32,
    pub height: u32,
}

impl Film {
    pub fn new() -> Film {
        Film {
            width: 800,
            height: 600,
        }
    }
    /// Loads a stand alone Film (i.e. not connected to a rendering session) from a file.
    pub fn load(filename: &str) -> Film {
        Film::new()
    }

    /// Create a stand alone Film (i.e. not conncted to a rendering session)
    /// from the properties.
    ///
    /// * props defining the film.
    /// * has_pixel_normalized_channel if the film must have ChannelRadiancePerPixelNormalized.
    /// * has_screen_normalized_channel if the film must have ChannelRadiancePerScreenNormalized.
    ///
    pub fn create(
        props: &Properties,
        has_pixel_normalized_channel: bool,
        has_screen_normalized_channel: bool,
    ) -> Film {
        Film::new()
    }

    /// Returns a list of statistics related to the film. Mostly useful for stand alone films.
    pub fn stats(&self) -> Properties {
        Properties::default()
    }

    /// Returns the Film average luminance. It can be use to estimate a good value for variance clamping.
    pub fn y(&self) -> f64 {
        0 as f64
    }

    /// Clear the film.
    pub fn clear(&self) {}

    /// Add a film.
    pub fn add_film(&self, film: &Film) {}

    pub fn add_film_with_args(
        &self,
        film: &Film,
        src_offset_x: u32,
        src_offset_y: u32,
        src_width: u32,
        src_height: u32,
        dist_offset_x: u32,
        dist_offset_y: u32,
    ) {
    }

    /// Saves all Film output channels defined in the current
    /// RenderSession. This method can not be used with a standalone film.
    pub fn save_outputs(&self) {
        println!("- @TODO: saves all film output channels")
    }

    /// Saves the specified Film output channel.
    pub fn save_output(&self, filename: &str, output_type: FilmOutput, props: &Properties) {}

    /// Serializes a Film in a file.
    pub fn save_film(&self, filename: &str) {}

    /// Returns the total sample count.
    pub fn total_sample_count(&self) -> f64 {
        0 as f64
    }

    /// Return the size (in f64 or u64) of a File output channel.
    pub fn output_size(&self) -> isize {
        0
    }

    /// Returns if a film channel output is available.
    pub fn has_output(&self, output_type: FilmOutput) -> bool {
        false
    }

    /// Returns the number of output channels of the passed type.
    pub fn output_count(&self, output_type: FilmOutput) -> u32 {
        0 as u32
    }
}

// Types of Film channel available.
#[derive(Debug)]
pub enum FilmChannel {
    // This list must be aligned with slg::film::FilmChannel
    RadiancePerPixelNormalized,
    RadiancePerScreenNormalized,
    Alpha,
    ImagePipeline,
    Depth,
    Position,
    GeometryNormal,
    ShadingNormal,
    MaterialId,
    DirectDiffuse,
    DirectDiffuseReflect,
    DirectDiffuseTransmit,
    DirectGlossy,
    DirectGlossyReflect,
    DirectGlossyTransmit,
    Emission,
    IndirectDiffuse,
    IndirectDiffuseReflect,
    IndirectDiffuseTransmit,
    IndirectGlossy,
    IndirectGlossyReflect,
    IndirectGlossyTransmit,
    IndirectSpecular,
    IndirectSpecularReflect,
    IndirectSpecularTransmit,
    MaterialIdMask,
    DirectShadowMask,
    IndirectShadowMask,
    Uv,
    RayCount,
    ByMaterialId,
    Irradiance,
    ObjectId,
    ObjectIdMask,
    ByObjectId,
    SampleCount,
    Convergence,
    MaterialIdColor,
    Albedo,
    AvgShadingNormal,
    Noise,
    UserImportance,
}

// Types of Film output available.
#[derive(Debug)]
pub enum FilmOutput {
    // This list must be aligned with slg::film::FilmOutput
    Rgb,
    Rgba,
    RgbImagePipeline,
    RgbaImagePipeline,
    Alpha,
    Depth,
    Position,
    GeometryNormal,
    ShadingNormal,
    MaterialId,
    DirectDiffuse,
    DirectDiffuseReflect,
    DirectDiffuseTransmit,
    DirectGlossy,
    DirectGlossyReflect,
    DirectGlossyTransmit,
    Emission,
    IndirectDiffuse,
    IndirectDiffuseReflect,
    IndirectDiffuseTransmit,
    IndirectGlossy,
    IndirectGlossyReflect,
    IndirectGlossyTransmit,
    IndirectSpecular,
    IndirectSpecularReflect,
    IndirectSpecularTransmit,
    MaterialIdMask,
    DirectShadowMask,
    IndirectShadowMask,
    RadianceGroup,
    Uv,
    RayCount,
    ByMaterialId,
    Irradiance,
    ObjectId,
    ObjectIdMask,
    ByObjectId,
    SampleCount,
    Convergence,
    SerializedFilm,
    MaterialIdColor,
    Albedo,
    AvgShadingNormal,
    Noise,
    UserImportance,
    Caustic,
}
