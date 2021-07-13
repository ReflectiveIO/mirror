bitflags! {
    pub struct BSDFEventType: i8 {
        const NONE = 0;
        const DIFFUSE = 1;
        const GLOSSY = 2;
        const SPECULAR = 4;
        const REFLECT = 8;
        const TRANSMIT = 16;

        const ALL_TYPES = Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
        const ALL_REFLECT = Self::REFLECT.bits | Self::ALL_TYPES.bits;
        const ALL_TRANSMIT = Self::TRANSMIT.bits | Self::ALL_TYPES.bits;
        const ALL = Self::ALL_REFLECT.bits | Self::ALL_TRANSMIT.bits;
    }
}

pub type BSDFEvent = BSDFEventType;

impl Default for BSDFEvent {
    fn default() -> Self {
        BSDFEventType::NONE
    }
}
