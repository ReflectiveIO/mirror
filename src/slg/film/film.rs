#[derive(Debug)]
pub enum FilmChannel {
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