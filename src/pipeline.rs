use crate::pipeline_layout::PipelineLayout;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub layout: PipelineLayout,
}

impl RawHandle for vk::Pipeline {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "pipeline"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        let device = &deps.layout.dependencies().device;
        unsafe { device.destroy_pipeline(*self, None) }
    }
}

pub type Pipeline = Handle<vk::Pipeline, Deps>;
