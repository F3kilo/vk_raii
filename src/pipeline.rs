use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;
use crate::pipeline_layout::PipelineLayout;

pub struct Deps {
    pub device: Device,
    pub layout: PipelineLayout
}

impl RawHandle for vk::Pipeline {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "pipeline"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_pipeline(*self, None) }
    }
}

pub type Pipeline = Handle<vk::Pipeline, Deps>;
