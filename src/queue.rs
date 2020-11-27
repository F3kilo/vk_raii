use crate::device::Device;
use crate::{Handle, RawHandle};
use ash::vk;

pub struct Dependencies {
    pub device: Device,
}

impl RawHandle for vk::Queue {
    type Dependencies = Dependencies;

    fn name() -> &'static str {
        "queue"
    }

    fn destroy(&self, _: &Self::Dependencies) {}
}

pub type Queue = Handle<vk::Queue, Dependencies>;
