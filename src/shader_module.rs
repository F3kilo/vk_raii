use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::ShaderModule {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "shader module"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_shader_module(*self, None) }
    }
}

pub type ShaderModule = Handle<vk::ShaderModule, Deps>;
