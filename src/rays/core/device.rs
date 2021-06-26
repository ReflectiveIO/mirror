//------------------------------------------------------------------------------
// Device
//------------------------------------------------------------------------------

/*
 * The inheritance scheme used here:
 *
 *  Device => | =>                  IntersectionDevice                   => | => NativeIntersectionDevice
 *
 *            | =>   HardwareDevice   => | =>        OpenCLDevice        => |
 *  Device => |                                                             | => OpenCLIntersectionDevice
 *            | =>   HardwareDevice   => |                                  |
 *            |                          | => HardwareIntersectionDevice => |
 *            | => IntersectionDevice => |
 *
 *            | =>   HardwareDevice   => | =>         CudaDevice         => |
 *  Device => |					                                            | => CudaIntersectionDevice
 *            | =>   HardwareDevice   => |                                  |
 *            |                          | => HardwareIntersectionDevice => |
 *            | => IntersectionDevice => |
 */
use config::Value;
use downcast_rs::Downcast;
use serde::{Deserialize, Serialize};

use super::context::Context;
use std::convert::TryFrom;

pub trait Device: Downcast {
    fn name(&self) -> &String;
    fn context(&self) -> &Context;
    fn description(&self) -> &dyn DeviceDescription;
    fn index(&self) -> usize;

    fn running(&self) -> bool;
    fn start(&self);
    fn interrupt(&self);
    fn stop(&self);

    fn push_thread_current_device(&mut self);
    fn pop_thread_current_device(&mut self);
}
impl_downcast!(Device);

pub trait DeviceDescription {
    fn name(&self) -> &String;
    fn get_type(&self) -> DeviceType;
    fn get_compute_units(&self) -> i32 {
        1
    }
    fn get_native_vector_width_float(&self) -> u32 {
        4
    }
    fn get_max_memory(&self) -> usize {
        usize::MAX
    }
    fn get_max_memory_alloc_size(&self) -> usize {
        usize::MAX
    }
    fn has_out_of_core_memory_support(&self) -> bool {
        false
    }

    fn get_force_workgroup_size(&self) -> u32;
    fn set_force_workgroup_size(&mut self, size: u32);
}

bitflags! {
    pub struct DeviceType: u32 {
        const NATIVE = 1 << 0;
        const OPENCL_DEFAULT = 1 << 1;
        const OPENCL_CPU = 1 << 2;
        const OPENCL_GPU = 1 << 3;
        const OPENCL_UNKNOWN = 1 << 4;
        const CUDA_GPU = 1 << 5;
        const OPENCL_ALL = Self::OPENCL_DEFAULT.bits | Self::OPENCL_CPU.bits
                        | Self::OPENCL_GPU.bits | Self::OPENCL_UNKNOWN.bits;
        const CUDA_ALL = Self::CUDA_GPU.bits;
        const ALL = Self::NATIVE.bits | Self::OPENCL_ALL.bits;
        const ALL_HARDWARE = Self::OPENCL_ALL.bits | Self::CUDA_ALL.bits;
        const ALL_INTERSECTION = Self::NATIVE.bits | Self::OPENCL_ALL.bits;
    }
}

impl Into<Value> for DeviceType {
    fn into(self) -> Value {
        Value::try_from(self).unwrap()
    }
}
