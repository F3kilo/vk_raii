use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
}

impl RawHandle for vk::PipelineCache {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "pipeline cache"
    }

    fn destroy(&self, dependencies: &Self::Dependencies) {
        unsafe { dependencies.device.destroy_pipeline_cache(*self, None) }
    }
}

pub type PipelineCache = Handle<vk::PipelineCache, Deps>;
