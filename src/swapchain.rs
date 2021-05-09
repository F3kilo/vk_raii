use crate::{Handle, RawHandle};
use ash::extensions::khr;
use ash::vk;
use crate::device::Device;
use crate::surface::Surface;

pub struct Deps {
    pub loader: khr::Swapchain,
    pub device: Device,
    pub surface: Surface,
}

impl RawHandle for vk::SwapchainKHR {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "swapchain"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.loader.destroy_swapchain(*self, None) }
    }
}

pub type Swapchain = Handle<vk::SwapchainKHR, Deps>;
