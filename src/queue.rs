use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::vk;

pub struct Deps {
    pub device: Device,
    pub family_index: u32,
    pub queue_index: u32,
}

impl RawHandle for vk::Queue {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "queue"
    }

    fn destroy(&self, _: &Self::Dependencies) {}
}

pub type Queue = Handle<vk::Queue, Deps>;
