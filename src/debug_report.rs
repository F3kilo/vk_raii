use crate::instance::Instance;
use crate::{Handle, RawHandle};
use ash::extensions::ext;
use ash::vk;
use std::ffi::{c_void, CStr};
use std::fmt;
use std::marker::PhantomData;

pub struct RawDebugMessenger<UserData> {
    handle: vk::DebugUtilsMessengerEXT,
    debug_utils: ext::DebugUtils,
    _user_data: PhantomData<UserData>,
}

impl<UserData> RawDebugMessenger<UserData> {
    pub fn new(handle: vk::DebugUtilsMessengerEXT, debug_report: ext::DebugUtils) -> Self {
        Self {
            handle,
            debug_utils: debug_report,
            _user_data: Default::default(),
        }
    }

    pub fn handle(&self) -> &vk::DebugUtilsMessengerEXT {
        &self.handle
    }
}

pub struct Deps<UserData> {
    pub instance: Instance,
    pub user_data: Option<Box<UserData>>,
}

impl<UserData> RawHandle for RawDebugMessenger<UserData> {
    type Dependencies = Deps<UserData>;

    fn name() -> &'static str {
        "debug report"
    }

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe {
            self.debug_utils
                .destroy_debug_utils_messenger(self.handle, None)
        }
    }
}

pub type DebugMessanger<UserData> = Handle<RawDebugMessenger<UserData>, Deps<UserData>>;

#[derive(Debug, Copy, Clone)]
pub enum MessageLevel {
    Information,
    Warning,
    Perfomance,
    Error,
    Debug,
}

impl From<vk::DebugUtilsMessageSeverityFlagsEXT> for MessageLevel {
    fn from(flags: vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        if flags.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
            return Self::Error;
        }

        if flags.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
            return Self::Warning;
        }

        if flags.contains(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
            return Self::Debug;
        }

        if flags.contains(vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE) {
            return Self::Information;
        }


        Self::Error
    }
}

impl fmt::Display for MessageLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            MessageLevel::Information => write!(f, "INFO "),
            MessageLevel::Warning => write!(f, "WARN "),
            MessageLevel::Perfomance => write!(f, "PERF "),
            MessageLevel::Error => write!(f, "ERROR"),
            MessageLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

impl From<MessageLevel> for log::Level {
    fn from(ml: MessageLevel) -> Self {
        match ml {
            MessageLevel::Information => log::Level::Info,
            MessageLevel::Warning => log::Level::Warn,
            MessageLevel::Perfomance => log::Level::Warn,
            MessageLevel::Error => log::Level::Error,
            MessageLevel::Debug => log::Level::Debug,
        }
    }
}

pub struct Callback(pub Box<dyn Fn(String, MessageLevel) + Send + Sync + 'static>);

impl Callback {
    pub fn cout_reports() -> Self {
        let callback = |msg, level| println!("Vulkan callback report [{}]: {}", level, msg);
        Self(Box::new(callback))
    }

    pub fn log_reports() -> Self {
        let callback = |msg, level: MessageLevel| {
            log::log!(level.into(), "Vulkan callback report [{}]: {}", level, msg)
        };
        Self(Box::new(callback))
    }
}

impl Drop for Callback {
    fn drop(&mut self) {
        log::trace!("Callback of debug report destroyed")
    }
}

/// # Safety
/// * `p_user_data` must be valid pointer to `Callback` struct;
/// * To destroy callback correctly, save it in `Dependencies` of `DebugReport`
pub unsafe extern "system" fn debug_report_with_default_callback(
    msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut c_void,
) -> vk::Bool32 {
    let callback: *mut Callback = p_user_data.cast();
    let callback_ref = callback.as_ref();
    let msg = CStr::from_ptr((*callback_data).p_message);
    let str = msg.to_string_lossy();
    let level = msg_severity.into();
    match callback_ref {
        Some(cb) => cb.0(format!("{}", str), level),
        None => eprintln!("Can't dereference vk debug report callback pointer"),
    }

    vk::FALSE
}
