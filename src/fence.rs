use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::Fence {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "fence"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_fence(*self, None) }
    }
}

pub type Fence = Handle<vk::Fence, Deps>;
