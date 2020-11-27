use crate::{Handle, RawHandle};
use ash::version::InstanceV1_0;

pub struct Deps {
    pub entry: ash::Entry,
}

impl RawHandle for ash::Instance {
    type Dependencies = Deps;

    fn name() -> &'static str {
        "instance"
    }

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe { self.destroy_instance(None) }
    }
}

pub type Instance = Handle<ash::Instance, Deps>;
