use downcast_rs::Downcast;

use crate::rays::accelerator::Accelerator;
use crate::rays::device::Device;

pub trait IntersectionDevice: Device + Downcast {
    /// Returns true if it support HardwareDevice ray tracing
    fn has_hardware_support(&self) -> bool {
        false
    }

    fn get_accelerator(&self) -> dyn Accelerator;

    /* Statistics */

    fn get_total_rays_count(&self) -> f64;
    fn get_total_performance(&self) -> f64;
    fn get_serial_performance(&self) -> i64;
    fn get_data_parallel_performance(&self) -> i64;
    fn reset_performance_stats(&mut self);

    /* Serial interface: to trace a single ray (i.e. on the CPU) */

    fn trace(&self) -> bool;
}
impl_downcast!(IntersectionDevice);
