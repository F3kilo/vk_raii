use crate::{Handle, RawHandle};
use ash::version::InstanceV1_0;

pub struct Dependencies {
    pub entry: ash::Entry,
}

impl RawHandle for ash::Instance {
    type Dependencies = Dependencies;

    fn destroy(&self, _: &Self::Dependencies) {
        unsafe { self.destroy_instance(None) }
    }
}

pub type Instance = Handle<ash::Instance, Dependencies>;
