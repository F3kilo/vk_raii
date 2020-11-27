use crate::instance::Instance;
use crate::{Handle, RawHandle};
use ash::version::DeviceV1_0;
use ash::vk;

pub struct Deps {
    pub instance: Instance,
    pub pdevice: vk::PhysicalDevice,
}

impl RawHandle for ash::Device {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "device"
    }

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe { self.destroy_device(None) }
    }
}

pub type Device = Handle<ash::Device, Deps>;
