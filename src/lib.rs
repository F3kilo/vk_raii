pub mod instance;

use std::fmt;
use std::rc::Rc;

/// Represent generic vulkan handle.
pub trait RawHandle: Sized + Eq + PartialEq {
    type Dependencies: Depend;
    type RawCreateInfo;
    type Error;

    /// Name of handle. "vulkan instance", for example.
    fn name() -> &'static str;

    /// Create handle.
    /// # Safety
    /// Safety requirement specified in Vulkan API specification for every handle.
    unsafe fn create(
        dispatcher: &<Self::Dependencies as Depend>::Dispatcher,
        create_info: &Self::RawCreateInfo,
    ) -> Result<Self, Self::Error>;

    /// Destroy handle.
    /// # Safety
    /// Must be called only after the all reference to this handle destroyed.
    unsafe fn destroy(&self, destroy_info: &<Self::Dependencies as Depend>::DestroyInfo);
}

pub trait Depend {
    type DestroyInfo;
    type Dispatcher;

    fn dispatcher(&self) -> &Self::Dispatcher;
    fn destroy_info(&self) -> &Self::DestroyInfo;
}

/// Generic create info for handle.
pub trait CreateInfoTrait: fmt::Display {
    /// Raw Vilkan API create info.
    type Raw;

    /// Handle Dependencies. Unique Handle guarantee that dependency will drop after dependent handle drop.
    type Dependencies: Depend;

    /// # Safety:
    /// All copies of returned struct become invalid after Drop of `self`.
    fn raw(&self) -> &Self::Raw;

    /// Handle dependencies.
    fn dependencies(&self) -> &Self::Dependencies;

    /// Convert to handle dependencies.
    fn into_dependencies(self) -> Self::Dependencies;

    /// Get dispatcher that usually creates and destroyes handle.
    fn dispatcher(&self) -> &<Self::Dependencies as Depend>::Dispatcher {
        self.dependencies().dispatcher()
    }
}

/// Raii wrapper around raw handle. Destroys wrapped handle on drop.
/// Also hold dependencies while self isn't dropped.
pub struct UniqueHandle<T: RawHandle> {
    handle: T,
    dependencies: T::Dependencies,
}

impl<T: RawHandle> UniqueHandle<T> {
    /// # Safety
    /// Safety requirement specified in Vulkan API specification for every handle.
    pub unsafe fn create<C>(info: C) -> Result<Self, T::Error>
    where
        C: CreateInfoTrait,
        T: RawHandle<RawCreateInfo = C::Raw, Dependencies = C::Dependencies>,
    {
        log::trace!("Creating {} with parameters: {}", T::name(), info);
        T::create(info.dispatcher(), info.raw()).map(|handle| {
            let dependencies = info.into_dependencies();
            Self {
                handle,
                dependencies,
            }
        })
    }

    /// # Safety
    /// All copies of that handle will be invalid after drop of `self`.
    pub unsafe fn handle(&self) -> &T {
        &self.handle
    }

    /// Handle dependencies getter
    pub fn dependencies(&self) -> &T::Dependencies {
        &self.dependencies
    }
}

impl<T: RawHandle> Drop for UniqueHandle<T> {
    fn drop(&mut self) {
        unsafe { self.handle.destroy(self.dependencies.destroy_info()) }
    }
}

impl<T: RawHandle> PartialEq for UniqueHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

pub type Shared<T> = Rc<T>;
