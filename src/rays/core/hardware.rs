use crate::rays::device::Device;

pub trait HardwareDevice : Device {
    /* Kernels handling for hardware (aka GPU) only applications */

    fn set_additional_compile_opts(&self, opts: Vec<String>);
    fn compile_program(&self);
    fn get_kernel(&self);
    fn get_kernel_workgroup_size(&self) -> u32;
    fn set_kernel_arg(&self);
    fn set_kernel_arg_buffer(&self);

    fn enqueue_kernel(&self);
    fn enqueue_read_buffer(&self);
    fn enqueue_write_buffer(&self);
    fn flush_queue(&self);
    fn finish_queue(&self);

    /* Memory management for hardware (aka GPU) only applications */
    fn get_used_memory(&self) -> usize;
    fn alloc_buffer(&self);
    fn alloc_buffer_ro(&self);
    fn alloc_buffer_rw(&self);
    fn free_buffer(&self);
}
