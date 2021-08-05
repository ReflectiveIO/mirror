pub use self::blend::BlenderBlendTexture;
pub use self::clouds::BlenderCloudsTexture;
pub use self::distorted::BlenderDistortedNoiseTexture;
pub use self::magic::BlenderMagicTexture;
pub use self::marble::BlenderMarbleTexture;
pub use self::musgrave::BlenderMusgraveTexture;
pub use self::noise::BlenderNoiseTexture;
pub use self::stucci::BlenderStucciTexture;
pub use self::voronoi::BlenderVoronoiTexture;
pub use self::wood::BlenderWoodTexture;

mod blend;
mod clouds;
mod distorted;
mod magic;
mod marble;
mod musgrave;
mod noise;
mod stucci;
mod voronoi;
mod wood;

pub enum BlenderNoiseBase {
    Sin,
    Saw,
    Tri,
}

pub enum BlenderMarbleType {
    Soft,
    Sharp,
    Sharper,
}

pub enum BlenderMusgraveType {
    MultiFractal,
    RidgedMultiFractal,
    HybridMultiFractal,
    Fbm,
    HeteroTerrain,
}

pub enum BlenderStucciType {
    Plastic,
    WallIn,
    WallOut,
}

pub enum BlenderWoodType {
    Bands,
    Rings,
    BandNoise,
    RingNoise,
}

pub enum ProgressionType {
    Lin,
    Quad,
    Ease,
    Diag,
    Sphere,
    Halo,
    Rad,
}

pub enum DistanceMetric {
    ActualDistance,
    DistanceSquared,
    MANHATTAN,
    CheByChev,
    MinkowskiHalf,
    MinkowskiFour,
    Minkowski,
}

pub enum BlenderNoiseBasis {
    BlenderOriginal,
    OriginalPerlin,
    ImprovedPerlin,
    VoronoiF1,
    VoronoiF2,
    VoronoiF3,
    VoronoiF4,
    VoronoiF2F1,
    VoronoiCrackle,
    CellNoise,
}
