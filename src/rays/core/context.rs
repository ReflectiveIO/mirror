use crate::rays::device::{Device, DeviceDescription, HardwareDevice, IntersectionDevice};
use crate::rays::Properties;

use super::dataset::Dataset;

/// A Context is the main tool to access all Rays functionalities. It includes
/// methods to list and create devices, to define the data set to use and to
/// start/stop all the activities.
pub struct Context {
    config: Properties,
    current_dataset: Box<Dataset>,
    device_descriptions: Vec<Box<dyn DeviceDescription>>,
    idevices: Vec<Box<dyn IntersectionDevice>>,
    hdevices: Vec<Box<dyn HardwareDevice>>,
    started: bool,
    verbose: bool,
    use_out_of_core_buffers: bool,
}

impl Context {
    /// Construct a new Rays Context for the optionally defined OpenCL platform.
    ///
    /// The list of configuration properties is:
    /// - opencl.platform.index is the index of the OpenCL platform to use (the
    /// order is the oen returned by cl::Platform::get() function). If the values is -1,
    /// the all the available platforms will be seletect.
    /// - context.verbose is an optional flag to enable/disable the log print of information
    /// related to the available devices.
    /// - accelerator.type
    /// - accelerator.instances.enable
    /// - accelerator.motionblur.enable
    ///
    pub fn new(config: Option<Properties>) -> Self {
        Context {
            config: Properties::default(),
            current_dataset: Box::new(Dataset::new(None)),
            device_descriptions: vec![],
            idevices: vec![],
            hdevices: vec![],
            started: false,
            verbose: false,
            use_out_of_core_buffers: false,
        }
    }

    /* Methods dedicated to device listing and creation */

    /// Return a list of all available device description within the Context.
    pub fn get_available_device_descriptions(&self) -> Vec<Box<dyn DeviceDescription>> {
        vec![]
    }

    /// Return a list of all intersection devices created within the Context.
    pub fn get_intersection_devices(&self) -> Vec<Box<dyn IntersectionDevice>> {
        vec![]
    }

    /// Return a list of all hardware devices create within the Context
    pub fn get_hardware_devices(&self) -> Vec<Box<dyn HardwareDevice>> {
        vec![]
    }

    /// Return a list of all devices created within the Context
    pub fn get_devices(&self) -> Vec<Box<dyn Device>> {
        vec![]
    }

    /// Create an IntersectionDevice within the Context.
    ///
    /// values is a DeviceDescription vector of the devices to create, and return the
    /// vector of all IntersectionDevice created.
    pub fn add_intersection_devices(
        &self,
        values: Vec<Box<dyn DeviceDescription>>,
    ) -> Vec<Box<dyn IntersectionDevice>> {
        vec![]
    }

    /// Create an IntersectionDevice within the Context.
    ///
    /// values is a DeviceDescription vector of the devices to create, and return the
    /// vector of all HardwareDevice created.
    pub fn add_hardware_devices(
        &self,
        values: Vec<Box<dyn DeviceDescription>>,
    ) -> Vec<Box<dyn HardwareDevice>> {
        vec![]
    }

    /* Methods dedicated to DataSet definition */

    pub fn get_current_dataset(&self) -> &Dataset {
        &self.current_dataset
    }

    pub fn set_dataset(&mut self, dataset: Box<Dataset>) {
        self.current_dataset = dataset
    }

    pub fn update_dataset(&mut self) {}

    /* Methods dedicated to out of core rendering */

    pub fn get_use_out_of_core_buffers(&self) -> bool {
        self.use_out_of_core_buffers
    }
    pub fn set_use_out_of_core_buffers(&mut self, v: bool) {
        self.use_out_of_core_buffers = v
    }

    /* Methods dedicated to Context management (i.e. start/stop, etc.) */

    pub fn start(&self) {}
    pub fn interrupt(&self) {}
    pub fn stop(&self) {}
    pub fn running(&self) -> bool {
        self.started
    }

    /* Methods dedicated to message debug handling */

    pub fn set_verbose(&mut self, v: bool) {
        self.verbose = v
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
