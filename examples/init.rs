use ash::extensions::ext;
use ash::version::EntryV1_0;
use ash::vk;
use log::LevelFilter;
use std::error::Error;
use std::fmt;
use vk_raii::instance;
use vk_raii::instance::Instance;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    let init_result = init_vulkan();
    log::info!("Init result: {:?}", init_result)
}

fn init_vulkan() -> Result<String, InitVulkanError> {
    let entry = ash::Entry::new().map_err(|e| init_err("entry", e))?;
    let _instance = init_instance(entry)?;

    Ok("Success".into())
}

fn init_instance(entry: ash::Entry) -> Result<Instance, InitVulkanError> {
    let app_inf = vk::ApplicationInfo::builder().api_version(vk::make_version(1, 0, 0));

    let exts = [ext::DebugReport::name().as_ptr()];
    let layers = ["VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8];

    let ci = vk::InstanceCreateInfo::builder()
        .application_info(&app_inf)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&exts);

    let raw = unsafe { entry.create_instance(&ci, None) }.map_err(|e| init_err("instance", e))?;
    unsafe { Ok(Instance::new(raw, instance::Dependencies { entry })) }
}

#[derive(Debug)]
struct InitVulkanError {
    msg: String,
}

impl Error for InitVulkanError {}

impl fmt::Display for InitVulkanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vulkan initialization failed: {}", self.msg)
    }
}

fn init_err(what: &str, e: impl Error) -> InitVulkanError {
    InitVulkanError {
        msg: format!("Can't init {}: {}", what, e),
    }
}
