use crate::command_pool::CommandPool;
use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
    pub pool: CommandPool,
}

impl RawHandle for vk::CommandBuffer {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "command buffer"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.device.free_command_buffers(*deps.pool, &[*self]) }
    }
}

pub type CommandBuffer = Handle<vk::CommandBuffer, Deps>;

impl RawHandle for Vec<vk::CommandBuffer> {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "command buffers"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe {
            deps.device
                .free_command_buffers(*deps.pool, self.as_slice())
        }
    }
}

pub type CommandBuffers = Handle<Vec<vk::CommandBuffer>, Deps>;
