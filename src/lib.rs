pub mod instance;

use std::sync::Arc;

pub trait RawHandle {
    type Dependencies;

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
        self.handle.destroy(&self.dependencies)
    }
}

#[derive(Clone)]
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
