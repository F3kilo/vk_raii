use crate::{Handle, RawHandle};
use ash::vk;
use ash::version::DeviceV1_0;
use crate::instance::Instance;

pub struct Dependencies {
    pub instance: Instance,
    pub pdevice: vk::PhysicalDevice,
}

impl RawHandle for ash::Device {
    type Dependencies = Dependencies;

    fn name() -> &'static str {
        "device"
    }

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe { self.destroy_device(None) }
    }
}

pub type Device = Handle<ash::Device, Dependencies>;
