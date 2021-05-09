use crate::instance::Instance;
use crate::{Handle, RawHandle};
use ash::extensions::khr;
use ash::vk;

pub struct Deps {
    pub loader: khr::Surface,
    pub instance: Instance,
}

impl RawHandle for vk::SurfaceKHR {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "surface"
    }

    fn destroy(&self, deps: &Self::Dependencies) {
        unsafe { deps.loader.destroy_surface(*self, None) }
    }
}

pub type Surface = Handle<vk::SurfaceKHR, Deps>;
