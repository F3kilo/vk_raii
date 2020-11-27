use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Dependencies {
    pub device: Device,
}

impl RawHandle for vk::Buffer {
    type Dependencies = Dependencies;

    fn name() -> &'static str {
        "device"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.handle().destroy_buffer(*self, None) }
    }
}

pub type Buffer = Handle<vk::Buffer, Dependencies>;
