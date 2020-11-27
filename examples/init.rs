use log::LevelFilter;
use std::error::Error;
use std::fmt;
use vk_raii::instance;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    let init_result = init();
    init_result.expect("Vulkan can't be initialized")
}

fn init() -> Result<(), InitError> {
    let entry = ash::Entry::new()?;
    let create_info = instance::CreateInfo::new(entry);
    let _instance = unsafe { instance::create(create_info) };
    Ok(())
}

#[derive(Debug)]
pub struct InitError {
    msg: String,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't init vulkan: {}", self.msg)
    }
}

impl<T: Error> From<T> for InitError {
    fn from(e: T) -> Self {
        Self {
            msg: format!("Vulkan init error: {}", e),
        }
    }
}
