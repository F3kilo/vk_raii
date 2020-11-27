use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::DescriptorSetLayout {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "descriptor set layout"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_descriptor_set_layout(*self, None) }
    }
}

pub type CommandPool = Handle<vk::DescriptorSetLayout, Deps>;
