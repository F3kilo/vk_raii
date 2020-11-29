use crate::descr_pool::DescriptorPool;
use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub device: Device,
    pub pool: DescriptorPool,
}

impl RawHandle for vk::DescriptorSet {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "descriptor set"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.device.free_descriptor_sets(*deps.pool, &[*self]) }
    }
}

pub type DescriptorSet = Handle<vk::DescriptorSet, Deps>;

impl RawHandle for Vec<vk::DescriptorSet> {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "descroptor sets"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe {
            deps.device
                .free_descriptor_sets(*deps.pool, self.as_slice())
        }
    }
}

pub type DescriptorSets = Handle<Vec<vk::DescriptorSet>, Deps>;
