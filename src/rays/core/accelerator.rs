use crate::rays::device::IntersectionDevice;

#[derive(Hash, Eq, PartialEq)]
pub enum AcceleratorType {
    AUTO,
    BVH,
    MBVH,
    EMBREE,
    OPTIX,
}

pub trait Accelerator {
    fn get_type(&self) -> AcceleratorType;

    fn has_native_support(&self, device: Box<dyn IntersectionDevice>) -> bool;
    fn has_hardware_support(&self, device: Box<dyn IntersectionDevice>) -> bool;

    fn new_hardware_intersection_kernel(&self);

    fn init(&self);
    fn does_support_update(&self) -> bool;
    fn update(&self);

    fn intersect(&self) -> bool;
}
