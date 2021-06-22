use std::str::FromStr;

use strum::EnumString;
use strum::ParseError;

use crate::rays::Properties;

/// FilmOutputs
pub struct FilmOutputs {
    types: Vec<FilmOutput>,
    names: Vec<String>,
    props: Vec<Properties>,
    safe_save: bool,
}

impl FilmOutputs {
    pub fn new() -> Self {
        FilmOutputs {
            types: vec![],
            names: vec![],
            props: vec![],
            safe_save: true,
        }
    }

    pub fn reset(&mut self) {
        self.types.clear();
        self.names.clear();
        self.props.clear();
        self.safe_save = true;
    }

    pub fn count(&self) -> usize {
        self.types.len()
    }

    pub fn has_type(&self, t: &FilmOutput) -> bool {
        self.types.contains(t)
    }

    pub fn get_type(&self, index: usize) -> Option<&FilmOutput> {
        self.types.get(index)
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(index)
    }

    pub fn get_properties(&self, index: usize) -> Option<&Properties> {
        self.props.get(index)
    }

    pub fn add(&mut self, t: FilmOutput, name: &str, prop: Option<Properties>) {
        self.types.push(t);
        self.names.push(name.to_string());

        match prop {
            Some(p) => {
                self.props.push(p);
            }
            None => {
                self.props.push(Properties::new());
            }
        }
    }

    pub fn use_safe_save(&self) -> bool {
        self.safe_save
    }

    pub fn set_safe_save(&mut self, v: bool) {
        self.safe_save = v
    }

    pub fn to_properties(cfg: &Properties) {}
}

#[derive(EnumString, PartialEq)]
pub enum FilmOutput {
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
    FilmOutputTypeCount,
}
