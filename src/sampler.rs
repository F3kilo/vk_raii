use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::Sampler {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "sampler"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_sampler(*self, None) }
    }
}

pub type Sampler = Handle<vk::Sampler, Deps>;
