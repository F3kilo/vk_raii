use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;
use crate::ds_layout::DescriptorSetLayout;

pub struct Deps {
    pub device: Device,
    pub ds_layouts: Vec<DescriptorSetLayout>
}

impl RawHandle for vk::PipelineLayout {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "pipeline layout"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_pipeline_layout(*self, None) }
    }
}

pub type PipelineLayout = Handle<vk::PipelineLayout, Deps>;
