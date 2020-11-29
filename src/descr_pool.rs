use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::DescriptorPool {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "descriptor pool"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_descriptor_pool(*self, None) }
    }
}

pub type DescriptorPool = Handle<vk::DescriptorPool, Deps>;
