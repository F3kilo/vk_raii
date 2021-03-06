use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::Buffer {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "buffer"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_buffer(*self, None) }
    }
}

pub type Buffer = Handle<vk::Buffer, Deps>;
