use crate::{Handle, RawHandle};
use ash::extensions::ext;
use ash::vk;
use crate::instance::Instance;

pub type RawDebugReport = (ext::DebugReport, vk::DebugReportCallbackEXT);

pub struct Dependencies {
    pub entry: ash::Entry,
    pub instance: Instance,
}

impl RawHandle for RawDebugReport {
    type Dependencies = Dependencies;

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe { self.0.destroy_debug_report_callback(self.1, None) }
    }
}

pub type DebugReport = Handle<RawDebugReport, Dependencies>;
