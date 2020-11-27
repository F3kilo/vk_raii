use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::DeviceMemory {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "memory"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.device.free_memory(*self, None) }
    }
}

pub type Memory = Handle<vk::DeviceMemory, Deps>;
