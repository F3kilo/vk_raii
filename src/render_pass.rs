use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::RenderPass {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "render pass"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_render_pass(*self, None) }
    }
}

pub type RenderPass = Handle<vk::RenderPass, Deps>;
