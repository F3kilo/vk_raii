pub mod buffer;
pub mod command_buffer;
pub mod command_pool;
pub mod debug_report;
pub mod device;
pub mod instance;
pub mod memory;
pub mod queue;
pub mod ds_layout;
pub mod sampler;
pub mod pipeline_layout;
pub mod pipeline_cache;
pub mod pipeline;
pub mod shader_module;
pub mod render_pass;
pub mod descr_pool;
pub mod descr_set;
pub mod fence;
pub mod swapchain;
pub mod surface;

use std::ops::Deref;
use std::sync::Arc;

pub trait RawHandle {
    type Dependencies;

    fn name() -> &'static str;

    fn destroy(&self, deps: &Self::Dependencies);
}

struct UniqueData<T, D> {
    handle: T,
    dependencies: D,
}

pub struct UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    data: Option<UniqueData<T, D>>,
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
            data: Some(UniqueData {
                handle,
                dependencies,
            }),
        }
    }

    pub fn handle(&self) -> &T {
        if let Some(d) = &self.data {
            return &d.handle;
        }
        unreachable!()
    }

    pub fn dependencies(&self) -> &D {
        if let Some(d) = &self.data {
            return &d.dependencies;
        }
        unreachable!()
    }

    pub fn into_inner(mut self) -> (T, D) {
        log::trace!("Unique {} unwrapped", T::name());
        let d = self
            .data
            .take()
            .expect("Try convert uninitialized unique handle into inner");
        (d.handle, d.dependencies)
    }
}

impl<T, D> Drop for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn drop(&mut self) {
        if let Some(d) = &self.data {
            log::trace!("Unique {} destroyed", T::name());
            d.handle.destroy(&d.dependencies)
        }
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
    pub unsafe fn new(handle: T, dependencies: D) -> Self {
        let unique = UniqueHandle::new(handle, dependencies);
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

    pub fn try_unwrap(self) -> Result<(T, D), Self> {
        match Arc::try_unwrap(self.handle) {
            Ok(unique) => Ok(unique.into_inner()),
            Err(e) => Err(Self { handle: e }),
        }
    }

    /// Count of references to this handle.
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.handle)
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

impl<T, D> Deref for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.handle()
    }
}
