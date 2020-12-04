use crate::command_pool::CommandPool;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub pool: CommandPool,
}

impl RawHandle for vk::CommandBuffer {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "command buffer"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        let device = &deps.pool.dependencies().device;
        unsafe { device.free_command_buffers(*deps.pool, &[*self]) }
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
            let device = &deps.pool.dependencies().device;
            device.free_command_buffers(*deps.pool, self.as_slice())
        }
    }
}

pub type CommandBuffers = Handle<Vec<vk::CommandBuffer>, Deps>;
