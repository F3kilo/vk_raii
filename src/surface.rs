use crate::instance::Instance;
use crate::{Handle, RawHandle};
use ash::extensions::khr;
use ash::version::EntryV1_0;
use ash::vk;
use std::ffi::CStr;

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

pub fn enumerate_surface_extensions(
    entry: &ash::Entry,
) -> ash::prelude::VkResult<Vec<&'static CStr>> {
    let supported_extensions = entry.enumerate_instance_extension_properties()?;
    let mut required_extensions = Vec::new();

    #[cfg(target_os = "windows")]
    required_extensions.extend_from_slice(&[khr::Surface::name(), khr::Win32Surface::name()]);

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    unsafe {
        let linux_surface_extensions = [
            khr::Surface::name(),
            khr::XcbSurface::name(),
            khr::XlibSurface::name(),
            khr::WaylandSurface::name(),
        ];

        let supported_linux_extensions = linux_surface_extensions.iter().filter(|lse| {
            supported_extensions
                .iter()
                .any(|se| CStr::from_ptr(se.extension_name.as_ptr()) == **lse)
        });

        required_extensions.extend(supported_linux_extensions);
    }

    #[cfg(any(target_os = "android"))]
    required_extensions.push(khr::AndroidSurface::name());

    #[cfg(any(target_os = "macos"))]
    required_extensions.push(ash::ext::MetalSurface::name());

    #[cfg(any(target_os = "ios"))]
    required_extensions.push(ash::ext::MetalSurface::name());

    unsafe {
        for re in &required_extensions {
            if !supported_extensions
                .iter()
                .any(|se| CStr::from_ptr(se.extension_name.as_ptr()) == *re)
            {
                return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
            }
        }
    }

    Ok(required_extensions)
}
