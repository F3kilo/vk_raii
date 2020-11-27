use crate::{CreateInfoTrait, Depend, RawHandle, Shared, UniqueHandle};
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;
use std::fmt;
use std::ops::Deref;

pub struct Instance(Shared<UniqueHandle<RawInstance>>);

impl Deref for Instance {
    type Target = Shared<UniqueHandle<RawInstance>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Entry(ash::Entry);

pub struct CreateInfo<'a> {
    builder: vk::InstanceCreateInfoBuilder<'a>,
    entry: Entry,
}

impl<'a> CreateInfo<'a> {
    pub fn new(entry: ash::Entry) -> Self {
        Self {
            entry: Entry(entry),
            builder: vk::InstanceCreateInfo::builder(),
        }
    }
}

impl Depend for Entry {
    type DestroyInfo = ();
    type Dispatcher = ash::Entry;

    fn dispatcher(&self) -> &Self::Dispatcher {
        &self.0
    }

    fn destroy_info(&self) -> &Self::DestroyInfo {
        &()
    }
}

impl<'a> CreateInfoTrait for CreateInfo<'a> {
    type Raw = vk::InstanceCreateInfo;
    type Dependencies = Entry;

    fn raw(&self) -> &Self::Raw {
        &self.builder
    }

    fn dependencies(&self) -> &Self::Dependencies {
        &self.entry
    }

    fn into_dependencies(self) -> Self::Dependencies {
        self.entry
    }
}

impl<'a> fmt::Display for CreateInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flags: {:?}; Extension count: {}; Layers count: {}",
            self.builder.flags,
            self.builder.enabled_extension_count,
            self.builder.enabled_layer_count
        )
    }
}

/// # Safety
/// https://www.khronos.org/registry/vulkan/specs/1.2/html/chap5.html#initialization-instances
pub unsafe fn create(info: CreateInfo) -> Result<Instance, ash::InstanceError> {
    let unique = UniqueHandle::create(info);
    unique.map(|unique| Instance(Shared::new(unique)))
}

pub struct RawInstance(ash::Instance);

impl RawHandle for RawInstance {
    type Dependencies = Entry;
    type RawCreateInfo = vk::InstanceCreateInfo;
    type Error = ash::InstanceError;

    fn name() -> &'static str {
        "vulkan instance"
    }

    unsafe fn create(
        dispatcher: &ash::Entry,
        create_info: &Self::RawCreateInfo,
    ) -> Result<Self, Self::Error> {
        let inst = dispatcher.create_instance(create_info, None);
        inst.map(Self)
    }

    unsafe fn destroy(&self, _: &()) {
        self.0.destroy_instance(None)
    }
}

impl PartialEq for RawInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0.handle() == other.0.handle()
    }
}

impl Eq for RawInstance {}
