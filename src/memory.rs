use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Dependencies {
    pub device: Device,
}

impl RawHandle for vk::DeviceMemory {
    type Dependencies = Dependencies;

    fn name() -> &'static str {
        "memory"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.device.handle().free_memory(*self, None) }
    }
}

pub type Memory = Handle<vk::DeviceMemory, Dependencies>;
