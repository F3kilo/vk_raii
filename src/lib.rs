pub mod debug_report;
pub mod instance;
pub mod device;

use std::sync::Arc;

pub trait RawHandle {
    type Dependencies;

    fn name() -> &'static str;

    fn destroy(&self, deps: &Self::Dependencies);
}

pub struct UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    handle: T,
    dependencies: D,
}

impl<T, D> UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    /// # Safety
    /// * `handle` must be valud initialized handle;
    /// * `dependencies` must contain valid and initialized handles;
    pub unsafe fn new(handle: T, dependencies: D) -> Self {
        log::trace!("Unique {} initialized", T::name());
        Self {
            handle,
            dependencies,
        }
    }

    pub fn handle(&self) -> &T {
        &self.handle
    }

    pub fn dependencies(&self) -> &D {
        &self.dependencies
    }
}

impl<T, D> Drop for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn drop(&mut self) {
        log::trace!("Unique {} destroyed", T::name());
        self.handle.destroy(&self.dependencies)
    }
}

pub struct Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    handle: Arc<UniqueHandle<T, D>>,
}

impl<T, D> Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    /// # Safety
    /// * `handle` must be valud initialized handle;
    /// * `dependencies` must contain valid and initialized handles;
    pub unsafe fn new(handle: T, destroy_info: D) -> Self {
        let unique = UniqueHandle::new(handle, destroy_info);
        Self {
            handle: Arc::new(unique),
        }
    }

    pub fn handle(&self) -> &T {
        &self.handle.handle()
    }

    pub fn dependencies(&self) -> &D {
        &self.handle.dependencies()
    }
}

impl<T, D> Clone for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}
