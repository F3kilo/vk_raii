use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::CommandPool {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "command pool"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_command_pool(*self, None) }
    }
}

pub type CommandPool = Handle<vk::CommandPool, Deps>;
