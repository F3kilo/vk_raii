use crate::descr_pool::DescriptorPool;
use crate::ds_layout::DescriptorSetLayout;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub pool: DescriptorPool,
    pub ds_layouts: Vec<DescriptorSetLayout>,
    pub can_free: bool,
}

impl RawHandle for vk::DescriptorSet {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "descriptor set"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        if deps.can_free {
            let device = &deps.pool.dependencies().device;
            unsafe { device.free_descriptor_sets(*deps.pool, &[*self]) }
        }
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
            if deps.can_free {
                let device = &deps.pool.dependencies().device;
                device.free_descriptor_sets(*deps.pool, self.as_slice())
            }
        }
    }
}

pub type DescriptorSets = Handle<Vec<vk::DescriptorSet>, Deps>;
