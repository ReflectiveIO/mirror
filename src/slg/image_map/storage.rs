pub struct ImageMapStorage;

pub enum ChannelSelectionType {
    Default,
    Red,
    Green,
    Blue,
    Alpha,
    Mean,
    WeightedMean,
    Rgb,
    Directx2openglNormalMap,
}

pub enum WrapType {
    Repeat,
    Black,
    White,
    Clamp,
}
