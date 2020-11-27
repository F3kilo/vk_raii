use ash::extensions::ext;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use log::LevelFilter;
use std::error::Error;
use std::fmt;
use std::ops::DerefMut;
use std::os::raw::c_void;
use std::pin::Pin;
use vk_raii::buffer::Buffer;
use vk_raii::debug_report::{Callback, DebugReport, RawDebugReport};
use vk_raii::device::Device;
use vk_raii::instance::Instance;
use vk_raii::memory::Memory;
use vk_raii::queue::Queue;
use vk_raii::{buffer, debug_report, device, instance, memory, queue};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    let init_result = init_vulkan();

    log::info!("Init result: {:?}", init_result)
}

fn init_vulkan() -> Result<String, InitVulkanError> {
    let entry = ash::Entry::new().map_err(|e| init_err("entry", e))?;
    let instance = init_instance(entry)?;
    let _debug_report = init_debug_report(instance.clone())?;
    let device = create_device(instance)?;
    let _buffer = create_buffer(device.clone())?;
    let _memory = allocate_memory(device.clone())?;
    let _queue = get_queue(device);
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
    unsafe {
        let raw = entry
            .create_instance(&ci, None)
            .map_err(|e| init_err("instance", e))?;
        Ok(Instance::new(raw, instance::Dependencies { entry }))
    }
}

fn init_debug_report(instance: Instance) -> Result<DebugReport<Callback>, InitVulkanError> {
    let mut callback = Box::pin(Callback::log_reports());
    let cb_ref = Pin::deref_mut(&mut callback);
    let cb_ptr: *mut Callback = cb_ref;

    let ci = vk::DebugReportCallbackCreateInfoEXT::builder()
        .user_data(cb_ptr as *mut c_void)
        .pfn_callback(Some(debug_report::debug_report_with_default_callback))
        .flags(vk::DebugReportFlagsEXT::all() ^ vk::DebugReportFlagsEXT::INFORMATION);

    let deb_rep = ext::DebugReport::new(&instance.dependencies().entry, instance.handle());
    let raw = unsafe { deb_rep.create_debug_report_callback(&ci, None) }
        .map_err(|e| init_err("debug report", e))?;

    let deps = debug_report::Dependencies {
        instance,
        user_data: Some(callback),
    };

    let raw_debug_report = RawDebugReport::new(raw, deb_rep);

    unsafe { Ok(DebugReport::new(raw_debug_report, deps)) }
}

fn create_device(instance: Instance) -> Result<Device, InitVulkanError> {
    let pdevices =
        unsafe { instance.enumerate_physical_devices() }.map_err(|e| init_err("pdevices", e))?;
    let pdevice = match pdevices.get(0) {
        Some(pd) => Ok(*pd),
        None => Err(InitVulkanError {
            msg: "Can't find vulkan pdevice".into(),
        }),
    }?;

    let features = vk::PhysicalDeviceFeatures::default();
    let prioreties = [1f32];
    let queue = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(0)
        .queue_priorities(&prioreties)
        .build();
    let queues_info = [queue];

    let ci = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queues_info)
        .enabled_features(&features);
    unsafe {
        let raw = instance
            .create_device(pdevice, &ci, None)
            .map_err(|e| init_err("device", e))?;

        Ok(Device::new(raw, device::Dependencies { pdevice, instance }))
    }
}

fn get_queue(device: Device) -> Queue {
    unsafe {
        let raw = device.get_device_queue(0, 0);
        Queue::new(raw, queue::Dependencies { device })
    }
}

fn create_buffer(device: Device) -> Result<Buffer, InitVulkanError> {
    let queue_family_indices = [0u32];
    let ci = vk::BufferCreateInfo::builder()
        .size(128)
        .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .queue_family_indices(&queue_family_indices);

    unsafe {
        let raw = device
            .create_buffer(&ci, None)
            .map_err(|e| init_err("buffer", e))?;
        Ok(Buffer::new(raw, buffer::Dependencies { device }))
    }
}

fn allocate_memory(device: Device) -> Result<Memory, InitVulkanError> {
    let ai = vk::MemoryAllocateInfo::builder()
        .allocation_size(128)
        .memory_type_index(0);

    unsafe {
        let raw = device
            .allocate_memory(&ai, None)
            .map_err(|e| init_err("memory", e))?;
        Ok(Memory::new(raw, memory::Dependencies { device }))
    }
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
